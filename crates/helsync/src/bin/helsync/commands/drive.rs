use crate::database::{Database, Drive, App};
use super::errors::*;
use helsync::oauth2;

use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
pub enum DriveCommand {

    /// Describe a drive.
    #[clap(alias = "get")]
    Describe(DescribeOpt),

    /// List available drives.
    #[clap(alias = "ls")]
    List(ListOpt),

    /// Create a new drive.
    Create(CreateOpt),

    /// Remove a drive.
    #[clap(alias = "rm")]
    Remove(RemoveOpt),

    /// Sign in to a drive.
    Connect(ConnectOpt),

    /// Forget credentials for a drive.
    Disconnect(DisconnectOpt),
}

/// Group of commands for managing drives.
#[derive(Parser, Debug)]
pub struct DriveOpt {
    #[clap(subcommand)]
    pub command: DriveCommand,
}

impl DriveOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        match &self.command {
            DriveCommand::Describe(opt) => opt.run(db).await,
            DriveCommand::List(opt) => opt.run(db).await,
            DriveCommand::Create(opt) => opt.run(db).await,
            DriveCommand::Remove(opt) => opt.run(db).await,
            DriveCommand::Connect(opt) => opt.run(db).await,
            DriveCommand::Disconnect(opt) => opt.run(db).await,
        }
    }
}

/// Describe a drive.
#[derive(Parser, Debug)]
pub struct DescribeOpt {
    /// The drive's name.
    pub name: String,
}

impl DescribeOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let row: Drive = sqlx::query_as("SELECT * FROM Drive WHERE name = ?")
            .bind(&self.name)
            .fetch_one(&mut *conn).await
            .map_err(handle_not_found_err)?;

        println!("Drive Description:\n{}", row);
        if let Some(token) = row.token {
            println!("\nOAuth2 Credentials:");
            println!(" - Access Token: {}", token.access_token);
            println!(" - Refresh Token: {}", token.refresh_token);
            println!(" - Created At: {}", token.created_at);
            println!(" - Expires In: {}", token.expires_in);
            println!(" - Is Expired: {}", token.is_expired());
        }

        Ok(())
    }
}

/// List available drives.
#[derive(Parser, Debug)]
pub struct ListOpt {}

impl ListOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let rows: Vec<Drive> = sqlx::query_as("SELECT * FROM Drive")
            .fetch_all(&mut *conn).await?;

        if rows.len() == 0 {
            return Err(anyhow::anyhow!("no drives found"));
        }

        println!("Available Drives:");
        rows.iter().for_each(|drive| println!(" - {drive}"));
        Ok(())
    }
}

/// Create a new drive.
#[derive(Parser, Debug)]
pub struct CreateOpt {
    /// The drive's name.
    #[arg(long, short)]
    pub name: String,

    /// Cloud storage path.
    #[arg(long, short)]
    pub path: String,

    /// App Registration.
    #[arg(long, short)]
    pub app: String,
}

impl CreateOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        super::utils::is_valid_name(&self.name)?;
        let mut conn = db.acquire().await?;
        sqlx::query("INSERT INTO Drive (name, path, app) VALUES (?, ?, ?)")
            .bind(&self.name)
            .bind(&self.path)
            .bind(&self.app)
            .execute(&mut *conn).await
            .map_err(handle_unique_violation_err)?;

        println!("drive \"{}\" successfully created", self.name);
        Ok(())
    }
}

/// Remove a drive.
#[derive(Parser, Debug)]
pub struct RemoveOpt {
    /// The drive's name.
    pub name: String,
}

impl RemoveOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let row = sqlx::query("DELETE FROM Drive WHERE name=?")
            .bind(&self.name)
            .execute(&mut *conn)
            .await?;

        if row.rows_affected() == 0 {
            return Err(anyhow::anyhow!("drive \"{}\" not found", self.name));
        }

        println!("drive \"{}\" successfully deleted", self.name);
        Ok(())
    }
}


/// Sign in to a drive.
#[derive(Parser, Debug)]
pub struct ConnectOpt {
    /// The drive's name.
    pub name: String,
}

impl ConnectOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let drive: Drive = sqlx::query_as("SELECT * FROM Drive WHERE name=?")
            .bind(&self.name)
            .fetch_one(&mut *conn).await
            .map_err(handle_not_found_err)?;

        let app: App = sqlx::query_as("SELECT App.* FROM App JOIN Drive ON App.name=Drive.app WHERE Drive.name=?")
            .bind(&self.name)
            .fetch_one(&mut *conn).await
            .map_err(handle_not_found_err)?;

        let app_config: oauth2::Config = app.clone().into();
        if let Some(token) = drive.token {
            let mut token = token.clone();
            let res = token.refresh_if_expired(&app_config).await;
            if let Ok(_) = res {
                sqlx::query("UPDATE Drive SET access_token=?, refresh_token=?, created_at=?, expires_in=? WHERE name=?")
                    .bind(&token.access_token)
                    .bind(&token.refresh_token)
                    .bind(token.created_at)
                    .bind(token.expires_in)
                    .bind(&self.name)
                    .execute(&mut *conn).await?;

                println!("drive \"{}\" successfully authenticated", self.name);
                return Ok(());
            }
        }

        let token = oauth2::Grant::from_server(app.port, &app_config).await?
            .to_token().await?;

        sqlx::query("UPDATE Drive SET access_token=?, refresh_token=?, created_at=?, expires_in=? WHERE name=?")
            .bind(&token.access_token)
            .bind(&token.refresh_token)
            .bind(token.created_at)
            .bind(token.expires_in)
            .bind(&self.name)
            .execute(&mut *conn).await?;

        println!("\ndrive \"{}\" successfully authenticated", self.name);
        Ok(())
    }
}

/// Forget credentials for a drive.
#[derive(Parser, Debug)]
pub struct DisconnectOpt {
    /// The drive's name.
    pub name: String,
}

impl DisconnectOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let res = sqlx::query("UPDATE Drive SET access_token=NULL, refresh_token=NULL, created_at=NULL, expires_in=NULL WHERE name=?")
            .bind(&self.name)
            .execute(&mut *conn).await?;

        if res.rows_affected() == 0 {
            return Err(anyhow::anyhow!("drive \"{}\" not found", self.name));
        }

        println!("drive \"{}\" successfully unauthenticated", self.name);
        Ok(())
    }
}

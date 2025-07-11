use crate::database::{Database, App, CloudProvider};
use super::errors::*;

use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
pub enum AppsCommand {

    /// Describe an app.
    #[clap(alias = "get")]
    Describe(DescribeOpt),

    /// List available apps.
    #[clap(alias = "ls")]
    List(ListOpt),

    /// Create a new app.
    Create(CreateOpt),

    /// Remove an app.
    #[clap(alias = "rm")]
    Remove(RemoveOpt),

}

/// Group of commands for managing apps.
#[derive(Parser, Debug)]
pub struct AppsOpt {
    #[clap(subcommand)]
    pub command: AppsCommand,
}

impl AppsOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        match &self.command {
            AppsCommand::Describe(opt) => opt.run(db).await,
            AppsCommand::List(opt) => opt.run(db).await,
            AppsCommand::Create(opt) => opt.run(db).await,
            AppsCommand::Remove(opt) => opt.run(db).await,
        }
    }
}

/// Describe an app.
#[derive(Parser, Debug)]
pub struct DescribeOpt {
    /// The app's name.
    pub name: String,
}

impl DescribeOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let row: App = sqlx::query_as("SELECT * FROM App WHERE name = ?")
            .bind(&self.name)
            .fetch_one(&mut *conn).await
            .map_err(handle_not_found_err)?;

        println!("Found App:\n{}", row);
        Ok(())
    }
}

/// List available apps.
#[derive(Parser, Debug)]
pub struct ListOpt {}

impl ListOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let rows: Vec<App> = sqlx::query_as("SELECT * FROM App")
            .fetch_all(&mut *conn).await?;

        if rows.len() == 0 {
            return Err(anyhow::anyhow!("no apps found"));
        }

        println!("Available Apps:");
        rows.iter().for_each(|app| println!(" - {app}"));
        Ok(())
    }
}

/// Create a new app.
#[derive(Parser, Debug)]
pub struct CreateOpt {
    /// The app's name.
    #[arg(long, short)]
    pub name: String,

    /// The cloud provider.
    #[arg(long, short = 'u')]
    pub provider: CloudProvider,

    /// The OAuth2 client ID.
    #[arg(long, short)]
    pub client_id: String,

    /// The auth grant server port.
    #[arg(long, short)]
    pub port: u16,

    /// The OAuth2 client secret.
    #[arg(long)]
    pub client_secret: Option<String>,
}

impl CreateOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        super::utils::is_valid_name(&self.name)?;
        let mut conn = db.acquire().await?;
        sqlx::query("INSERT INTO App VALUES (?, ?, ?, ?, ?)")
            .bind(&self.name)
            .bind(&self.provider)
            .bind(&self.client_id)
            .bind(self.port)
            .bind(self.client_secret.clone())
            .execute(&mut *conn).await
            .map_err(handle_unique_violation_err)?;

        println!("app \"{}\" successfully created", self.name);
        Ok(())
    }
}

/// Remove an app.
#[derive(Parser, Debug)]
pub struct RemoveOpt {
    /// The apps's name.
    pub name: String,
}

impl RemoveOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let row = sqlx::query("DELETE FROM App WHERE name=?")
            .bind(&self.name)
            .execute(&mut *conn)
            .await?;

        if row.rows_affected() == 0 {
            return Err(anyhow::anyhow!("app \"{}\" not found", self.name));
        }

        println!("app \"{}\" successfully deleted", self.name);
        Ok(())
    }
}

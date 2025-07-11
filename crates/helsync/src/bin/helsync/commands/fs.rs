use crate::database::{Database, Filesystem};
use super::errors::*;

use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
pub enum FsCommand {

    /// Describe a filesystem.
    #[clap(alias = "get")]
    Describe(DescribeOpt),

    /// List all available filesystems.
    #[clap(alias = "ls")]
    List(ListOpt),

    /// Create a new filesystem.
    Create(CreateOpt),

    /// Remove a filesystem.
    #[clap(alias = "rm")]
    Remove(RemoveOpt),

}

/// Group of commands for managing filesystems.
#[derive(Parser, Debug)]
pub struct FsOpt {
    #[clap(subcommand)]
    pub command: FsCommand,
}

impl FsOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        match &self.command {
            FsCommand::Describe(opt) => opt.run(db).await,
            FsCommand::List(opt) => opt.run(db).await,
            FsCommand::Create(opt) => opt.run(db).await,
            FsCommand::Remove(opt) => opt.run(db).await,
        }
    }
}

/// Describe a Filesystem.
#[derive(Parser, Debug)]
pub struct DescribeOpt {
    /// The filesystem's local path.
    pub path: String,
}

impl DescribeOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let abs_path = std::fs::canonicalize(&self.path)?;
        let abs_path_str = abs_path.to_str()
            .ok_or(anyhow::anyhow!("invalid path"))?;

        let row: Filesystem = sqlx::query_as("SELECT * FROM Filesystem WHERE path = ?")
            .bind(abs_path_str)
            .fetch_one(&mut *conn).await
            .map_err(handle_not_found_err)?;

        println!("Found Filesystem:\n{}", row);
        Ok(())
    }
}

/// List all available filesystems.
#[derive(Parser, Debug)]
pub struct ListOpt {}

impl ListOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let rows: Vec<Filesystem> = sqlx::query_as("SELECT * FROM Filesystem")
            .fetch_all(&mut *conn).await?;

        if rows.len() == 0 {
            return Err(anyhow::anyhow!("no filesystems found"));
        }

        println!("Available Filesystems:");
        rows.iter().for_each(|fs| println!(" - {fs}"));
        Ok(())
    }
}

/// Create a new app.
#[derive(Parser, Debug)]
pub struct CreateOpt {
    /// The filesystem's local path.
    #[arg(long, short)]
    pub path: String,

    /// The associated drive.
    #[arg(long, short)]
    pub drive: String,
}

impl CreateOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let abs_path = std::fs::canonicalize(&self.path)?;
        let abs_path_str = abs_path.to_str()
            .ok_or(anyhow::anyhow!("invalid path"))?;

        sqlx::query("INSERT INTO Filesystem VALUES (?, ?)")
            .bind(abs_path_str)
            .bind(&self.drive)
            .execute(&mut *conn).await
            .map_err(handle_unique_violation_err)?;

        println!("filesystem \"{}\" successfully created", self.path);
        Ok(())
    }
}

/// Remove a filesystem.
#[derive(Parser, Debug)]
pub struct RemoveOpt {
    /// The filesystem's local path.
    pub path: String,
}

impl RemoveOpt {
    pub async fn run(&self, db: &Database) -> Result<()> {
        let mut conn = db.acquire().await?;
        let abs_path = std::fs::canonicalize(&self.path)?;
        let abs_path_str = abs_path.to_str()
            .ok_or(anyhow::anyhow!("invalid path"))?;

        let row = sqlx::query("DELETE FROM Filesystem WHERE path=?")
            .bind(abs_path_str)
            .execute(&mut *conn)
            .await?;

        if row.rows_affected() == 0 {
            return Err(anyhow::anyhow!("filesystem \"{}\" not found", self.path));
        }

        println!("filesystem \"{}\" successfully deleted", self.path);
        Ok(())
    }
}

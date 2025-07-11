use crate::database::{Database, Drive, Filesystem, App, CloudProvider};
use helsync::oauth2;
use helsync::fs;
use super::errors::*;

use sqlx::Error as SqlxError;
use anyhow::Result;
use clap::Parser;

/// Group of commands for inspecting sync changes.
#[derive(Parser, Debug)]
pub struct StatusOpt {
    /// Name of drive to merge to.
    pub name: String,
}

impl StatusOpt {
    pub async fn run (&self, db: &Database) -> Result<()> {
        unimplemented!();
    }
}

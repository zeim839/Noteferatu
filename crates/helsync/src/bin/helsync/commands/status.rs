use crate::database::Database;
use anyhow::Result;
use clap::Parser;

/// Group of commands for inspecting sync changes.
#[derive(Parser, Debug)]
pub struct StatusOpt {
    /// Name of drive to merge to.
    pub name: String,
}

impl StatusOpt {
    pub async fn run (&self, _: &Database) -> Result<()> {
        unimplemented!();
    }
}

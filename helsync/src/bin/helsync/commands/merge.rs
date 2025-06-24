use crate::database::Database;
use anyhow::Result;
use clap::Parser;

/// Group of commands for merging sync changes.
#[derive(Parser, Debug)]
pub struct MergeOpt {
    /// Name of drive to merge to.
    #[arg(long, short)]
    pub name: String,

    /// Path to local drive data.
    pub path: String,
}

impl MergeOpt {
    pub async fn run (&self, _: &Database) -> Result<()> {
        unimplemented!();
    }
}

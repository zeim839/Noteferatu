mod apps;
mod drive;
mod merge;
mod status;
mod errors;
mod utils;
mod fs;

use anyhow::Result;

use crate::database::Database;

use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap::Parser;

/// Configures the color/decoration scheme of the clap parser.
const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Blue.on_default().bold())
    .usage(AnsiColor::Blue.on_default().bold())
    .literal(AnsiColor::BrightMagenta.on_default())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Parser, Debug)]
pub enum Command {

    /// Synchronize pending changes.
    Merge(merge::MergeOpt),

    /// Show pending changes.
    Status(status::StatusOpt),

    /// Manage Filesystems.
    Fs(fs::FsOpt),

    /// Manage synchronization drives.
    Drive(drive::DriveOpt),

    /// Manage OAuth2 app registrations.
    Apps(apps::AppsOpt),

}

#[derive(Parser, Debug)]
#[command(version, styles = HELP_STYLES)]
pub struct Opt {
    #[clap(subcommand)]
    pub command: Command,
}

impl Opt {
    pub async fn run(&self) -> Result<()> {
        let db = Database::new().await?;
        match &self.command {
            Command::Merge(opt) => opt.run(&db).await,
            Command::Status(opt) => opt.run(&db).await,
            Command::Fs(opt) => opt.run(&db).await,
            Command::Drive(opt) => opt.run(&db).await,
            Command::Apps(opt) => opt.run(&db).await,
        }
    }
}

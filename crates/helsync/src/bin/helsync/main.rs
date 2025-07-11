mod commands;
mod database;

use console::style;
use clap::Parser;

#[tokio::main]
async fn main() {
    let opt = commands::Opt::parse();
    if let Err(error) = opt.run().await {
        println!("{} {}", style("error:").bold().red(), error);
        std::process::exit(1);
    }
}

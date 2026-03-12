use clap::{Parser, Subcommand};

mod cli;
mod core;
mod registry;
mod installer;
mod config;
mod error;

use cli::HelloCommand;

#[derive(Parser)]
#[command(name = "skillmine")]
#[command(about = "CLI package manager for AI coding assistant skills", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello(HelloArgs),
}

#[derive(Parser)]
struct HelloArgs {
    #[command(subcommand)]
    cmd: HelloCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello(args) => {
            cli::handle_hello(args.cmd).await?;
        }
    }

    Ok(())
}

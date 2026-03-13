use clap::{Parser, Subcommand};
use std::process;

mod cli;
mod config;
mod error;
mod installer;
mod registry;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(
    name = "skillmine",
    version = VERSION,
    about = "The package manager for AI coding assistant skills",
    long_about = "Skillmine brings declarative, deterministic package management to AI coding assistants."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        local: bool,
    },
    Add {
        repo: String,
        #[arg(short, long)]
        branch: Option<String>,
        #[arg(short, long)]
        tag: Option<String>,
    },
    Install {
        #[arg(short, long)]
        force: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Sync {
        #[arg(short, long)]
        target: String,
        #[arg(short, long)]
        path: Option<String>,
    },
    Freeze,
    Thaw,
    List {
        #[arg(short, long)]
        detailed: bool,
    },
    Update {
        skill: Option<String>,
    },
    Remove {
        name: String,
    },
    Info {
        name: String,
    },
    Outdated,
    Doctor,
    Clean {
        #[arg(long)]
        all: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli).await {
        eprintln!("{} error: {}", NAME, error);
        process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Some(Commands::Init { local }) => cli::init(local).await?,
        Some(Commands::Add { repo, branch, tag }) => cli::add(repo, branch, tag).await?,
        Some(Commands::Install { force, verbose }) => cli::install(force, verbose).await?,
        Some(Commands::Sync { target, path }) => cli::sync(target, path).await?,
        Some(Commands::Freeze) => cli::freeze().await?,
        Some(Commands::Thaw) => cli::thaw().await?,
        Some(Commands::List { detailed }) => cli::list(detailed).await?,
        Some(Commands::Update { skill }) => cli::update(skill).await?,
        Some(Commands::Remove { name }) => cli::remove(name).await?,
        Some(Commands::Info { name }) => cli::info(name).await?,
        Some(Commands::Outdated) => cli::outdated().await?,
        Some(Commands::Doctor) => cli::doctor().await?,
        Some(Commands::Clean { all }) => cli::clean(all).await?,
        None => {
            println!("⛏ Skillmine {}", VERSION);
            println!("\nUsage: skillmine <COMMAND>");
            println!("\nRun 'skillmine --help' for more information.");
        }
    }

    Ok(())
}

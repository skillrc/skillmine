use clap::{Parser, Subcommand};
use std::process;

mod cli;
mod core;
mod registry;
mod installer;
mod config;
mod error;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(
    name = "skillmine",
    version = VERSION,
    about = "⛏ The package manager for AI coding assistant skills",
    long_about = "Skillmine brings declarative, deterministic package management \
to Claude Code, OpenCode, Cursor, and other AI coding assistants."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new skillmine configuration
    Init {
        /// Create a local configuration instead of global
        #[arg(short, long)]
        local: bool,
    },

    /// Add a skill to the configuration
    Add {
        /// Repository URL or shorthand (e.g., "owner/repo")
        repo: String,

        /// Specific branch to use
        #[arg(short, long)]
        branch: Option<String>,

        /// Specific tag to use
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Install all configured skills
    Install {
        /// Force reinstall even if already installed
        #[arg(short, long)]
        force: bool,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Sync installed skills to AI assistant
    Sync {
        /// Target AI assistant (claude, opencode, custom)
        #[arg(short, long, value_name = "TARGET")]
        target: String,

        /// Custom path for sync target
        #[arg(short, long, value_name = "PATH")]
        path: Option<String>,
    },

    /// Lock current versions to skills.lock
    Freeze,

    /// Restore versions from skills.lock
    Thaw,

    /// List installed skills
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Update skills to latest versions
    Update {
        /// Specific skill to update
        skill: Option<String>,
    },

    /// Remove a skill
    Remove {
        /// Name of the skill to remove
        name: String,
    },

    /// Check for outdated skills
    Outdated,

    /// Diagnose issues
    Doctor,

    /// Clean cache
    Clean {
        /// Remove all cached data
        #[arg(long)]
        all: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("{} error: {}", NAME, e);
        process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Some(Commands::Init { local }) => {
            cli::init(local).await?;
        }
        Some(Commands::Add { repo, branch, tag }) => {
            cli::add(repo, branch, tag).await?;
        }
        Some(Commands::Install { force, verbose }) => {
            cli::install(force, verbose).await?;
        }
        Some(Commands::Sync { target, path }) => {
            cli::sync(target, path).await?;
        }
        Some(Commands::Freeze) => {
            cli::freeze().await?;
        }
        Some(Commands::Thaw) => {
            cli::thaw().await?;
        }
        Some(Commands::List { detailed }) => {
            cli::list(detailed).await?;
        }
        Some(Commands::Update { skill }) => {
            cli::update(skill).await?;
        }
        Some(Commands::Remove { name }) => {
            cli::remove(name).await?;
        }
        Some(Commands::Outdated) => {
            cli::outdated().await?;
        }
        Some(Commands::Doctor) => {
            cli::doctor().await?;
        }
        Some(Commands::Clean { all }) => {
            cli::clean(all).await?;
        }
        None => {
            println!("⛏ Skillmine {}", VERSION);
            println!("\nUsage: skillmine <COMMAND>");
            println!("\nRun 'skillmine --help' for more information.");
        }
    }

    Ok(())
}

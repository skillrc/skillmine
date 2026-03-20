use clap::{Parser, Subcommand};
use std::process;

mod cli;
mod config;
mod error;
mod installer;
mod lockfile;
mod manifest;
mod pure;
mod registry;
mod tui;

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

#[derive(Debug, Subcommand)]
enum Commands {
    Create {
        name: String,
        #[arg(short = 'o', long)]
        output_dir: Option<String>,
        #[arg(long)]
        and_add: bool,
    },
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
    Enable {
        name: String,
    },
    Disable {
        name: String,
    },
    Unsync {
        name: String,
    },
    Resync {
        name: String,
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
    Tui,
    Clean {
        #[arg(long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(error) = run(cli) {
        eprintln!("{} error: {}", NAME, error);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Some(Commands::Tui) => {
            let action_executor = cli::api::TuiActionExecutor::new()?;
            tui::run(&action_executor)?;
        }
        command => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(run_async(Cli { command }))?;
        }
    }

    Ok(())
}

async fn run_async(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Some(Commands::Create {
            name,
            output_dir,
            and_add,
        }) => {
            let output = if and_add {
                cli::create_and_add(name, output_dir).await?
            } else {
                cli::create(name, output_dir).await?
            };
            println!("{}", output);
        }
        Some(Commands::Init { local }) => cli::init(local).await?,
        Some(Commands::Add { repo, branch, tag }) => cli::add(repo, branch, tag).await?,
        Some(Commands::Enable { name }) => cli::enable(name).await?,
        Some(Commands::Disable { name }) => cli::disable(name).await?,
        Some(Commands::Unsync { name }) => cli::unsync(name).await?,
        Some(Commands::Resync { name }) => cli::resync(name).await?,
        Some(Commands::Install { force, verbose }) => cli::install(force, verbose).await?,
        Some(Commands::Sync { target, path }) => {
            let _ = cli::sync(target, path).await?;
        }
        Some(Commands::Freeze) => cli::freeze().await?,
        Some(Commands::Thaw) => cli::thaw().await?,
        Some(Commands::List { detailed }) => cli::list(detailed).await?,
        Some(Commands::Update { skill }) => cli::update(skill).await?,
        Some(Commands::Remove { name }) => cli::remove(name).await?,
        Some(Commands::Info { name }) => {
            let output = cli::api::info_skill(name).await?;
            println!("{}", output);
        }
        Some(Commands::Outdated) => {
            let output = cli::api::outdated_skills().await?;
            println!("{}", output);
        }
        Some(Commands::Doctor) => cli::api::doctor_skills().await?,
        Some(Commands::Tui) => unreachable!("TUI handled in synchronous entrypoint"),
        Some(Commands::Clean { all }) => cli::api::clean_generated(all).await?,
        None => {
            println!("⛏ Skillmine {}", VERSION);
            println!("\nUsage: skillmine <COMMAND>");
            println!("\nRun 'skillmine --help' for more information.");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parses_create_and_add_flag() {
        let cli = Cli::parse_from(["skillmine", "create", "demo-skill", "--and-add"]);

        match cli.command {
            Some(Commands::Create {
                name,
                output_dir,
                and_add,
            }) => {
                assert_eq!(name, "demo-skill");
                assert_eq!(output_dir, None);
                assert!(and_add);
            }
            other => panic!("expected create command, got {other:?}"),
        }
    }
}

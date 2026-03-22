use clap::{Parser, Subcommand};
use std::process;

mod cli;
mod config;
mod error;
mod installer;
mod manifest;
mod pure;
mod resolved_state;
mod source_refs;
mod tui;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser)]
#[command(
    name = "skillmine",
    version = VERSION,
    about = "Local-first skill lifecycle manager for AI coding assistants",
    long_about = "Skillmine manages local skill creation, registration, preparation, sync, and diagnostics for assistant runtimes."
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
        /// Asset type to create: skill (default), command, agent
        #[arg(short = 't', long = "type", default_value = "skill")]
        asset_type: String,
    },
    /// Manage bundles (named collections of skills/commands/agents)
    Bundle {
        #[command(subcommand)]
        action: BundleCommands,
    },
    Init {
        #[arg(short, long)]
        local: bool,
    },
    Add {
        path: String,
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
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

#[derive(Debug, Subcommand)]
enum ConfigCommands {
    Set {
        key: String,
        value: String,
    },
    Show,
}

#[derive(Debug, Subcommand)]
enum BundleCommands {
    /// Activate a bundle (write skills to opencode config)
    Apply {
        name: String,
        /// Path to opencode config JSON (default: ~/.config/opencode/config.json)
        #[arg(long)]
        config_path: Option<String>,
    },
    /// List all defined bundles
    List,
    /// Deactivate current bundle (clear instructions)
    Clear {
        /// Path to opencode config JSON (default: ~/.config/opencode/config.json)
        #[arg(long)]
        config_path: Option<String>,
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

fn default_opencode_config_path(custom: Option<String>) -> std::path::PathBuf {
    if let Some(p) = custom {
        return std::path::PathBuf::from(p);
    }
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
        .join("opencode")
        .join("config.json")
}

async fn bundle_action(action: BundleCommands) -> Result<(), Box<dyn std::error::Error>> {
    use cli::bundle;

    let config_path = crate::config::io::find_config()?;
    let config = crate::config::io::load_config(&config_path)?;

    match action {
        BundleCommands::Apply { name, config_path: occ_path } => {
            let bundles = &config.bundles;
            let opencode_path = default_opencode_config_path(occ_path);
            let output = bundle::bundle_apply(&name, bundles, &config, &opencode_path)?;
            println!("{}", output);
        }
        BundleCommands::List => {
            let output = bundle::bundle_list(&config.bundles);
            println!("{}", output);
        }
        BundleCommands::Clear { config_path: occ_path } => {
            let opencode_path = default_opencode_config_path(occ_path);
            let output = bundle::bundle_clear(&opencode_path)?;
            println!("{}", output);
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
            asset_type,
        }) => {
            let output = match asset_type.as_str() {
                "command" => cli::command::create(name, output_dir).await?,
                "agent" => cli::agent::create(name, output_dir).await?,
                _ => {
                    if and_add {
                        cli::create_and_add(name, output_dir).await?
                    } else {
                        cli::create(name, output_dir).await?
                    }
                }
            };
            println!("{}", output);
        }
        Some(Commands::Bundle { action }) => {
            bundle_action(action).await?;
        }
        Some(Commands::Init { local }) => cli::init(local).await?,
        Some(Commands::Add { path }) => cli::add(path).await?,
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
        Some(Commands::Config { action }) => match action {
            ConfigCommands::Set { key, value } => {
                let output = cli::config_set(key, value).await?;
                println!("{}", output);
            }
            ConfigCommands::Show => {
                let output = cli::config_show().await?;
                println!("{}", output);
            }
        },
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
                ..
            }) => {
                assert_eq!(name, "demo-skill");
                assert_eq!(output_dir, None);
                assert!(and_add);
            }
            other => panic!("expected create command, got {other:?}"),
        }
    }

    #[test]
    fn test_cli_parses_config_set() {
        let cli = Cli::parse_from(["skillmine", "config", "set", "workspace", "~/Project/Skills"]);

        match cli.command {
            Some(Commands::Config {
                action: ConfigCommands::Set { key, value },
            }) => {
                assert_eq!(key, "workspace");
                assert_eq!(value, "~/Project/Skills");
            }
            other => panic!("expected config set command, got {other:?}"),
        }
    }
}

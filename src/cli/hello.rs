use clap::Subcommand;

#[derive(Subcommand)]
pub enum HelloCommand {
    World,
}

pub async fn handle_hello(cmd: HelloCommand) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        HelloCommand::World => {
            println!("Hello, World!");
        }
    }
    Ok(())
}

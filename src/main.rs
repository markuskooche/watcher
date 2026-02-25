mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "watcher", version = "0.1.3")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Watch,
    Update,
}

fn main() -> notify::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Update => {
            if let Err(error) = commands::update::run() {
                eprintln!("Update command failed: {}", error);
                std::process::exit(1);
            }

            Ok(())
        },
        Command::Watch => {
            if let Err(error) = commands::watch::run() {
                eprintln!("Watch command failed: {}", error);
                std::process::exit(1);
            }

            Ok(())
        }
    }
}

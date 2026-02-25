mod commands;

use clap::{Parser, Subcommand};
use self_update::cargo_crate_version;

#[derive(Parser, Debug)]
#[command(name = "watcher", version = cargo_crate_version!())]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Watch(commands::watch::WatchArgs),
    Update(commands::update::UpdateArgs),
}

fn main() -> notify::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Update(args) => {
            if let Err(error) = commands::update::run(args) {
                eprintln!("Update command failed: {}", error);
                std::process::exit(1);
            }

            Ok(())
        },
        Command::Watch(args) => {
            if let Err(error) = commands::watch::run(args) {
                eprintln!("Watch command failed: {}", error);
                std::process::exit(1);
            }

            Ok(())
        }
    }
}

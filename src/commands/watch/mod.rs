use clap::{Args, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
enum WatchMode {
    Local,
    Network,
}

#[derive(Args, Debug)]
pub struct WatchArgs {
    #[arg(short, long, value_enum)]
    mode: WatchMode,

    #[arg(short, long, value_name = "PATH")]
    path: PathBuf,

    #[arg(short, long, value_name = "TOKEN")]
    token: String,

    #[arg(short, long, default_value_t = 5, value_name = "SECONDS")]
    interval: u64,

    #[arg(short, long, default_value_t = false)]
    recursive: bool,

    #[arg(short, long, value_name = "EXTENSION")]
    extension: Option<String>,
}

pub fn run(args: WatchArgs) -> Result<(), Box<dyn std::error::Error>> {
    if !args.path.exists() {
        eprintln!("ERROR: Path '{}' not found.", args.path.display());
        std::process::exit(1);
    }

    println!("Starting File Watcher...");
    println!("  Path:        {}", args.path.display());
    println!("  Recursive:   {}", args.recursive);
    println!("  Mode:        {:?}", args.mode);
    if matches!(args.mode, WatchMode::Network) {
        println!("  Interval:    {} sec", args.interval);
    }
    if let Some(ref ext) = args.extension {
        println!("  Extension:   .{}", ext);
    }

    match args.mode {
        WatchMode::Local => {
            println!("Starting local file watcher...");
            Ok(())
        },
        WatchMode::Network => {
            println!("Starting network file watcher...");
            Ok(())
        }
    }
}

use std::path;

use clap::{Parser, Subcommand};

use libvale::vale::ValeManager;

/// A helper utility for running Vale in CI environments.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Installs a specific version of Vale to the specified path.
    Install {
        path: path::PathBuf,
        version: Option<String>,
    },
}

fn main() {
    let cli = Args::parse();
    match &cli.command {
        Some(Commands::Install { path, version }) => {
            let manager = ValeManager::new();

            let mut version = match version {
                Some(v) => v,
                None => "latest",
            };

            if version.starts_with("v") {
                version = version.trim_start_matches("v");
            }

            match manager.install(&path, version) {
                Ok(_) => println!("Installed {:?} to {:?}", version, path),
                Err(e) => println!("Error: {:?}", e.to_string()),
            }
        }
        _ => {}
    }
}

use anyhow::Result;
use clap::{Parser, Subcommand};

mod auth_manifest;
use auth_manifest::AuthManifestCommands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RootCommandHierarchy {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Auth Manifest generation and parsing
    AuthManifest {
        #[command(subcommand)]
        subcommand: AuthManifestCommands,
    },
}

fn main() -> Result<()> {
    let cli = RootCommandHierarchy::parse();
    match &cli.command {
        Commands::AuthManifest { subcommand } => match subcommand {
            AuthManifestCommands::Create(opts) => auth_manifest::create(opts)?,
        },
    };
    Ok(())
}

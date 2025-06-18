use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

mod cli;
mod config;
mod git;
mod hooks;

#[derive(Parser)]
#[command(name = "rusky")]
#[command(about = "Fast Git hooks manager written in Rust")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize rusky in your project
    Init,
    /// Install a git hook
    Add {
        /// Hook name (e.g., pre-commit, pre-push)
        hook: String,
        /// Command to run
        command: String,
    },
    /// Remove a git hook
    Remove {
        /// Hook name to remove
        hook: String,
    },
    /// List all configured hooks
    List,
    /// Install git hooks
    Install,
    /// Uninstall git hooks
    Uninstall,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("{}", "🚀 Initializing rusky...".green());
            cli::init().await?;
        }
        Commands::Add { hook, command } => {
            println!("{}", format!("📝 Adding {} hook...", hook).green());
            cli::add_hook(&hook, &command).await?;
        }
        Commands::Remove { hook } => {
            println!("{}", format!("🗑️  Removing {} hook...", hook).yellow());
            cli::remove_hook(&hook).await?;
        }
        Commands::List => {
            println!("{}", "📋 Listing hooks...".blue());
            cli::list_hooks().await?;
        }
        Commands::Install => {
            println!("{}", "⚙️  Installing git hooks...".green());
            cli::install_hooks().await?;
        }
        Commands::Uninstall => {
            println!("{}", "🧹 Uninstalling git hooks...".yellow());
            cli::uninstall_hooks().await?;
        }
    }

    Ok(())
}

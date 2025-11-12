mod alias;
mod cli;
mod command;
mod config;
mod error;
mod shell;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => command::init(),
        Commands::Add {
            name,
            command,
            description,
            group,
        } => command::add(name, command, description, group),
        Commands::Remove { names } => command::remove(names),
        Commands::List {
            group,
            enabled_only,
        } => command::list(group, enabled_only),
        Commands::Search { keyword } => command::search(keyword),
        Commands::Edit {
            name,
            command,
            description,
            group,
        } => command::edit(name, command, description, group),
        Commands::Enable { name } => command::enable(name),
        Commands::Disable { name } => command::disable(name),
        Commands::Export { output, format } => command::export(output, format),
        Commands::Import { file } => command::import(file),
        Commands::Groups => command::groups(),
        Commands::Info => command::info(),
        Commands::Migrate { from, group } => command::migrate(from, group),
    }
}

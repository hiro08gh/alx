mod alias;
mod cli;
mod config;
mod error;
mod shell;

use alias::Alias;
use alias::store::AliasStore;
use alias::validator::AliasValidator;
use clap::Parser;
use cli::{Cli, Commands};
use config::manager::ConfigManager;
use error::Result;
use shell::bash::BashHandler;
use shell::detector::ShellDetector;
use shell::fish::FishHandler;
use shell::zsh::ZshHandler;
use shell::{ShellHandler, ShellType};
use std::fs;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Add {
            name,
            command,
            description,
            group,
        } => cmd_add(name, command, description, group),
        Commands::Remove { name } => cmd_remove(name),
        Commands::List {
            group,
            enabled_only,
        } => cmd_list(group, enabled_only),
        Commands::Search { keyword } => cmd_search(keyword),
        Commands::Edit {
            name,
            command,
            description,
            group,
        } => cmd_edit(name, command, description, group),
        Commands::Enable { name } => cmd_enable(name),
        Commands::Disable { name } => cmd_disable(name),
        Commands::Sync { shell } => cmd_sync(shell),
        Commands::Export { output, format } => cmd_export(output, format),
        Commands::Import { file } => cmd_import(file),
        Commands::Groups => cmd_groups(),
        Commands::Info => cmd_info(),
    }
}

fn cmd_init() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    if config_manager.is_initialized() {
        println!(
            "alx is already initialized at: {:?}",
            config_manager.config_dir()
        );
        return Ok(());
    }

    config_manager.init()?;
    println!(
        "✓ Initialized alx configuration at: {:?}",
        config_manager.config_dir()
    );
    println!("\nNext steps:");
    println!("  1. Add aliases with: alx add <name> <command>");
    println!("  2. Sync to your shell: alx sync");
    println!("  3. Add the following line to your shell config file:");

    let shell = ShellDetector::detect().ok();
    if let Some(shell_type) = shell {
        let handler: Box<dyn ShellHandler> = match shell_type {
            ShellType::Bash => Box::new(BashHandler::new()),
            ShellType::Zsh => Box::new(ZshHandler::new()),
            ShellType::Fish => Box::new(FishHandler::new()),
        };

        if let Ok(config_file) = handler.config_file_path() {
            println!("\n     # Add to {:?}", config_file);
        }

        if shell_type == ShellType::Fish {
            println!("     source ~/.config/alx/shell/aliases.sh");
        } else {
            println!(
                "     [ -f ~/.config/alx/shell/aliases.sh ] && source ~/.config/alx/shell/aliases.sh"
            );
        }
    }

    Ok(())
}

fn cmd_add(
    name: String,
    command: String,
    description: Option<String>,
    group: Option<String>,
) -> Result<()> {
    AliasValidator::validate_name(&name)?;
    AliasValidator::validate_command(&command)?;

    if AliasValidator::is_reserved_keyword(&name) {
        eprintln!("Warning: '{}' is a reserved shell keyword", name);
    }

    let config_manager = ConfigManager::new()?;
    let mut store = AliasStore::load(config_manager.aliases_file())?;

    let mut alias = Alias::new(name.clone(), command);
    if let Some(desc) = description {
        alias = alias.with_description(desc);
    }
    if let Some(grp) = group {
        alias = alias.with_group(grp);
    }

    store.add(alias)?;
    store.save(config_manager.aliases_file())?;

    println!("✓ Added alias: {}", name);
    println!("  Run 'alx sync' to apply changes to your shell");

    Ok(())
}

fn cmd_remove(name: String) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut store = AliasStore::load(config_manager.aliases_file())?;

    store.remove(&name)?;
    store.save(config_manager.aliases_file())?;

    println!("✓ Removed alias: {}", name);
    println!("  Run 'alx sync' to apply changes to your shell");

    Ok(())
}

fn cmd_list(group: Option<String>, enabled_only: bool) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let aliases: Vec<&Alias> = if let Some(grp) = group {
        store.list_by_group(&grp)
    } else if enabled_only {
        store.list_enabled()
    } else {
        store.list().iter().collect()
    };

    if aliases.is_empty() {
        println!("No aliases found");
        return Ok(());
    }

    println!("Aliases:\n");
    for alias in aliases {
        let status = if alias.enabled { "✓" } else { "✗" };
        print!("{} {}: {}", status, alias.name, alias.command);
        if let Some(desc) = &alias.description {
            print!(" - {}", desc);
        }
        if let Some(grp) = &alias.group {
            print!(" [{}]", grp);
        }
        println!();
    }

    Ok(())
}

fn cmd_search(keyword: String) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let results = store.search(&keyword);

    if results.is_empty() {
        println!("No aliases found matching '{}'", keyword);
        return Ok(());
    }

    println!("Search results for '{}':\n", keyword);
    for alias in results {
        print!("{}: {}", alias.name, alias.command);
        if let Some(desc) = &alias.description {
            print!(" - {}", desc);
        }
        println!();
    }

    Ok(())
}

fn cmd_edit(
    name: String,
    command: Option<String>,
    description: Option<String>,
    group: Option<String>,
) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut store = AliasStore::load(config_manager.aliases_file())?;

    let alias = store
        .get_mut(&name)
        .ok_or_else(|| error::AlxError::AliasNotFound(name.clone()))?;

    if let Some(cmd) = command {
        AliasValidator::validate_command(&cmd)?;
        alias.update_command(cmd);
    }

    if let Some(desc) = description {
        alias.description = Some(desc);
        alias.updated_at = chrono::Utc::now();
    }

    if let Some(grp) = group {
        alias.group = Some(grp);
        alias.updated_at = chrono::Utc::now();
    }

    store.save(config_manager.aliases_file())?;

    println!("✓ Updated alias: {}", name);
    println!("  Run 'alx sync' to apply changes to your shell");

    Ok(())
}

fn cmd_enable(name: String) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut store = AliasStore::load(config_manager.aliases_file())?;

    let alias = store
        .get_mut(&name)
        .ok_or_else(|| error::AlxError::AliasNotFound(name.clone()))?;

    alias.enable();
    store.save(config_manager.aliases_file())?;

    println!("✓ Enabled alias: {}", name);
    println!("  Run 'alx sync' to apply changes to your shell");

    Ok(())
}

fn cmd_disable(name: String) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut store = AliasStore::load(config_manager.aliases_file())?;

    let alias = store
        .get_mut(&name)
        .ok_or_else(|| error::AlxError::AliasNotFound(name.clone()))?;

    alias.disable();
    store.save(config_manager.aliases_file())?;

    println!("✓ Disabled alias: {}", name);
    println!("  Run 'alx sync' to apply changes to your shell");

    Ok(())
}

fn cmd_sync(shell: Option<String>) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let shell_type = if let Some(shell_name) = shell {
        match shell_name.as_str() {
            "bash" => ShellType::Bash,
            "zsh" => ShellType::Zsh,
            "fish" => ShellType::Fish,
            _ => return Err(error::AlxError::UnsupportedShell(shell_name)),
        }
    } else {
        ShellDetector::detect()?
    };

    let handler: Box<dyn ShellHandler> = match shell_type {
        ShellType::Bash => Box::new(BashHandler::new()),
        ShellType::Zsh => Box::new(ZshHandler::new()),
        ShellType::Fish => Box::new(FishHandler::new()),
    };

    let enabled_aliases = store.list_enabled();
    let content = handler.generate_aliases_file(&enabled_aliases);

    let shell_aliases_file = config_manager.shell_aliases_file();
    fs::write(&shell_aliases_file, content)?;

    println!(
        "✓ Synced {} aliases to {}",
        enabled_aliases.len(),
        handler.shell_type().as_str()
    );
    println!("  Generated: {:?}", shell_aliases_file);
    println!("\nMake sure your shell config sources this file:");
    println!("  source {:?}", shell_aliases_file);

    Ok(())
}

fn cmd_export(output: Option<String>, format: String) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let content = match format.as_str() {
        "json" => serde_json::to_string_pretty(&store)?,
        "toml" => toml::to_string_pretty(&store)
            .map_err(|e| error::AlxError::ConfigError(e.to_string()))?,
        _ => {
            return Err(error::AlxError::ConfigError(format!(
                "Unsupported format: {}",
                format
            )));
        }
    };

    if let Some(output_path) = output {
        fs::write(&output_path, content)?;
        println!("✓ Exported aliases to: {}", output_path);
    } else {
        println!("{}", content);
    }

    Ok(())
}

fn cmd_import(file: String) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let content = fs::read_to_string(&file)?;

    let imported_store: AliasStore = if file.ends_with(".json") {
        serde_json::from_str(&content)?
    } else if file.ends_with(".toml") {
        toml::from_str(&content)?
    } else {
        // Try to detect format
        serde_json::from_str(&content).or_else(|_| toml::from_str(&content))?
    };

    let mut store = AliasStore::load(config_manager.aliases_file())?;
    let mut imported_count = 0;
    let mut skipped_count = 0;

    for alias in imported_store.aliases {
        if store.exists(&alias.name) {
            skipped_count += 1;
            eprintln!("  Skipped existing alias: {}", alias.name);
        } else {
            store.add(alias.clone())?;
            imported_count += 1;
        }
    }

    store.save(config_manager.aliases_file())?;

    println!("✓ Imported {} aliases", imported_count);
    if skipped_count > 0 {
        println!("  Skipped {} existing aliases", skipped_count);
    }
    println!("  Run 'alx sync' to apply changes to your shell");

    Ok(())
}

fn cmd_groups() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let groups = store.groups();

    if groups.is_empty() {
        println!("No groups found");
        return Ok(());
    }

    println!("Available groups:\n");
    for group in groups {
        let count = store.list_by_group(&group).len();
        println!("  {} ({} aliases)", group, count);
    }

    Ok(())
}

fn cmd_info() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    println!("alx - Modern Alias Manager");
    println!("Version: 0.1.0");
    println!("\nConfiguration:");
    println!("  Config directory: {:?}", config_manager.config_dir());
    println!("  Config file: {:?}", config_manager.config_file());
    println!("  Aliases file: {:?}", config_manager.aliases_file());
    println!("  Shell aliases: {:?}", config_manager.shell_aliases_file());

    if let Ok(store) = AliasStore::load(config_manager.aliases_file()) {
        println!("\nStatistics:");
        println!("  Total aliases: {}", store.list().len());
        println!("  Enabled aliases: {}", store.list_enabled().len());
        println!("  Groups: {}", store.groups().len());
    }

    if let Ok(shell_type) = ShellDetector::detect() {
        println!("\nDetected shell: {}", shell_type.as_str());
    }

    Ok(())
}

use crate::alias::Alias;
use crate::alias::store::AliasStore;
use crate::alias::validator::AliasValidator;
use crate::config::manager::ConfigManager;
use crate::error::{self, Result};
use crate::shell::bash::BashHandler;
use crate::shell::detector::ShellDetector;
use crate::shell::fish::FishHandler;
use crate::shell::zsh::ZshHandler;
use crate::shell::{ShellHandler, ShellType};
use comfy_table::{
    Cell, ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_BORDERS_ONLY,
};
use dialoguer::{Confirm, Select};
use std::fs;

fn sync_aliases() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let shell_type = ShellDetector::detect()?;

    let handler: Box<dyn ShellHandler> = match shell_type {
        ShellType::Bash => Box::new(BashHandler::new()),
        ShellType::Zsh => Box::new(ZshHandler::new()),
        ShellType::Fish => Box::new(FishHandler::new()),
    };

    let aliases: Vec<&crate::alias::Alias> = store.list().iter().collect();
    let content = handler.generate_aliases_file(&aliases);

    let shell_aliases_file = config_manager.shell_aliases_file();
    fs::write(&shell_aliases_file, content)?;

    Ok(())
}

pub fn init() -> Result<()> {
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

    // Detect default shell
    let default_shell = ShellDetector::detect().ok();

    // Create shell options with default shell at the top
    let mut shell_options = vec![];
    if let Some(default) = default_shell {
        shell_options.push(format!("{} (default)", default.as_str()));
    }

    // Add other shells
    let all_shells = vec![ShellType::Bash, ShellType::Zsh, ShellType::Fish];
    for shell in all_shells {
        if Some(shell) != default_shell {
            shell_options.push(shell.as_str().to_string());
        }
    }

    // Prompt user to select shell
    let selection = Select::new()
        .with_prompt("Select your shell")
        .items(&shell_options)
        .default(0)
        .interact()
        .map_err(|e| error::AlxError::ConfigError(format!("Failed to select shell: {}", e)))?;

    // Parse selected shell
    let selected_shell = if let (0, Some(default)) = (selection, default_shell) {
        default
    } else {
        let shell_name = shell_options[selection].split_whitespace().next().unwrap();
        match shell_name {
            "bash" => ShellType::Bash,
            "zsh" => ShellType::Zsh,
            "fish" => ShellType::Fish,
            _ => unreachable!(),
        }
    };

    let handler: Box<dyn ShellHandler> = match selected_shell {
        ShellType::Bash => Box::new(BashHandler::new()),
        ShellType::Zsh => Box::new(ZshHandler::new()),
        ShellType::Fish => Box::new(FishHandler::new()),
    };

    let shell_aliases_file = config_manager.shell_aliases_file();
    let aliases_path = shell_aliases_file.display();

    let source_line = if selected_shell == ShellType::Fish {
        format!("source '{}'", aliases_path)
    } else {
        format!("[ -f '{}' ] && source '{}'", aliases_path, aliases_path)
    };

    // Ask if user wants to add source line automatically
    let config_file = handler.config_file_path()?;

    println!("\nTo enable aliases, add the following line to your shell config:");
    println!("     # Add to '{}'", config_file.display());
    println!("     {}", source_line);

    let should_add = Confirm::new()
        .with_prompt(format!(
            "Do you want to add this line to '{}' automatically?",
            config_file.display()
        ))
        .default(false)
        .interact()
        .map_err(|e| error::AlxError::ConfigError(format!("Failed to confirm: {}", e)))?;

    if should_add {
        // Add source line to shell config
        let mut file_content = if config_file.exists() {
            fs::read_to_string(&config_file)?
        } else {
            String::new()
        };

        // Check if the line already exists
        if file_content.contains(&source_line) {
            println!(
                "✓ Source line already exists in '{}'",
                config_file.display()
            );
        } else {
            // Add newline if file doesn't end with one
            if !file_content.is_empty() && !file_content.ends_with('\n') {
                file_content.push('\n');
            }

            // Add comment and source line
            file_content.push_str("\n# alx - alias manager\n");
            file_content.push_str(&source_line);
            file_content.push('\n');

            fs::write(&config_file, file_content)?;
            println!("✓ Added source line to '{}'", config_file.display());
            println!("\nPlease restart your shell or run:");
            println!("     source '{}'", config_file.display());
        }
    }

    println!("\nNext steps:");
    println!("  1. Add aliases with: alx add <name> <command>");
    println!("  2. Run 'alx list' to see your aliases");

    Ok(())
}

pub fn add(
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

    sync_aliases()?;

    println!("✓ Added alias: {}", name);

    Ok(())
}

pub fn remove(names: Vec<String>) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut store = AliasStore::load(config_manager.aliases_file())?;

    let mut removed_count = 0;
    let mut errors = Vec::new();

    for name in &names {
        match store.remove(name) {
            Ok(_) => {
                removed_count += 1;
            }
            Err(e) => {
                errors.push(format!("{}: {}", name, e));
            }
        }
    }

    if removed_count > 0 {
        store.save(config_manager.aliases_file())?;
        sync_aliases()?;
    }

    if removed_count > 0 {
        if removed_count == 1 {
            println!("✓ Removed 1 alias");
        } else {
            println!("✓ Removed {} aliases", removed_count);
        }
    }

    if !errors.is_empty() {
        eprintln!("\nErrors:");
        for error in &errors {
            eprintln!("  {}", error);
        }
    }

    if removed_count == 0 && !errors.is_empty() {
        return Err(error::AlxError::ConfigError(
            "No aliases were removed".to_string(),
        ));
    }

    Ok(())
}

pub fn list(group: Option<String>) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let aliases: Vec<&Alias> = if let Some(grp) = group {
        store.list_by_group(&grp)
    } else {
        store.list().iter().collect()
    };

    if aliases.is_empty() {
        println!("No aliases found");
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_BORDERS_ONLY);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Name", "Command", "Description", "Group"]);

    for alias in aliases {
        let description = alias.description.as_deref().unwrap_or("-");
        let group = alias.group.as_deref().unwrap_or("-");

        table.add_row(vec![
            Cell::new(&alias.name),
            Cell::new(&alias.command),
            Cell::new(description),
            Cell::new(group),
        ]);
    }

    println!("{table}");

    Ok(())
}

pub fn search(keyword: String) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let results = store.search(&keyword);

    if results.is_empty() {
        println!("No aliases found matching '{}'", keyword);
        return Ok(());
    }

    println!("Search results for '{}':\n", keyword);

    let mut table = Table::new();
    table.load_preset(UTF8_BORDERS_ONLY);
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Name", "Command", "Description", "Group"]);

    for alias in results {
        let description = alias.description.as_deref().unwrap_or("-");
        let group = alias.group.as_deref().unwrap_or("-");

        table.add_row(vec![
            Cell::new(&alias.name),
            Cell::new(&alias.command),
            Cell::new(description),
            Cell::new(group),
        ]);
    }

    println!("{table}");

    Ok(())
}

pub fn edit(
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

    sync_aliases()?;

    println!("✓ Updated alias: {}", name);

    Ok(())
}

pub fn export(output: Option<String>, format: String) -> Result<()> {
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

pub fn import(file: String) -> Result<()> {
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

    sync_aliases()?;

    println!("✓ Imported {} aliases", imported_count);
    if skipped_count > 0 {
        println!("  Skipped {} existing aliases", skipped_count);
    }

    Ok(())
}

pub fn groups() -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let store = AliasStore::load(config_manager.aliases_file())?;

    let groups_list = store.groups();

    if groups_list.is_empty() {
        println!("No groups found");
        return Ok(());
    }

    println!("Available groups:\n");
    for group in groups_list {
        let count = store.list_by_group(&group).len();
        println!("  {} ({} aliases)", group, count);
    }

    Ok(())
}

pub fn info() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    println!("alx - Modern Alias Manager");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("\nConfiguration:");
    println!("  Config directory: {:?}", config_manager.config_dir());
    println!("  Config file: {:?}", config_manager.config_file());
    println!("  Aliases file: {:?}", config_manager.aliases_file());
    println!("  Shell aliases: {:?}", config_manager.shell_aliases_file());

    if let Ok(store) = AliasStore::load(config_manager.aliases_file()) {
        println!("\nStatistics:");
        println!("  Total aliases: {}", store.list().len());
        println!("  Groups: {}", store.groups().len());
    }

    if let Ok(shell_type) = ShellDetector::detect() {
        println!("\nDetected shell: {}", shell_type.as_str());
    }

    Ok(())
}

pub fn migrate(from: Option<String>) -> Result<()> {
    let config_manager = ConfigManager::new()?;
    let mut store = AliasStore::load(config_manager.aliases_file())?;

    // Determine the config file path and shell type
    let (config_path, shell_type) = if let Some(path) = from {
        let path_buf = std::path::PathBuf::from(path);
        let shell_type = ShellDetector::detect_from_path(&path_buf)?;
        (path_buf, shell_type)
    } else {
        let shell_type = ShellDetector::detect()?;
        let handler: Box<dyn ShellHandler> = match shell_type {
            ShellType::Bash => Box::new(BashHandler::new()),
            ShellType::Zsh => Box::new(ZshHandler::new()),
            ShellType::Fish => Box::new(FishHandler::new()),
        };
        (handler.config_file_path()?, shell_type)
    };

    if !config_path.exists() {
        return Err(error::AlxError::ConfigError(format!(
            "Configuration file not found: {:?}",
            config_path
        )));
    }

    let handler: Box<dyn ShellHandler> = match shell_type {
        ShellType::Bash => Box::new(BashHandler::new()),
        ShellType::Zsh => Box::new(ZshHandler::new()),
        ShellType::Fish => Box::new(FishHandler::new()),
    };

    println!("Migrating aliases from: {:?}", config_path);

    // Parse aliases from the config file
    let parsed_aliases = handler.parse_aliases_from_file(&config_path)?;

    if parsed_aliases.is_empty() {
        println!("No aliases found in the configuration file");
        return Ok(());
    }

    let mut imported_count = 0;
    let mut skipped_count = 0;

    for (name, command) in parsed_aliases {
        if store.exists(&name) {
            skipped_count += 1;
            eprintln!("  Skipped existing alias: {}", name);
        } else {
            let alias = Alias::new(name.clone(), command);
            store.add(alias)?;
            imported_count += 1;
        }
    }

    store.save(config_manager.aliases_file())?;

    sync_aliases()?;

    println!("✓ Migrated {} aliases", imported_count);
    if skipped_count > 0 {
        println!("  Skipped {} existing aliases", skipped_count);
    }

    Ok(())
}

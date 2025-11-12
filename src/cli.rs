use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "alx")]
#[command(author = "hiro08gh")]
#[command(version = "0.1.0")]
#[command(about = "A modern alias manager for multiple shells", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Initialize alx configuration
    Init,

    // Add a new alias
    Add {
        // Name of the alias
        name: String,

        // Command to execute
        command: String,

        // Description of the alias
        #[arg(short, long)]
        description: Option<String>,

        // Group/category for the alias
        #[arg(short, long)]
        group: Option<String>,
    },

    // Remove one or more aliases
    Remove {
        // Names of the aliases to remove
        names: Vec<String>,
    },

    // List all aliases
    List {
        // Filter by group
        #[arg(short, long)]
        group: Option<String>,

        // Show only enabled aliases
        #[arg(short, long)]
        enabled_only: bool,
    },

    // Search aliases by keyword
    Search {
        // Keyword to search for
        keyword: String,
    },

    // Edit an alias
    Edit {
        // Name of the alias to edit
        name: String,

        // New command (optional)
        #[arg(short, long)]
        command: Option<String>,

        // New description (optional)
        #[arg(short, long)]
        description: Option<String>,

        // New group (optional)
        #[arg(short, long)]
        group: Option<String>,
    },

    // Enable an alias
    Enable {
        // Name of the alias to enable
        name: String,
    },

    // Disable an alias
    Disable {
        // Name of the alias to disable
        name: String,
    },

    // Export aliases to a file
    Export {
        // Output file path
        #[arg(short, long)]
        output: Option<String>,

        // Export format (json or toml)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    // Import aliases from a file
    Import {
        // Input file path
        file: String,
    },

    // Show all available groups
    Groups,

    // Show information about alx
    Info,

    // Migrate aliases from shell configuration file
    Migrate {
        // Shell configuration file to migrate from (optional)
        #[arg(short, long)]
        from: Option<String>,

        // Group to assign to migrated aliases (optional)
        #[arg(short, long)]
        group: Option<String>,
    },
}

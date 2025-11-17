use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "alx")]
#[command(author = "hiro08gh")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A simple alias manager for multiple shells", long_about = None)]
#[command(
    help_template = "{about-section}\n{usage-heading} {usage}\n\n{all-args}{after-help}",
    after_help = "EXAMPLES:\n    alx add ll 'ls -la' -d 'List all files with details'\n    alx list -g dev\n    alx search git\n    alx remove temp-alias\n    alx export -o aliases.json\n\nFor more information on a specific command, use: alx <COMMAND> --help"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize alx configuration
    ///
    /// Example: alx init
    Init,

    /// Add a new alias
    ///
    /// Example: alx add ll 'ls -la' -d 'List all files' -g utils
    #[command(
        after_help = "EXAMPLES:\n    alx add ll 'ls -la' -d 'List all files with details'\n    alx add gs 'git status' -d 'Show git status' -g git\n    alx add serve 'python -m http.server' -d 'Start HTTP server' -g dev"
    )]
    Add {
        /// Name of the alias
        name: String,

        /// Command to execute
        command: String,

        /// Description of the alias
        #[arg(short, long)]
        description: Option<String>,

        /// Group/category for the alias
        #[arg(short, long)]
        group: Option<String>,
    },

    /// Remove one or more aliases
    ///
    /// Example: alx remove ll gs
    #[command(after_help = "EXAMPLES:\n    alx remove ll\n    alx remove ll gs serve")]
    Remove {
        /// Names of the aliases to remove
        names: Vec<String>,
    },

    /// List all aliases
    ///
    /// Example: alx list -g git
    #[command(after_help = "EXAMPLES:\n    alx list\n    alx list -g git\n    alx list -g dev")]
    List {
        /// Filter by group
        #[arg(short, long)]
        group: Option<String>,
    },

    /// Search aliases by keyword
    ///
    /// Example: alx search git
    #[command(
        after_help = "EXAMPLES:\n    alx search git\n    alx search 'list files'\n    alx search server"
    )]
    Search {
        /// Keyword to search for
        keyword: String,
    },

    /// Edit an alias
    ///
    /// Example: alx edit ll -c 'ls -lah' -d 'Updated description'
    #[command(
        after_help = "EXAMPLES:\n    alx edit ll -c 'ls -lah'\n    alx edit gs -d 'Show git status with branch info'\n    alx edit serve -g webdev"
    )]
    Edit {
        /// Name of the alias to edit
        name: String,

        /// New command (optional)
        #[arg(short, long)]
        command: Option<String>,

        /// New description (optional)
        #[arg(short, long)]
        description: Option<String>,

        /// New group (optional)
        #[arg(short, long)]
        group: Option<String>,
    },

    /// Export aliases to a file
    ///
    /// Example: alx export -o aliases.json -f json
    #[command(
        after_help = "EXAMPLES:\n    alx export\n    alx export -o my-aliases.json\n    alx export -o aliases.toml -f toml"
    )]
    Export {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Export format (json or toml)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Import aliases from a file
    ///
    /// Example: alx import aliases.json
    #[command(
        after_help = "EXAMPLES:\n    alx import aliases.json\n    alx import backup.toml\n    alx import ~/Downloads/shared-aliases.json"
    )]
    Import {
        /// Input file path
        file: String,
    },

    /// Show all available groups
    ///
    /// Example: alx groups
    Groups,

    /// Show information about alx
    ///
    /// Example: alx info
    Info,

    /// Migrate aliases from shell configuration file
    ///
    /// Example: alx migrate -f ~/.bashrc
    #[command(
        after_help = "EXAMPLES:\n    alx migrate\n    alx migrate -f ~/.bashrc\n    alx migrate -f ~/.zshrc"
    )]
    Migrate {
        /// Shell configuration file to migrate from (optional)
        #[arg(short, long)]
        from: Option<String>,
    },
}

pub mod bash;
pub mod detector;
pub mod fish;
pub mod zsh;

use crate::alias::Alias;
use crate::error::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
}

impl ShellType {
    pub fn as_str(&self) -> &str {
        match self {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
        }
    }
}

pub trait ShellHandler {
    fn shell_type(&self) -> ShellType;
    fn generate_alias_line(&self, alias: &Alias) -> String;
    fn generate_aliases_file(&self, aliases: &[&Alias]) -> String;
    fn config_file_path(&self) -> Result<std::path::PathBuf>;
}

use crate::error::{AlxError, Result};
use crate::shell::ShellType;
use std::env;
use std::path::Path;

pub struct ShellDetector;

impl ShellDetector {
    pub fn detect() -> Result<ShellType> {
        // Try SHELL environment variable first
        if let Ok(shell_path) = env::var("SHELL")
            && let Some(shell_name) = shell_path.split('/').next_back()
        {
            return Self::parse_shell_name(shell_name);
        }

        // Fallback: try to detect from parent process
        #[cfg(unix)]
        {
            if let Some(shell) = Self::detect_from_parent() {
                return Ok(shell);
            }
        }

        Err(AlxError::ShellDetectionFailed)
    }

    pub fn detect_from_path<P: AsRef<Path>>(path: P) -> Result<ShellType> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                AlxError::ConfigError(format!("Invalid file path: {:?}", path))
            })?;

        // Check for bash config files
        if file_name.contains("bash") {
            return Ok(ShellType::Bash);
        }

        // Check for zsh config files
        if file_name.contains("zsh") {
            return Ok(ShellType::Zsh);
        }

        // Check for fish config files
        if file_name.contains("fish") || path.to_string_lossy().contains("fish") {
            return Ok(ShellType::Fish);
        }

        Err(AlxError::ConfigError(format!(
            "Could not detect shell type from file path: {:?}",
            path
        )))
    }

    fn parse_shell_name(name: &str) -> Result<ShellType> {
        if !Self::is_supported(name) {
            return Err(AlxError::UnsupportedShell(name.to_string()));
        }

        match name {
            "bash" => Ok(ShellType::Bash),
            "zsh" => Ok(ShellType::Zsh),
            "fish" => Ok(ShellType::Fish),
            _ => unreachable!("already validated the shell name"),
        }
    }

    #[cfg(unix)]
    fn detect_from_parent() -> Option<ShellType> {
        use std::process::Command;

        let output = Command::new("ps")
            .args(["-p", &format!("{}", std::process::id()), "-o", "comm="])
            .output()
            .ok()?;

        let shell_name = String::from_utf8_lossy(&output.stdout);
        let shell_name = shell_name.trim();

        Self::parse_shell_name(shell_name).ok()
    }

    pub fn is_supported(shell: &str) -> bool {
        matches!(shell, "bash" | "zsh" | "fish")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shell_name() {
        assert_eq!(
            ShellDetector::parse_shell_name("bash").unwrap(),
            ShellType::Bash
        );
        assert_eq!(
            ShellDetector::parse_shell_name("zsh").unwrap(),
            ShellType::Zsh
        );
        assert_eq!(
            ShellDetector::parse_shell_name("fish").unwrap(),
            ShellType::Fish
        );
        assert!(ShellDetector::parse_shell_name("unknown").is_err());
    }

    #[test]
    fn test_is_supported() {
        assert!(ShellDetector::is_supported("bash"));
        assert!(ShellDetector::is_supported("zsh"));
        assert!(ShellDetector::is_supported("fish"));
        assert!(!ShellDetector::is_supported("powershell"));
    }
}

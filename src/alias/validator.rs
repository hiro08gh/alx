use crate::error::{AlxError, Result};

pub struct AliasValidator;

impl AliasValidator {
    // Validate alias name
    // - Must start with a letter or underscore
    // - Can contain letters, numbers, underscores, and hyphens
    // - Cannot be empty
    // - Cannot contain spaces
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(AlxError::InvalidAliasName(
                "Alias name cannot be empty".to_string(),
            ));
        }

        if name.contains(' ') {
            return Err(AlxError::InvalidAliasName(
                "Alias name cannot contain spaces".to_string(),
            ));
        }

        let first_char = name.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(AlxError::InvalidAliasName(
                "Alias name must start with a letter or underscore".to_string(),
            ));
        }

        for c in name.chars() {
            if !c.is_alphanumeric() && c != '_' && c != '-' {
                return Err(AlxError::InvalidAliasName(format!(
                    "Alias name contains invalid character: '{}'",
                    c
                )));
            }
        }

        Ok(())
    }

    // Validate alias command
    // - Cannot be empty
    pub fn validate_command(command: &str) -> Result<()> {
        if command.trim().is_empty() {
            return Err(AlxError::InvalidCommand(
                "Command cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    // Check if name is a reserved shell keyword
    pub fn is_reserved_keyword(name: &str) -> bool {
        const RESERVED: &[&str] = &[
            "if", "then", "else", "elif", "fi", "case", "esac", "for", "select", "while", "until",
            "do", "done", "in", "function", "time", "coproc", "cd", "exit", "export", "alias",
            "unalias", "source", ".", "exec",
        ];

        RESERVED.contains(&name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(AliasValidator::validate_name("ll").is_ok());
        assert!(AliasValidator::validate_name("my_alias").is_ok());
        assert!(AliasValidator::validate_name("my-alias").is_ok());
        assert!(AliasValidator::validate_name("_private").is_ok());
        assert!(AliasValidator::validate_name("alias123").is_ok());
    }

    #[test]
    fn test_invalid_names() {
        assert!(AliasValidator::validate_name("").is_err());
        assert!(AliasValidator::validate_name("my alias").is_err());
        assert!(AliasValidator::validate_name("123alias").is_err());
        assert!(AliasValidator::validate_name("-alias").is_err());
        assert!(AliasValidator::validate_name("my@alias").is_err());
    }

    #[test]
    fn test_valid_commands() {
        assert!(AliasValidator::validate_command("ls -la").is_ok());
        assert!(AliasValidator::validate_command("git status").is_ok());
    }

    #[test]
    fn test_invalid_commands() {
        assert!(AliasValidator::validate_command("").is_err());
        assert!(AliasValidator::validate_command("   ").is_err());
    }

    #[test]
    fn test_reserved_keywords() {
        assert!(AliasValidator::is_reserved_keyword("if"));
        assert!(AliasValidator::is_reserved_keyword("export"));
        assert!(!AliasValidator::is_reserved_keyword("myalias"));
    }
}

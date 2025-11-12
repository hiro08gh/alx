pub mod store;
pub mod validator;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub description: Option<String>,
    pub group: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Alias {
    pub fn new(name: String, command: String) -> Self {
        let now = Utc::now();
        Self {
            name,
            command,
            description: None,
            group: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_group(mut self, group: String) -> Self {
        self.group = Some(group);
        self
    }

    pub fn update_command(&mut self, command: String) {
        self.command = command;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_alias() {
        let alias = Alias::new("ll".to_string(), "ls -la".to_string());
        assert_eq!(alias.name, "ll");
        assert_eq!(alias.command, "ls -la");
        assert!(alias.description.is_none());
        assert!(alias.group.is_none());
    }

    #[test]
    fn test_alias_with_description() {
        let alias = Alias::new("ll".to_string(), "ls -la".to_string())
            .with_description("List all files".to_string());
        assert_eq!(alias.description, Some("List all files".to_string()));
    }

    #[test]
    fn test_alias_with_group() {
        let alias =
            Alias::new("gs".to_string(), "git status".to_string()).with_group("git".to_string());
        assert_eq!(alias.group, Some("git".to_string()));
    }
}

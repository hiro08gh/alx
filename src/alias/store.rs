use crate::alias::Alias;
use crate::error::{AlxError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AliasStore {
    pub aliases: Vec<Alias>,
}

impl AliasStore {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path)?;
        let store: AliasStore = toml::from_str(&content)?;
        Ok(store)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content =
            toml::to_string_pretty(self).map_err(|e| AlxError::ConfigError(e.to_string()))?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn add(&mut self, alias: Alias) -> Result<()> {
        if self.exists(&alias.name) {
            return Err(AlxError::AliasExists(alias.name.clone()));
        }
        self.aliases.push(alias);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<Alias> {
        let index = self
            .aliases
            .iter()
            .position(|a| a.name == name)
            .ok_or_else(|| AlxError::AliasNotFound(name.to_string()))?;

        Ok(self.aliases.remove(index))
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Alias> {
        self.aliases.iter_mut().find(|a| a.name == name)
    }

    pub fn exists(&self, name: &str) -> bool {
        self.aliases.iter().any(|a| a.name == name)
    }

    pub fn list(&self) -> &[Alias] {
        &self.aliases
    }

    pub fn list_by_group(&self, group: &str) -> Vec<&Alias> {
        self.aliases
            .iter()
            .filter(|a| a.group.as_deref() == Some(group))
            .collect()
    }

    pub fn search(&self, keyword: &str) -> Vec<&Alias> {
        let keyword_lower = keyword.to_lowercase();
        self.aliases
            .iter()
            .filter(|a| {
                a.name.to_lowercase().contains(&keyword_lower)
                    || a.command.to_lowercase().contains(&keyword_lower)
                    || a.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&keyword_lower))
                        .unwrap_or(false)
            })
            .collect()
    }

    pub fn groups(&self) -> Vec<String> {
        let mut groups: Vec<String> = self
            .aliases
            .iter()
            .filter_map(|a| a.group.clone())
            .collect();
        groups.sort();
        groups.dedup();
        groups
    }
}

impl Default for AliasStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_alias() {
        let mut store = AliasStore::new();
        let alias = Alias::new("ll".to_string(), "ls -la".to_string());

        assert!(store.add(alias.clone()).is_ok());
        assert_eq!(store.aliases.len(), 1);
        assert!(store.add(alias).is_err()); // Duplicate
    }

    #[test]
    fn test_remove_alias() {
        let mut store = AliasStore::new();
        let alias = Alias::new("ll".to_string(), "ls -la".to_string());
        store.add(alias).unwrap();

        assert!(store.remove("ll").is_ok());
        assert_eq!(store.aliases.len(), 0);
        assert!(store.remove("ll").is_err()); // Not found
    }

    #[test]
    fn test_search() {
        let mut store = AliasStore::new();
        store
            .add(Alias::new("ll".to_string(), "ls -la".to_string()))
            .unwrap();
        store
            .add(Alias::new("gs".to_string(), "git status".to_string()))
            .unwrap();

        let results = store.search("git");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "gs");
    }

    #[test]
    fn test_groups() {
        let mut store = AliasStore::new();
        store
            .add(
                Alias::new("gs".to_string(), "git status".to_string())
                    .with_group("git".to_string()),
            )
            .unwrap();
        store
            .add(Alias::new("gp".to_string(), "git push".to_string()).with_group("git".to_string()))
            .unwrap();
        store
            .add(
                Alias::new("dps".to_string(), "docker ps".to_string())
                    .with_group("docker".to_string()),
            )
            .unwrap();

        let groups = store.groups();
        assert_eq!(groups.len(), 2);
        assert!(groups.contains(&"git".to_string()));
        assert!(groups.contains(&"docker".to_string()));
    }
}

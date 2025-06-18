use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

const CONFIG_FILE: &str = ".rusky/config.json";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub hooks: HashMap<String, String>,
    pub version: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hooks: HashMap::new(),
            version: "0.1.0".to_string(),
        }
    }
}

impl Config {
    pub async fn load() -> Result<Self> {
        if !Path::new(CONFIG_FILE).exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(CONFIG_FILE).await?;
        let config: Config = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;

        Ok(config)
    }

    pub async fn save(&self) -> Result<()> {
        // .rusky 디렉토리가 없다면 생성
        let rusky_dir = Path::new(".rusky");
        if !rusky_dir.exists() {
            fs::create_dir(rusky_dir).await?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;

        fs::write(CONFIG_FILE, content).await?;
        Ok(())
    }

    pub fn add_hook(&mut self, hook_name: String, command: String) {
        self.hooks.insert(hook_name, command);
    }

    pub fn remove_hook(&mut self, hook_name: &str) -> bool {
        self.hooks.remove(hook_name).is_some()
    }

    #[allow(dead_code)]
    pub fn get_hook(&self, hook_name: &str) -> Option<&String> {
        self.hooks.get(hook_name)
    }

    #[allow(dead_code)]
    pub fn has_hooks(&self) -> bool {
        !self.hooks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;

    #[tokio::test]
    async fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.version, "0.1.0");
        assert!(config.hooks.is_empty());
    }

    #[tokio::test]
    async fn test_add_hook() {
        let mut config = Config::default();
        config.add_hook("pre-commit".to_string(), "echo test".to_string());
        
        assert_eq!(config.hooks.len(), 1);
        assert_eq!(config.hooks.get("pre-commit"), Some(&"echo test".to_string()));
    }

    #[tokio::test]
    async fn test_remove_hook() {
        let mut config = Config::default();
        config.add_hook("pre-commit".to_string(), "echo test".to_string());
        
        let removed = config.remove_hook("pre-commit");
        assert!(removed);
        assert!(config.hooks.is_empty());
        
        let not_removed = config.remove_hook("non-existent");
        assert!(!not_removed);
    }

    #[tokio::test]
    async fn test_get_hook() {
        let mut config = Config::default();
        config.add_hook("pre-commit".to_string(), "echo test".to_string());
        
        assert_eq!(config.get_hook("pre-commit"), Some(&"echo test".to_string()));
        assert_eq!(config.get_hook("non-existent"), None);
    }

    #[tokio::test]
    async fn test_has_hooks() {
        let mut config = Config::default();
        assert!(!config.has_hooks());
        
        config.add_hook("pre-commit".to_string(), "echo test".to_string());
        assert!(config.has_hooks());
    }

    #[tokio::test]
    async fn test_save_and_load_config() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동
        env::set_current_dir(temp_dir.path()).unwrap();
        
        // .rusky 디렉토리가 없는 상태에서 시작
        if std::path::Path::new(".rusky").exists() {
            tokio::fs::remove_dir_all(".rusky").await.unwrap();
        }
        
        // 설정 생성 및 저장
        let mut config = Config::default();
        config.add_hook("pre-commit".to_string(), "echo test".to_string());
        config.add_hook("pre-push".to_string(), "npm test".to_string());
        
        config.save().await.unwrap();
        
        // 설정 로드 및 검증
        let loaded_config = Config::load().await.unwrap();
        assert_eq!(loaded_config.hooks.len(), 2);
        assert_eq!(loaded_config.hooks.get("pre-commit"), Some(&"echo test".to_string()));
        assert_eq!(loaded_config.hooks.get("pre-push"), Some(&"npm test".to_string()));
        assert_eq!(loaded_config.version, "0.1.0");
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }

    #[tokio::test]
    async fn test_load_nonexistent_config() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        
        // 임시 디렉토리로 이동 (설정 파일이 없는 상태)
        env::set_current_dir(temp_dir.path()).unwrap();
        
        let config = Config::load().await.unwrap();
        assert_eq!(config, Config::default());
        
        // 원래 디렉토리로 복원
        env::set_current_dir(original_dir).unwrap();
    }
}

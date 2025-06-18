use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

const CONFIG_FILE: &str = ".rusky/config.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
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

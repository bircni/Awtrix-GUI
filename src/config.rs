use anyhow::Context;
use core::str;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write};

const CONFIG: &str = "awtrix.conf";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    // true = On, false = Off
    pub last_state: bool,
}

// TODO refactoring to use home dir
impl Config {
    pub fn new() -> Self {
        Self::read().unwrap_or_else(|_| Self {
            ip: String::new(),
            last_state: false,
        })
    }

    fn read() -> anyhow::Result<Self> {
        let home_dir = std::env::home_dir().context("Failed to get home directory")?;
        let config_dir = home_dir.join(".config").join("awtrix-gui");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }
        let filepath = config_dir.join(CONFIG);
        anyhow::ensure!(filepath.exists(), "Path does not exist");
        let content = fs::read_to_string(filepath)?;
        serde_json::from_str(&content).context("Failed to deserialize Config")
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let home_dir = std::env::home_dir().context("Failed to get home directory")?;
        let config_dir = home_dir.join(".config").join("awtrix-gui");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }
        let filepath = config_dir.join(CONFIG);
        let mut file = fs::File::create(filepath)?;
        file.write_all(serde_json::to_string_pretty(self)?.as_bytes())?;
        file.flush()?;
        Ok(())
    }
}

use anyhow::Context;
use core::str;
use serde::{Deserialize, Serialize};
use std::{env, fs, io::Write};

const ENV: &str = ".awtrix.env";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    // true = On, false = Off
    pub last_state: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::read().unwrap_or_else(|_| Self {
            ip: String::new(),
            last_state: false,
        })
    }

    fn read() -> anyhow::Result<Self> {
        let curr = env::current_exe()?;
        let filepath = curr.parent().context("Failed to gt parent path")?.join(ENV);
        anyhow::ensure!(filepath.exists(), "Path does not exist");
        let content = fs::read_to_string(filepath)?;
        serde_json::from_str(&content).context("Failed to deserialize Config")
    }

    pub fn write(&self) -> anyhow::Result<()> {
        if let Some(filepath) = env::current_exe()?.parent().map(|x| x.join(ENV)) {
            let mut file = fs::File::create(filepath)?;
            file.write_all(serde_json::to_string(self)?.as_bytes())?;
            file.flush()?;
            Ok(())
        } else {
            anyhow::bail!("Failed to write to file")
        }
    }
}

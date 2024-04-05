use anyhow::Context;
use core::str;
use ping::ping;
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::Write,
    net::IpAddr,
    str::FromStr,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const ENV: &str = ".awtrix.env";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub ip: Option<IpAddr>,
    pub ip_str: String,
    #[serde(skip, default = "Instant::now")]
    pub last_ping: Instant,
    // true = On, false = Off
    pub last_state: bool,
}

impl Config {
    pub fn new() -> Self {
        match Self::read() {
            Ok(config) => {
                let mut config = config;
                config.check_status(true);
                config
            }
            Err(_) => Self {
                ip: None,
                ip_str: String::new(),
                last_ping: Instant::now(),
                last_state: false,
            },
        }
    }

    pub fn check_status(&mut self, force: bool) {
        if !force && self.last_ping.elapsed() < Duration::from_secs(10) {
            return;
        }

        if let Some(ip) = self.ip {
            self.last_state = Config::check_power(ip).is_ok();
            self.last_ping = Instant::now();
        }
    }

    pub fn set_ip(&mut self) -> anyhow::Result<()> {
        let ip = IpAddr::from_str(&self.ip_str).context("Failed to parse IP")?;
        self.ip = Some(ip);
        Config::write(self)?;
        Ok(())
    }

    fn read() -> anyhow::Result<Self> {
        let curr = env::current_exe()?;
        let filepath = curr.parent().context("Failed to gt parent path")?.join(ENV);
        anyhow::ensure!(filepath.exists(), "Path does not exist");
        let content = fs::read_to_string(filepath)?;
        serde_json::from_str(&content).context("Failed to deserialize Config")
    }

    pub fn write(config: &Config) -> anyhow::Result<()> {
        if let Some(filepath) = env::current_exe()?.parent().map(|x| x.join(ENV)) {
            let mut file = fs::File::create(filepath)?;
            file.write_all(serde_json::to_string(config)?.as_bytes())?;
            file.flush()?;
            Ok(())
        } else {
            anyhow::bail!("Failed to write to file")
        }
    }

    fn check_power(ip: IpAddr) -> anyhow::Result<()> {
        check_ip(ip)?;

        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            sender.send(ping(
                ip,
                Some(Duration::from_secs(1)),
                None,
                None,
                None,
                None,
            ))
        });
        match receiver.recv()? {
            Ok(()) => Ok(()),
            _ => anyhow::bail!("Device is not reachable"),
        }
    }
}

pub fn check_ip(ip: IpAddr) -> anyhow::Result<()> {
    if ip.is_unspecified() {
        anyhow::bail!("IP is empty");
    }
    Ok(())
}

use anyhow::Context;
use egui::{Button, DragValue, ScrollArea, SidePanel, Ui};
use reqwest::blocking::Client;
use semver::Version;
use serde_json::{from_str, Value};

use super::status::{self, Stat};

pub struct Device {
    time: i32,
    update_available: bool,
}

impl Device {
    pub const fn new() -> Self {
        Self {
            time: 0,
            update_available: false,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, ip: &str, stats: &Option<Stat>) -> anyhow::Result<()> {
        SidePanel::right("panel")
            .show_separator_line(true)
            .show_inside(ui, |ui| {
                ScrollArea::new([false, true])
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading("Awtrix Options");
                        });
                        ui.separator();
                        if ip.is_empty() {
                            ui.label("No IP set");
                            Ok(())
                        } else {
                            ui.vertical_centered(|ui| {
                                ui.horizontal(|ui| {
                                    // TODO: FIX THIS
                                    Self::power(ui, ip);
                                    // TODO: FIX THIS
                                    Self::reboot(ui, ip);
                                    ui.separator();
                                    self.sleep(ui, ip)
                                });
                            });
                            self.update_device(ui, ip, stats, self.update_available)
                        }
                    })
                    .inner
            })
            .inner
        //Ok(())
    }

    fn power(ui: &mut Ui, ip: &str) {
        ui.horizontal(|ui| {
            ui.button("On").clicked().then(|| Self::set_power(ip, true));
            ui.button("Off")
                .clicked()
                .then(|| Self::set_power(ip, false));
        });
    }

    fn set_power(ip: &str, curr_power: bool) -> anyhow::Result<()> {
        let payload = format!("{{\"power\": {curr_power}}}");
        Client::new()
            .post(format!("http://{ip}/api/power"))
            .body(payload)
            .send()
            .context("Failed to send")?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to set power")
    }

    fn sleep(&mut self, ui: &mut Ui, ip: &str) -> anyhow::Result<()> {
        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut self.time)
                    .speed(1.0)
                    .range(0..=3600)
                    .suffix("s"),
            );
            ui.button("Sleep").clicked().then(|| self.set_sleep(ip))
        })
        .inner
        .unwrap_or(Ok(()))
    }

    fn set_sleep(&self, ip: &str) -> anyhow::Result<()> {
        let payload = format!("{{\"sleep\": {}}}", self.time);
        Client::new()
            .post(format!("http://{ip}/api/sleep"))
            .body(payload)
            .send()?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to set sleep")
    }

    fn reboot(ui: &mut Ui, ip: &str) {
        ui.button("Reboot").clicked().then(|| Self::set_reboot(ip));
    }

    fn set_reboot(ip: &str) -> anyhow::Result<()> {
        Client::new()
            .post(format!("http://{ip}/api/reboot"))
            .body("-")
            .send()?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to reboot")
    }

    fn update_device(
        &mut self,
        ui: &mut Ui,
        ip: &str,
        stats: &Option<Stat>,
        enabled: bool,
    ) -> anyhow::Result<()> {
        if self.update_available {
            ui.add(Button::new("Update now"))
                .on_hover_text("Update device")
                .clicked()
                .then(|| Self::set_update(ip))
                .unwrap_or(Ok(()))
        } else {
            ui.horizontal(|ui| {
                let ret = ui
                    .add_enabled(enabled, Button::new("Update"))
                    .clicked()
                    .then(|| self.check_update(ip));
                if let Some(stats) = stats {
                    ui.label(format!("Version: {}", stats.version));
                }
                ret
            })
            .inner
            .unwrap_or(Ok(()))
        }
    }

    fn check_update(&mut self, ip: &str) -> anyhow::Result<()> {
        let stats = status::get_stats(ip).context("Failed to get stats")?;
        let current = Self::parse_to_version(&stats.version);
        let latest = Self::parse_to_version(&Self::get_latest_tag().unwrap_or_default());
        if current < latest {
            self.update_available = true;
            Ok(())
        } else {
            self.update_available = false;
            anyhow::bail!("No update available")
        }
    }

    pub fn get_latest_tag() -> anyhow::Result<String> {
        let url = "https://api.github.com/repos/Blueforcer/awtrix3/releases/latest";

        let response = match Client::new()
            .get(url)
            .header("User-Agent", "reqwest")
            .send()
        {
            Ok(response) if response.status().is_success() => response,
            _ => anyhow::bail!("Could not get latest tag"),
        };
        let text = response.text().context("Could not get response")?;
        let json = from_str::<Value>(&text).context("Could not read latest tag")?;
        let text = json
            .get("tag_name")
            .context("")?
            .as_str()
            .context("")?
            .to_owned()
            .replace('"', "");

        Ok(text)
    }

    fn parse_to_version(version: &str) -> Version {
        let parts: Vec<u64> = version.split('.').map(|x| x.parse().unwrap_or(0)).collect();
        match parts[..] {
            [a, b, c] => Version::new(a, b, c),
            [a, b] => Version::new(a, b, 0),
            [a] => Version::new(a, 0, 0),
            _ => Version::new(0, 0, 0),
        }
    }

    fn set_update(ip: &str) -> anyhow::Result<()> {
        Client::new()
            .post(format!("http://{ip}/api/doupdate"))
            .body(String::new())
            .send()
            .context("Failed to send")?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to update")
    }
}

use std::time::Duration;

use anyhow::Context;
use egui::{Button, DragValue, Label, ScrollArea, SidePanel, Ui};
use egui_notify::Toasts;
use reqwest::blocking::Client;
use semver::Version;
use serde_json::{from_str, Value};

use super::status::{self, Stat};

pub struct Device {
    toasts: Toasts,
    time: i32,
    update_available: bool,
}

impl Device {
    pub fn new() -> Self {
        Self {
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
            time: 0,
            update_available: false,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        ip: &str,
        stats: &Option<Stat>,
        write_boot: &mut (bool, bool),
    ) {
        SidePanel::right("panel")
            .show_separator_line(true)
            .show_inside(ui, |ui| {
                ScrollArea::new([false, true]).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Awtrix Options");
                    });
                    ui.separator();
                    if ip.is_empty() {
                        ui.label("No IP set");
                    } else {
                        ui.vertical_centered(|ui| {
                            ui.horizontal(|ui| {
                                // TODO: FIX THIS
                                //self.power(ui, ip);
                                self.reboot(ui, ip, write_boot);
                                ui.separator();
                                self.sleep(ui, ip);
                            });
                        });
                        self.update_device(ui, ip, stats);
                    }
                });
            });
        self.toasts.show(ui.ctx());
    }

    /*
    fn power(&mut self, ui: &mut Ui, ip: &str) {
        fn handle_power_result(result: &anyhow::Result<()>, toasts: &mut Toasts, power: &str) {
            match result {
                Ok(()) => toasts
                    .info(format!("Power {power}"))
                    .set_duration(Some(Duration::from_secs(5))),
                Err(_) => toasts
                    .error("Failed to set power")
                    .set_duration(Some(Duration::from_secs(5))),
            };
        }

        ui.horizontal(|ui| {
            ui.button("On").clicked().then(|| {
                handle_power_result(&Self::set_power(ip, true), &mut self.toasts, "On");
            });
            ui.button("Off").clicked().then(|| {
                handle_power_result(&Self::set_power(ip, true), &mut self.toasts, "Off");
            });
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
    */

    fn sleep(&mut self, ui: &mut Ui, ip: &str) {
        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut self.time)
                    .speed(1.0)
                    .clamp_range(0..=3600)
                    .suffix("s"),
            );
            ui.button("Sleep")
                .clicked()
                .then(|| match self.set_sleep(ip) {
                    Ok(()) => self
                        .toasts
                        .info(format!("Sleep set to: {}s", self.time))
                        .set_duration(Some(Duration::from_secs(5))),
                    Err(_) => self
                        .toasts
                        .error("Failed to set sleep")
                        .set_duration(Some(Duration::from_secs(5))),
                });
        });
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

    fn reboot(&mut self, ui: &mut Ui, ip: &str, write_boot: &mut (bool, bool)) {
        ui.button("Reboot")
            .clicked()
            .then(|| match Self::set_reboot(ip) {
                Ok(()) => {
                    write_boot.0 = false;
                    write_boot.1 = false;
                    self.toasts
                        .info("Rebooting device")
                        .set_duration(Some(Duration::from_secs(5)))
                }
                Err(_) => self
                    .toasts
                    .error("Failed to reboot")
                    .set_duration(Some(Duration::from_secs(5))),
            });
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

    fn update_device(&mut self, ui: &mut Ui, ip: &str, stats: &Option<Stat>) {
        ui.horizontal(|ui| {
            ui.add(Button::new("Update"))
                .clicked()
                .then(|| match self.check_update(ip) {
                    Ok(()) => self
                        .toasts
                        .info("Update available")
                        .set_duration(Some(Duration::from_secs(5))),
                    Err(e) => self
                        .toasts
                        .info(e.to_string())
                        .set_duration(Some(Duration::from_secs(5))),
                });
            if let Some(stats) = stats {
                ui.label(format!("Version: {}", stats.version));
            }
        });

        if self.update_available {
            ui.add(Label::new("An Update is available!"));
            ui.add(Button::new("Update now"))
                .on_hover_text("Update device")
                .clicked()
                .then(|| match Self::set_update(ip) {
                    Ok(()) => self
                        .toasts
                        .info("Updating device")
                        .set_duration(Some(Duration::from_secs(5))),
                    Err(_) => self
                        .toasts
                        .error("Failed to update")
                        .set_duration(Some(Duration::from_secs(5))),
                });
        }
    }

    fn check_update(&mut self, ip: &str) -> anyhow::Result<()> {
        let stats = status::get_stats(ip).context("Failed to get stats")?;
        let current = Device::parse_to_version(&stats.version);
        let latest = Device::parse_to_version(&Device::get_latest_tag().unwrap_or_default());
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
            .to_string()
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

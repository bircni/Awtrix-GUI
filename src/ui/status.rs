use std::fmt::Display;

use egui::{Button, Ui};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

pub fn show(ui: &mut Ui, ip: &str, stat: &mut Option<Stat>) -> anyhow::Result<()> {
    if ui.add(Button::new("Refresh")).clicked() {
        match get_stats(ip) {
            Ok(s) => {
                *stat = Some(s);
                return Ok(());
            }
            Err(e) => anyhow::bail!(e),
        }
    }
    ui.add(egui::Label::new(get_string(stat.as_ref())));
    Ok(())
}

pub fn get_stats(ip: &str) -> anyhow::Result<Stat> {
    let response = match get(format!("http://{ip}/api/stats")) {
        Ok(response) if response.status().is_success() => response,
        _ => anyhow::bail!("Failed to get stats"),
    };
    Ok(from_str(&response.text()?)?)
}

fn get_string(stat: Option<&Stat>) -> String {
    stat.as_ref().map_or_else(||
        "Battery: N/A\nBattery Raw: N/A\nData Type: N/A\nLux: N/A\nLDR Raw: N/A\nRAM: N/A\nBrightness: N/A\nTemperature: N/A\nHumidity: N/A\nUptime: N/A\nWiFi Signal: N/A\nMessages: N/A\nVersion: N/A\nIndicator 1: N/A\nIndicator 2: N/A\nIndicator 3: N/A\nApp: N/A\nUID: N/A\nMatrix: N/A\nIP Address: N/A".to_owned(), |stat| format!(
        "Battery: {}%\nBattery Raw: {}\nData Type: {}\nLux: {}\nLDR Raw: {}\nRAM: {}%\nBrightness: {}\nTemperature: {}°C\nHumidity: {}%\nUptime: {}s\nWiFi Signal: {}%\nMessages: {}\nVersion: {}\nIndicator 1: {}\nIndicator 2: {}\nIndicator 3: {}\nApp: {}\nUID: {}\nMatrix: {}\nIP Address: {}",
        stat.bat, stat.bat_raw, stat.data_type, stat.lux, stat.ldr_raw,
        stat.ram, stat.bri, stat.temp, stat.hum, stat.uptime, stat.wifi_signal,
        stat.messages, stat.version, stat.indicator1, stat.indicator2, stat.indicator3,
        stat.app, stat.uid, stat.matrix, stat.ip_address
        ))
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Stat {
    bat: u32,
    bat_raw: u32,
    #[serde(rename = "type")]
    data_type: u32,
    lux: u32,
    ldr_raw: u32,
    ram: u32,
    bri: u32,
    temp: f32,
    hum: f32,
    uptime: u32,
    wifi_signal: i32,
    messages: u32,
    pub version: String,
    indicator1: bool,
    indicator2: bool,
    indicator3: bool,
    app: String,
    uid: String,
    matrix: bool,
    ip_address: String,
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Battery: {}%", self.bat)?;
        writeln!(f, "Battery Raw: {}", self.bat_raw)?;
        writeln!(f, "Data Type: {}", self.data_type)?;
        writeln!(f, "Lux: {}", self.lux)?;
        writeln!(f, "LDR Raw: {}", self.ldr_raw)?;
        writeln!(f, "RAM: {}%", self.ram)?;
        writeln!(f, "Brightness: {}", self.bri)?;
        writeln!(f, "Temperature: {}°C", self.temp)?;
        writeln!(f, "Humidity: {}%", self.hum)?;
        writeln!(f, "Uptime: {}s", self.uptime)?;
        writeln!(f, "WiFi Signal: {}%", self.wifi_signal)?;
        writeln!(f, "Messages: {}", self.messages)?;
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Indicator 1: {}", self.indicator1)?;
        writeln!(f, "Indicator 2: {}", self.indicator2)?;
        writeln!(f, "Indicator 3: {}", self.indicator3)?;
        writeln!(f, "App: {}", self.app)?;
        writeln!(f, "UID: {}", self.uid)?;
        writeln!(f, "Matrix: {}", self.matrix)?;
        writeln!(f, "IP Address: {}", self.ip_address)
    }
}

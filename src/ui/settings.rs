use std::{net::IpAddr, time::Duration};

use anyhow::Context;
use egui::{Align, Button, Layout, ScrollArea, TextEdit, TextStyle, Ui};
use egui_extras::syntax_highlighting;
use egui_notify::Toasts;
use reqwest::blocking::{get, Client};

pub struct Settings {
    language: String,
    code: String,
    toasts: Toasts,
    pub write_boot: (bool, bool),
}

impl Settings {
    pub fn new() -> Self {
        Self {
            language: "json".to_string(),
            code: String::new(),
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
            write_boot: (false, false),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, ip: IpAddr) {
        ui.horizontal(|ui| {
            ui.add(Button::new("Get Settings")).clicked().then(|| {
                self.code = match Self::get_settings(ip) {
                    Ok(settings) => settings,
                    Err(e) => {
                        self.toasts
                            .error(format!("Failed to get settings: {e}"))
                            .set_duration(Some(Duration::from_secs(5)));
                        String::new()
                    }
                };
            });
            ui.add(Button::new("Write Settings"))
                .clicked()
                .then(|| match self.set_settings(ip) {
                    Ok(()) => {
                        self.write_boot = (true, false);
                        self.toasts
                            .info("Settings written")
                            .set_duration(Some(Duration::from_secs(5)));
                    }
                    Err(e) => {
                        self.toasts
                            .error(format!("Failed to write settings: {e}"))
                            .set_duration(Some(Duration::from_secs(5)));
                    }
                });
            if self.write_boot.0 && !self.write_boot.1 {
                ui.label("Reboot required!");
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing();
                ui.add(Button::new(" i ").rounding(40.0))
                    .clicked()
                    .then(|| {
                        if open::that(
                            "https://blueforcer.github.io/awtrix3/#/api?id=change-settings",
                        )
                        .is_err()
                        {
                            self.toasts
                                .error("Failed to open browser")
                                .set_duration(Some(Duration::from_secs(5)));
                        }
                    });
            });
        });
        let theme = syntax_highlighting::CodeTheme::from_style(ui.style());
        let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                syntax_highlighting::highlight(ui.ctx(), &theme, string, &self.language);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };
        ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                TextEdit::multiline(&mut self.code)
                    .font(TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(20)
                    .lock_focus(true)
                    .desired_width(f32::MAX)
                    .layouter(&mut layouter),
            );
        });
        self.toasts.show(ui.ctx());
    }

    fn get_settings(ip: IpAddr) -> anyhow::Result<String> {
        let response = get(format!("http://{ip}/api/settings"))
            .map_err(|_| anyhow::anyhow!("Failed to get settings"))?;

        Ok(response
            .text()?
            .replace(',', ",\n")
            .replace('{', "{\n")
            .replace('}', "\n}"))
    }

    pub fn set_settings(&mut self, ip: IpAddr) -> anyhow::Result<()> {
        Client::new()
            .post(format!("http://{ip}/api/settings"))
            .body(self.code.clone())
            .send()?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to write settings")
    }
}

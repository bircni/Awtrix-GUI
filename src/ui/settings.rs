use anyhow::Context;
use egui::{Align, Button, Layout, ScrollArea, TextEdit, TextStyle, Ui};

pub struct Settings {
    code: String,
}

impl Settings {
    pub const fn new() -> Self {
        Self {
            code: String::new(),
        }
    }

    // #[expect(clippy::unnecessary_wraps, reason = "TODO")]
    pub fn show(&mut self, ui: &mut Ui, ip: &str) {
        ui.horizontal(|ui| {
            if ui.add(Button::new("Get Settings")).clicked() {
                match Self::get_settings(ip) {
                    Ok(settings) => {
                        self.code = settings;
                        return Ok(());
                    }
                    Err(e) => {
                        self.code = String::new();
                        anyhow::bail!(e)
                    }
                };
            }
            if ui.add(Button::new("Write Settings")).clicked() {
                match self.set_settings(ip) {
                    Ok(()) => return Ok(()),
                    Err(e) => anyhow::bail!(e),
                }
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing();
                if ui.add(Button::new(" i ").corner_radius(40.0)).clicked()
                    && open::that("https://blueforcer.github.io/awtrix3/#/api?id=change-settings")
                        .is_err()
                {
                    anyhow::bail!("Failed to open browser")
                }
                Ok(())
            });
            Ok(())
        });

        ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                TextEdit::multiline(&mut self.code)
                    .font(TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(20)
                    .lock_focus(true)
                    .desired_width(f32::MAX), // .layouter(&mut layouter),
            );
        });
    }

    fn get_settings(ip: &str) -> anyhow::Result<String> {
        let mut response = ureq::get(format!("http://{ip}/api/settings"))
            .call()
            .map_err(|_e| anyhow::anyhow!("Failed to get settings"))?;
        let settings = response
            .body_mut()
            .read_to_string()?
            .replace(',', ",\n")
            .replace('{', "{\n")
            .replace('}', "\n}");

        Ok(settings)
    }

    pub fn set_settings(&self, ip: &str) -> anyhow::Result<()> {
        ureq::post(format!("http://{ip}/api/settings"))
            .send(&self.code)?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to write settings")
    }
}

use anyhow::Context;
use egui::{Align, Button, Layout, ScrollArea, TextEdit, TextStyle, Ui};
use egui_extras::syntax_highlighting;
use reqwest::blocking::{get, Client};

pub struct Settings {
    language: String,
    code: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            language: "json".to_owned(),
            code: String::new(),
        }
    }

    // TODO: Remove this lint suppression
    #[allow(clippy::unnecessary_wraps)]
    pub fn show(&mut self, ui: &mut Ui, ip: &str) -> anyhow::Result<()> {
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
                if ui.add(Button::new(" i ").rounding(40.0)).clicked()
                    && open::that("https://blueforcer.github.io/awtrix3/#/api?id=change-settings")
                        .is_err()
                {
                    anyhow::bail!("Failed to open browser")
                }
                Ok(())
            });
            Ok(())
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
        Ok(())
    }

    fn get_settings(ip: &str) -> anyhow::Result<String> {
        let response = get(format!("http://{ip}/api/settings"))
            .map_err(|_e| anyhow::anyhow!("Failed to get settings"))?;

        Ok(response
            .text()?
            .replace(',', ",\n")
            .replace('{', "{\n")
            .replace('}', "\n}"))
    }

    pub fn set_settings(&mut self, ip: &str) -> anyhow::Result<()> {
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

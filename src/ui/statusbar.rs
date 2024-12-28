use crate::config::Config;
use egui::{
    include_image, special_emojis::GITHUB, vec2, Align, Align2, Button, Frame, Image, Layout,
    TextEdit, Ui, Window,
};

use super::Tab;

pub struct StatusBar {
    show_about: bool,
}

impl StatusBar {
    pub const fn new() -> Self {
        Self { show_about: false }
    }

    pub fn show(&mut self, ui: &mut Ui, tab: &mut Tab, config: &mut Config) -> anyhow::Result<()> {
        self.about_window(ui);
        ui.horizontal(|ui| {
            ui.add_enabled_ui(!config.ip.is_empty(), |ui| {
                ui.selectable_value(tab, Tab::Screen, "Screen");
                ui.selectable_value(tab, Tab::Status, "Status");
                ui.selectable_value(tab, Tab::Settings, "Settings");
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(Button::new(" ? ").rounding(40.0))
                    .clicked()
                    .then(|| self.show_about = true);
                let ret = ui
                    .add(Button::new("Save"))
                    .clicked()
                    .then(|| match config.write() {
                        Ok(()) => anyhow::Ok(()),
                        Err(e) => anyhow::bail!(e),
                    })
                    .unwrap_or(Ok(()));
                ui.add(
                    TextEdit::singleline(&mut config.ip)
                        .hint_text("IP")
                        .desired_width(150.0),
                );
                ui.label("IP:");
                ret
            })
            .inner
        })
        .inner
    }

    fn about_window(&mut self, ui: &Ui) {
        Window::new("About")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.show_about)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .fixed_size(vec2(200.0, 150.0))
            .frame(Frame::window(ui.style()).fill(ui.style().visuals.widgets.open.weak_bg_fill))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(
                        Image::new(include_image!("../../res/icon.png"))
                            .shrink_to_fit()
                            .rounding(10.0),
                    );

                    ui.label(format!("{}: {}", "Version", env!("CARGO_PKG_VERSION")));
                    ui.hyperlink_to(
                        format!("{GITHUB} {}", "Github"),
                        "https://github.com/bircni/awtrix-GUI",
                    );

                    ui.hyperlink_to("Built with egui", "https://docs.rs/egui/");
                });
            });
    }
}

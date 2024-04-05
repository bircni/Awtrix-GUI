use std::time::Duration;

use crate::config::Config;
use egui::{
    include_image, special_emojis::GITHUB, vec2, Align, Align2, Button, Frame, Image, Layout,
    TextEdit, Ui, Window,
};
use egui_notify::Toasts;

use super::Tab;

pub struct StatusBar {
    show_about: bool,
    toasts: Toasts,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            show_about: false,
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, tab: &mut Tab, config: &mut Config) {
        ui.horizontal(|ui| {
            if config.last_state {
                ui.selectable_value(tab, Tab::Screen, "Screen");
                ui.selectable_value(tab, Tab::Status, "Status");
                ui.selectable_value(tab, Tab::Settings, "Settings");
                ui.selectable_value(tab, Tab::Custom, "Custom");
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(Button::new(" ? ").rounding(40.0))
                    .clicked()
                    .then(|| self.show_about = true);
                ui.add(Button::new("Set")).clicked().then(|| {
                    if config.set_ip().is_err() {
                        self.toasts
                            .error("Failed to set IP")
                            .set_duration(Some(Duration::from_secs(5)));
                    } else {
                        config.check_status(true);
                    }
                });
                ui.add(
                    TextEdit::singleline(&mut config.ip_str)
                        .hint_text("IP")
                        .desired_width(150.0),
                );
                ui.label("IP:");
            });
        });
        self.about_window(ui);
        self.toasts.show(ui.ctx());
    }

    fn about_window(&mut self, ui: &mut Ui) {
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

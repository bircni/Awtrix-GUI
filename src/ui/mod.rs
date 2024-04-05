use eframe::CreationContext;
use egui::{
    vec2, CentralPanel, Color32, ColorImage, Context, ImageData, TextStyle, TextureOptions,
};
use egui_notify::Toasts;
use std::sync::Arc;

use crate::config::Config;

use self::custom::Custom;
use self::device::Device;
use self::screen::Screen;
use self::settings::Settings;
use self::status::Status;
use self::statusbar::StatusBar;

mod custom;
mod device;
mod screen;
mod settings;
mod status;
mod statusbar;

pub struct App {
    current_tab: Tab,
    config: Config,
    toasts: Toasts,
    stats: Status,
    device: Device,
    screen: Screen,
    settings: Settings,
    statusbar: StatusBar,
    custom: Custom,
}

#[derive(PartialEq)]
enum Tab {
    Screen,
    Status,
    Settings,
    Custom,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.style_mut(|s| {
            s.text_styles.insert(
                TextStyle::Name("subheading".into()),
                TextStyle::Monospace.resolve(s),
            );
            s.text_styles
                .insert(TextStyle::Body, TextStyle::Monospace.resolve(s));
            s.spacing.item_spacing = vec2(10.0, std::f32::consts::PI * 1.76643);
        });

        let screen_texture = cc.egui_ctx.load_texture(
            "screen",
            ImageData::Color(Arc::new(ColorImage::new([1, 10], Color32::TRANSPARENT))),
            TextureOptions::default(),
        );
        Self {
            current_tab: Tab::Screen,
            config: Config::new(),
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
            stats: Status::new(),
            device: Device::new(),
            screen: Screen::new(screen_texture),
            settings: Settings::new(),
            statusbar: StatusBar::new(),
            custom: Custom::new(),
        }
    }
}
/// Main application loop (called every frame)
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.config.check_status(false);
        CentralPanel::default().show(ctx, |ui| {
            self.statusbar
                .show(ui, &mut self.current_tab, &mut self.config);
            ui.vertical_centered(|ui| {
                ui.separator();
            });

            self.device.show(
                ui,
                self.config.ip,
                &self.stats.stat,
                &mut self.settings.write_boot,
            );
            if self.config.last_state {
                if let Some(ip) = self.config.ip {
                    match self.current_tab {
                        Tab::Status => self.stats.show(ui, ip),
                        Tab::Screen => self.screen.show(ui, ip),
                        Tab::Settings => self.settings.show(ui, ip),
                        Tab::Custom => self.custom.show(ui),
                    }
                }
            } else {
                ui.label("Your Awtrix Clock seems to be offline");
                ui.button("Reconnect").clicked().then(|| {
                    self.config.check_status(true);
                });
            }
        });
        self.toasts.show(ctx);
    }
}

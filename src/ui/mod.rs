use eframe::CreationContext;
use egui::{
    vec2, CentralPanel, Color32, ColorImage, Context, ImageData, ScrollArea, TextStyle,
    TextureOptions,
};
use egui_notify::Toasts;
use parking_lot::RwLock;
use status::Stat;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::config::Config;

use self::device::Device;
use self::settings::Settings;
use self::statusbar::StatusBar;

mod device;
mod screen;
mod settings;
mod status;
mod statusbar;

pub struct App {
    current_tab: Tab,
    config: Config,
    toasts: Toasts,
    device: Device,
    settings: Settings,
    statusbar: StatusBar,
    pub stat: Option<Stat>,
    screen_texture: Arc<RwLock<egui::TextureHandle>>,
}

#[derive(PartialEq)]
enum Tab {
    Screen,
    Status,
    Settings,
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
            ImageData::Color(Arc::new(ColorImage::new([320, 80], Color32::TRANSPARENT))),
            TextureOptions::default(),
        );
        let screen_texture = Arc::new(RwLock::new(screen_texture));
        Self {
            current_tab: Tab::Screen,
            config: Config::new(),
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
            device: Device::new(),
            screen_texture,
            settings: Settings::new(),
            statusbar: StatusBar::new(),
            stat: None,
        }
    }
}

/// Main application loop (called every frame)
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.statusbar
                .show(ui, &mut self.current_tab, &mut self.config);
            ui.vertical_centered(|ui| {
                ui.separator();
            });

            self.device.show(
                ui,
                &self.config.ip,
                &self.stat,
                &mut self.settings.write_boot,
            );
            if !self.config.ip.is_empty() {
                let ip = &self.config.ip;
                match self.current_tab {
                    Tab::Status => status::show(ui, ip, &mut self.stat),
                    Tab::Screen => screen::show(ui, ip, self.screen_texture.clone()),
                    Tab::Settings => self.settings.show(ui, ip),
                }
                .unwrap_or_else(|e| {
                    self.toasts
                        .error(e.to_string())
                        .set_duration(Some(std::time::Duration::from_secs(5)));
                });
            }
        });
        self.toasts.show(ctx);
    }
}

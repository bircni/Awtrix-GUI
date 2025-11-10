use anyhow::Ok;
use egui::{CentralPanel, Context, TextStyle, vec2};
use egui_notify::Toasts;
use parking_lot::RwLock;
use status::Stat;
use std::f32;
use std::sync::Arc;

use crate::config::Config;

use self::device::Device;
use self::settings::Settings;
use self::statusbar::StatusBar;

mod device;
mod settings;
mod status;
mod statusbar;

pub struct App {
    current_tab: Arc<RwLock<Tab>>,
    config: Config,
    toasts: Toasts,
    device: Device,
    settings: Settings,
    statusbar: StatusBar,
    pub stat: Option<Stat>,
}

#[derive(PartialEq)]
enum Tab {
    Status,
    Settings,
}

impl Tab {
    const fn as_str(&self) -> &str {
        match self {
            Self::Status => "Status",
            Self::Settings => "Settings",
        }
    }
}

impl App {
    pub fn new(ctx: &Context) -> Self {
        egui_extras::install_image_loaders(ctx);
        ctx.style_mut(|s| {
            s.text_styles.insert(
                TextStyle::Name("subheading".into()),
                TextStyle::Monospace.resolve(s),
            );
            s.text_styles
                .insert(TextStyle::Body, TextStyle::Monospace.resolve(s));
            s.spacing.item_spacing = vec2(10.0, f32::consts::PI * 1.76643);
        });

        let current_tab = Arc::new(RwLock::new(Tab::Status));
        Self {
            current_tab,
            config: Config::new(),
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
            device: Device::new(ctx),
            settings: Settings::new(),
            statusbar: StatusBar::new(),
            stat: None,
        }
    }
}

impl App {
    pub fn show(&mut self, ctx: &Context) {
        let mut current_tab = self.current_tab.write();
        CentralPanel::default().show(ctx, |ui| {
            self.statusbar
                .show(ui, &mut current_tab, &mut self.config)
                .unwrap_or_else(|e| {
                    self.toasts.error(e.to_string());
                });

            ui.vertical_centered(|ui| {
                ui.separator();
            });

            self.device.show(ui, &self.config.ip);
            if !self.config.ip.is_empty() {
                let ip = &self.config.ip;
                match current_tab.as_str() {
                    "Status" => status::show(ui, ip, &mut self.stat),
                    "Settings" => {
                        self.settings.show(ui, ip);
                        Ok(())
                    }
                    _ => Ok(()),
                }
                .unwrap_or_else(|e| {
                    self.toasts.error(e.to_string());
                });
            }
        });
        self.toasts.show(ctx);
    }
}

/// Main application loop (called every frame)
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.show(ctx);
    }
}

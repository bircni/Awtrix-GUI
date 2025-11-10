use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread, time,
};

use anyhow::Context;
use egui::{
    Color32, ColorImage, DragValue, ImageData, ScrollArea, SidePanel, TextureHandle,
    TextureOptions, Ui,
};
use image::imageops;
use parking_lot::{RawRwLock, RwLock, lock_api};

const SCREEN_SIZE: [usize; 2] = [320, 80];

pub struct Device {
    time: i32,
    screen_texture: Arc<RwLock<egui::TextureHandle>>,
    update_screen: bool,
    auto_refresh_handle: Option<(thread::JoinHandle<()>, Arc<AtomicBool>)>,
}

impl Device {
    pub fn new(ctx: &egui::Context) -> Self {
        let screen_texture = ctx.load_texture(
            "screen",
            ImageData::Color(Arc::new(ColorImage::filled(
                SCREEN_SIZE,
                Color32::TRANSPARENT,
            ))),
            TextureOptions::default(),
        );
        let screen_texture = Arc::new(RwLock::new(screen_texture));
        Self {
            time: 0,
            screen_texture,
            update_screen: true,
            auto_refresh_handle: None,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, ip: &str) {
        SidePanel::right("panel")
            .show_separator_line(true)
            .min_width(340.0)
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
                                Self::power(ui, ip);
                                // TODO: FIX THIS
                                Self::reboot(ui, ip);
                                ui.separator();
                                self.sleep(ui, ip)
                            });
                        });
                        ui.separator();
                        self.show_screen(ui, ip);
                    }
                })
            });
    }

    pub fn show_screen(&mut self, ui: &mut Ui, ip: &str) {
        ui.toggle_value(&mut self.update_screen, "Auto refresh");

        if self.update_screen {
            self.start_auto_refresh(ip.to_owned());
        }

        ui.image(&self.screen_texture.read().clone());
    }

    fn start_auto_refresh(&mut self, ip: String) {
        if self.auto_refresh_handle.is_none() {
            println!("Starting auto refresh thread");
            let texture =
                Arc::<lock_api::RwLock<RawRwLock, TextureHandle>>::clone(&self.screen_texture);
            let running = Arc::new(AtomicBool::new(true));
            let running_clone = Arc::<AtomicBool>::clone(&running);
            // let cycle = Arc::<RwLock<u64>>::clone(&self.auto_refresh_cycle);

            let handle = thread::spawn(move || {
                while running_clone.load(Ordering::Relaxed) {
                    match Self::get_screen(&ip) {
                        Ok(image) => {
                            texture.write().set(image, TextureOptions::default());
                        }
                        Err(e) => {
                            eprintln!("Error fetching screen: {e}");
                        }
                    }
                    thread::sleep(time::Duration::from_secs(2));
                }
            });

            self.auto_refresh_handle = Some((handle, running));
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "The image size is fixed and known to be safe"
    )]
    fn get_screen(ip: &str) -> anyhow::Result<ColorImage> {
        let mut response: ureq::http::Response<ureq::Body> =
            match ureq::get(format!("http://{ip}/api/screen")).call() {
                Ok(response) if response.status().is_success() => response,
                _ => anyhow::bail!("Failed to get screen"),
            };
        let pixels = response
            .body_mut()
            .read_to_string()?
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<u32>>()
            .into_iter()
            .flat_map(|x: u32| {
                [
                    ((x >> 16) & 0xFF) as u8,
                    ((x >> 8) & 0xFF) as u8,
                    (x & 0xFF) as u8,
                ]
            })
            .collect::<Vec<u8>>();
        Ok(ColorImage::from_rgb(
            SCREEN_SIZE,
            &imageops::resize(
                &image::RgbImage::from_vec(32, 8, pixels).context("Failed to create image")?,
                SCREEN_SIZE[0] as u32,
                SCREEN_SIZE[1] as u32,
                imageops::FilterType::Nearest,
            ),
        ))
    }

    fn power(ui: &mut Ui, ip: &str) {
        ui.horizontal(|ui| {
            ui.button("On").clicked().then(|| Self::set_power(ip, true));
            ui.button("Off")
                .clicked()
                .then(|| Self::set_power(ip, false));
        });
    }

    fn set_power(ip: &str, curr_power: bool) -> anyhow::Result<()> {
        let payload = format!("{{\"power\": {curr_power}}}");
        ureq::post(format!("http://{ip}/api/power"))
            .send(&payload)
            .context("Failed to send")?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to set power")
    }

    fn sleep(&mut self, ui: &mut Ui, ip: &str) -> anyhow::Result<()> {
        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut self.time)
                    .speed(1.0)
                    .range(0..=3600)
                    .suffix("s"),
            );
            ui.button("Sleep").clicked().then(|| self.set_sleep(ip))
        })
        .inner
        .unwrap_or(Ok(()))
    }

    fn set_sleep(&self, ip: &str) -> anyhow::Result<()> {
        let payload = format!("{{\"sleep\": {}}}", self.time);
        ureq::post(format!("http://{ip}/api/sleep"))
            .send(&payload)
            .context("Failed to send")?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to set sleep")
    }

    fn reboot(ui: &mut Ui, ip: &str) {
        ui.button("Reboot").clicked().then(|| Self::set_reboot(ip));
    }

    fn set_reboot(ip: &str) -> anyhow::Result<()> {
        ureq::post(format!("http://{ip}/api/reboot"))
            .send("-")
            .context("Failed to send")?
            .status()
            .is_success()
            .then_some(())
            .context("Failed to reboot")
    }
}

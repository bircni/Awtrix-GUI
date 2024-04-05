use std::{net::IpAddr, time::Duration};

use anyhow::Context;
use egui::{ColorImage, TextureHandle, TextureOptions, Ui};
use egui_notify::Toasts;
use reqwest::blocking::get;

pub struct Screen {
    texture: TextureHandle,
    size: [usize; 2],
    toasts: Toasts,
}

impl Screen {
    pub fn new(texture: TextureHandle) -> Self {
        Self {
            texture,
            size: [320, 80],
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, ip: IpAddr) {
        ui.button("Refresh").clicked().then(|| {
            match self.get_screen(ip) {
                Ok(image) => self.texture.set(image, TextureOptions::default()),
                Err(e) => {
                    self.toasts
                        .error(e.to_string())
                        .set_duration(Some(Duration::from_secs(5)));
                }
            };
        });

        ui.image(&self.texture);
        self.toasts.show(ui.ctx());
    }

    fn get_screen(&mut self, ip: IpAddr) -> anyhow::Result<ColorImage> {
        let response = match get(format!("http://{ip}/api/screen")) {
            Ok(response) if response.status().is_success() => response,
            _ => anyhow::bail!("Failed to get screen"),
        };
        let pixels = response
            .text()?
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
            self.size,
            &image::imageops::resize(
                &image::RgbImage::from_vec(32, 8, pixels).context("Failed to create image")?,
                self.size[0] as u32,
                self.size[1] as u32,
                image::imageops::FilterType::Nearest,
            ),
        ))
    }
}

use std::sync::{mpsc, Arc};

use anyhow::Context;
use egui::{ColorImage, TextureHandle, TextureOptions, Ui};
use parking_lot::RwLock;
use reqwest::blocking;

const SIZE: [usize; 2] = [320, 80];

pub fn show(ui: &mut Ui, ip: &str, texture: Arc<RwLock<TextureHandle>>) -> anyhow::Result<()> {
    if ui.button("Refresh").clicked() {
        return threaded_screen(ip.to_string(), texture);
    }

    ui.image(&texture.read().clone());
    Ok(())
}

#[allow(clippy::expect_used)]
fn threaded_screen(ip: String, texture: Arc<RwLock<TextureHandle>>) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let result = match get_screen(&ip) {
            Ok(image) => {
                texture.write().set(image, TextureOptions::default());
                Ok(())
            }
            Err(e) => Err(e),
        };

        tx.send(result).expect("Failed to send result");
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    // Wait for the result from the thread
    match rx.recv() {
        Ok(result) => result,
        Err(_) => Err(anyhow::anyhow!("Failed to receive result from thread")),
    }
}

fn get_screen(ip: &str) -> anyhow::Result<ColorImage> {
    let response = match blocking::get(format!("http://{ip}/api/screen")) {
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
        SIZE,
        &image::imageops::resize(
            &image::RgbImage::from_vec(32, 8, pixels).context("Failed to create image")?,
            SIZE[0] as u32,
            SIZE[1] as u32,
            image::imageops::FilterType::Nearest,
        ),
    ))
}

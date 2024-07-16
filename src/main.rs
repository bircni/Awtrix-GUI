#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![deny(clippy::all)]
#![warn(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
use anyhow::Context;
use egui::ViewportBuilder;

mod config;
mod ui;

fn main() -> anyhow::Result<()> {
    let viewport = ViewportBuilder::default()
        .with_title("Awtrix")
        .with_app_id("awtrix-gui")
        .with_inner_size(egui::vec2(600.0, 400.0))
        .with_icon(
            eframe::icon_data::from_png_bytes(include_bytes!("../res/icon.png"))
                .unwrap_or_default(),
        );

    eframe::run_native(
        "Awtrix",
        eframe::NativeOptions {
            viewport,
            follow_system_theme: true,
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(ui::App::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))
    .context("Failed to run native")
}

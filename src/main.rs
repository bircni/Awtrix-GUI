use anyhow::Context;
use egui::ViewportBuilder;

mod config;
mod ui;

fn main() -> anyhow::Result<()> {
    let viewport = ViewportBuilder::default()
        .with_title("Awtrix")
        .with_app_id("awtrix-gui")
        .with_inner_size(egui::vec2(900.0, 600.0))
        .with_icon(
            eframe::icon_data::from_png_bytes(include_bytes!("../res/icon.png"))
                .unwrap_or_default(),
        );

    eframe::run_native(
        "Awtrix",
        eframe::NativeOptions {
            viewport,
            centered: true,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(ui::App::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))
    .context("Failed to run native")
}

use egui::{ThemePreference, accesskit::Role};
use egui_kittest::{Harness, kittest::Queryable};
use wgpu::InstanceDescriptor;

use crate::ui::App;

pub fn app() -> Harness<'static> {
    let mut app = None;

    Harness::new(move |ctx| {
        let app_instance = app.get_or_insert_with(|| App::new(ctx));
        app_instance.show(ctx);
    })
}

async fn gpu_available() -> bool {
    wgpu::Instance::new(&InstanceDescriptor::default())
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .is_ok()
}

#[tokio::test]
pub async fn test_main_view() {
    if !gpu_available().await {
        return;
    }

    let themes = vec![ThemePreference::Dark, ThemePreference::Light];

    for theme in themes {
        let mut harness = app();
        harness.ctx.set_theme(theme);

        harness.run();
        harness.snapshot(format!("{theme:?}/main_view"));

        let input = harness.get_by_role(Role::TextInput);
        input.focus();
        input.type_text("192.168.1.1");
        harness.run();
        harness.snapshot(format!("{theme:?}/ip_entered"));

        harness
            .get_by_role_and_label(Role::Button, "Settings")
            .click();
        harness.run();
        harness.snapshot(format!("{theme:?}/settings_view"));

        harness.get_by_role_and_label(Role::Button, " ? ").click();

        harness.run();
        harness.snapshot(format!("{theme:?}/about_dialog"));
    }
}

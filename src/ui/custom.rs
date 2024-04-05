use std::time::Duration;

use egui::{vec2, Align, Align2, Button, Color32, Frame, Layout, ScrollArea, TextEdit, Ui, Window};

use egui_notify::Toasts;

use crate::customapp::CustomApp;

pub struct Custom {
    toasts: Toasts,
    app: CustomApp,
    show_preview: bool,
}

impl Custom {
    pub fn new() -> Self {
        Self {
            toasts: Toasts::new().with_anchor(egui_notify::Anchor::BottomLeft),
            app: CustomApp::new(),
            show_preview: false,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add(Button::new("Set App"))
                .clicked()
                .then(|| match self.app.to_json() {
                    Ok(json) => {
                        self.toasts
                            .info("Not implemented yet")
                            .set_duration(Some(Duration::from_secs(5)));
                        println!("{json}");
                    }
                    Err(e) => {
                        self.toasts
                            .info(format!("Error {e}"))
                            .set_duration(Some(Duration::from_secs(5)));
                        println!("{e}");
                    }
                });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(Button::new("Reset")).clicked().then(|| {
                    self.app = CustomApp::new();
                });
                ui.add(Button::new("Preview"))
                    .clicked()
                    .then(|| self.show_preview = true);
            });
        });
        ui.separator();
        //ScrollArea::new([false, true]).show(ui, |ui| {
        //    for (name, value) in self.custom_app.iter() {
        //        // Change the type of value to &mut dyn Any
        //        ui.horizontal(|ui| {
        //            ui.label(name);
        //            if value.is::<Option<bool>>() {
        //                ui.label("bool");
        //            } else if value.is::<Option<f32>>() {
        //                ui.label("f32");
        //            } else if value.is::<Option<i32>>() {
        //                ui.label("i32");
        //            } else if value.is::<Option<Vec<i32>>>() {
        //                ui.label("Option<Vec<i32>>");
        //            } else if value.is::<Option<String>>() {
        //                Self::show_text_edit(ui, self.custom_app.get_mut(&name));
        //            } else {
        //                ui.label("unknown");
        //            }
        //        });
        //    }
        //});
        ScrollArea::new([false, true])
            .max_width(f32::INFINITY)
            .show(ui, |ui| {
                Self::show_text_edit(ui, &mut self.app.text, "Text");
                Self::show_int_edit(ui, &mut self.app.text_case, "Text Case");
                Self::show_bool_edit(ui, &mut self.app.top_text, "Top Text");
                Self::show_int_edit(ui, &mut self.app.text_offset, "Text Offset");
                Self::show_bool_edit(ui, &mut self.app.center, "Center");
                Self::show_color_edit(ui, &mut self.app.color, "Color");
                Self::show_color_edit(ui, &mut self.app.gradient, "Gradient");
                Self::show_int_edit(ui, &mut self.app.blink_text, "Blink Text");
                Self::show_int_edit(ui, &mut self.app.fade_text, "Fade Text");
                Self::show_color_edit(ui, &mut self.app.background, "Background");
                Self::show_bool_edit(ui, &mut self.app.rainbow, "Rainbow");
                Self::show_text_edit(ui, &mut self.app.icon, "Icon");
                Self::show_int_edit(ui, &mut self.app.push_icon, "Push Icon");
                Self::show_int_edit(ui, &mut self.app.repeat, "Repeat");
                Self::show_int_edit(ui, &mut self.app.duration, "Duration");
                Self::show_color_edit(ui, &mut self.app.bar, "Bar");
                Self::show_color_edit(ui, &mut self.app.line, "Line");
                Self::show_bool_edit(ui, &mut self.app.autoscale, "Autoscale");
                Self::show_int_edit(ui, &mut self.app.progress, "Progress");
                Self::show_color_edit(ui, &mut self.app.progress_c, "Progress C");
                Self::show_color_edit(ui, &mut self.app.progress_bc, "Progress BC");
                Self::show_int_edit(ui, &mut self.app.pos, "Pos");
                Self::show_int_edit(ui, &mut self.app.lifetime, "Lifetime");
                Self::show_int_edit(ui, &mut self.app.lifetime_mode, "Lifetime Mode");
                Self::show_bool_edit(ui, &mut self.app.no_scroll, "No Scroll");
                Self::show_int_edit(ui, &mut self.app.scroll_speed, "Scroll Speed");
                Self::show_text_edit(ui, &mut self.app.effect, "Effect");
                Self::show_text_edit(ui, &mut self.app.overlay, "Overlay");
            });

        self.preview_window(ui);
        self.toasts.show(ui.ctx());
    }

    fn show_color_edit(ui: &mut Ui, color: &mut Option<Color32>, name: &str) {
        ui.horizontal(|ui| {
            ui.label(name);
            if let Some(existing_color) = color {
                ui.color_edit_button_srgba(existing_color);
            } else {
                let mut empty_color = Color32::WHITE;
                ui.color_edit_button_srgba(&mut empty_color);
                if empty_color != Color32::WHITE {
                    *color = Some(empty_color);
                }
            }
        });
    }

    fn show_bool_edit(ui: &mut Ui, value: &mut Option<bool>, name: &str) {
        ui.horizontal(|ui| {
            ui.label(name);
            if let Some(existing_value) = value {
                ui.checkbox(existing_value, "");
            } else {
                let mut empty_value = false;
                ui.checkbox(&mut empty_value, "");
                if empty_value {
                    *value = Some(empty_value);
                }
            }
        });
    }

    fn show_int_edit(ui: &mut Ui, value: &mut Option<i32>, name: &str) {
        ui.horizontal(|ui| {
            ui.label(name);
            if let Some(existing_value) = value {
                ui.add(egui::DragValue::new(existing_value));
            } else {
                let mut empty_value = 0;
                ui.add(egui::DragValue::new(&mut empty_value));
                if empty_value != 0 {
                    *value = Some(empty_value);
                }
            }
        });
    }

    fn show_text_edit(ui: &mut Ui, text: &mut String, name: &str) {
        ui.horizontal(|ui| {
            ui.label(name);
            ui.add(TextEdit::singleline(text).desired_width(150.0));
        });
    }

    fn preview_window(&mut self, ui: &mut Ui) {
        let string = match self.app.to_json() {
            Ok(string) => string.replace(',', ",\n"),
            Err(_) => "Failed to serialize".to_string(),
        };
        Window::new("Preview")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.show_preview)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .fixed_size(vec2(200.0, 150.0))
            .frame(Frame::window(ui.style()).fill(ui.style().visuals.widgets.open.weak_bg_fill))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ScrollArea::new([false, true])
                        .max_width(f32::INFINITY)
                        .show(ui, |ui| {
                            ui.label(string);
                        });
                });
            });
    }
}

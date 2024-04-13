use egui::Color32;
use serde::{Deserialize, Serialize, Serializer};
//use struct_iterable::Iterable;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomApp {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(rename = "textCase", skip_serializing_if = "Option::is_none")]
    pub text_case: Option<i32>,
    #[serde(rename = "topText", skip_serializing_if = "Option::is_none")]
    pub top_text: Option<bool>,
    #[serde(rename = "textOffset", skip_serializing_if = "Option::is_none")]
    pub text_offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_to_i32"
    )]
    pub color: Option<Color32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_to_i32"
    )]
    pub gradient: Option<Color32>,
    #[serde(rename = "blinkText", skip_serializing_if = "Option::is_none")]
    pub blink_text: Option<i32>,
    #[serde(rename = "fadeText", skip_serializing_if = "Option::is_none")]
    pub fade_text: Option<i32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_to_i32"
    )]
    pub background: Option<Color32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rainbow: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
    #[serde(rename = "pushIcon", skip_serializing_if = "Option::is_none")]
    pub push_icon: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_to_i32"
    )]
    pub bar: Option<Color32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_to_i32"
    )]
    pub line: Option<Color32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(
        rename = "progressC",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_to_i32"
    )]
    pub progress_c: Option<Color32>,
    #[serde(
        rename = "progressBC",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_to_i32"
    )]
    pub progress_bc: Option<Color32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<i32>,
    #[serde(rename = "lifetimeMode", skip_serializing_if = "Option::is_none")]
    pub lifetime_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_scroll: Option<bool>,
    #[serde(rename = "scrollSpeed", skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<i32>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub effect: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub overlay: String,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_to_i32<S>(c: &Option<Color32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(color) = c {
        let red = i32::from(color.r()) << 16;
        let green = i32::from(color.g()) << 8;
        let blue = i32::from(color.b());
        serializer.serialize_i32(red | green | blue)
    } else {
        serializer.serialize_none()
    }
}

impl CustomApp {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            text_case: None,
            top_text: None,
            text_offset: None,
            center: None,
            color: None,
            gradient: None,
            blink_text: None,
            fade_text: None,
            background: None,
            rainbow: None,
            icon: String::new(),
            push_icon: None,
            repeat: None,
            duration: None,
            bar: None,
            line: None,
            autoscale: None,
            progress: None,
            progress_c: None,
            progress_bc: None,
            pos: None,
            lifetime: None,
            lifetime_mode: None,
            no_scroll: None,
            scroll_speed: None,
            effect: String::new(),
            overlay: String::new(),
        }
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}

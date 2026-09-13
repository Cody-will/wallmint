use crate::config::AppConfig;
use crate::hypr::get_active_opacity;
use crate::wallust::{load_wallust_colors, WallmintColors};

#[derive(Debug, Clone)]
pub struct WallmintTheme {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub secondary: String,
    pub background_alpha: f32,
}

fn fallback_theme() -> WallmintTheme {
    WallmintTheme {
        background: "#0f111a".into(),
        foreground: "#e6e6e6".into(),
        accent: "#7aa2f7".into(),
        secondary: "#a6e3a1".into(),
        background_alpha: get_active_opacity().unwrap_or(0.85),
    }
}

fn pick<'a>(primary: &'a str, fallback: &'a str, last: &'a str) -> String {
    if !primary.is_empty() {
        primary.to_string()
    } else if !fallback.is_empty() {
        fallback.to_string()
    } else {
        last.to_string()
    }
}

pub fn load_theme(cfg: &AppConfig) -> WallmintTheme {
    let cols: WallmintColors = match load_wallust_colors(cfg) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("wallust colors not loaded ({e}); using fallback theme");
            return fallback_theme();
        }
    };

    WallmintTheme {
        background: pick(&cols.background, &cols.color0, "#0f111a"),
        foreground: pick(&cols.foreground, &cols.color15, "#e6e6e6"),
        accent: pick(&cols.color13, &cols.color5, "#7aa2f7"),
        secondary: pick(&cols.color12, &cols.color4, "#a6e3a1"),
        background_alpha: get_active_opacity().unwrap_or(0.85),
    }
}

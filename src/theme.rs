
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

pub fn load_theme(cfg: &AppConfig) -> WallmintTheme {
    let cols: WallmintColors = match load_wallust_colors(cfg) {
        Ok(c) => c,
        Err(_) => return fallback_theme(),
    };

    WallmintTheme {
        background: cols.background,
        foreground: cols.foreground,
        accent: cols.color13,     // your choice
        secondary: cols.color12,  // your choice
        background_alpha: get_active_opacity().unwrap_or(0.85),
    }
}


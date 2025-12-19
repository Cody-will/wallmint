
use crate::wallust::load_wallust_colors;
use crate::hypr::get_active_opacity;

#[derive(Debug, Clone)]
pub struct WallmintTheme {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub background_alpha: f32,
}

pub fn load_theme() -> WallmintTheme {
    let cols = load_wallust_colors();

    let opacity = get_active_opacity().unwrap_or(0.85);

    
    WallmintTheme {
        background: cols.background,
        foreground: cols.foreground,
        accent: cols.color13,
        background_alpha: opacity,
    }
}

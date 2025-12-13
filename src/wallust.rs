
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct WallmintColors {
    pub background: String,
    pub foreground: String,
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
}

#[derive(Debug, Deserialize)]
pub struct WallustFile {
    pub colors: WallmintColors,
}

pub fn load_wallust_colors() -> WallmintColors {
    let path: PathBuf = dirs::cache_dir()
        .expect("Could not find cache dir")
        .join("wallust")
        .join("wallmint.json");

    let raw = std::fs::read_to_string(&path)
        .expect("Failed to read wallmint.json");

    let parsed: WallustFile =
        serde_json::from_str(&raw).expect("Invalid JSON in wallmint.json");

    parsed.colors
}


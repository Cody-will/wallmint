
use crate::config::AppConfig;
use serde::Deserialize;
use std::path::{Path, PathBuf};

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
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
}

#[derive(Debug, Deserialize)]
pub struct WallustFile {
    pub colors: WallmintColors,
}

#[derive(Debug)]
pub enum WallustError {
    MissingPath,
    Io { path: PathBuf, source: std::io::Error },
    Json { path: PathBuf, source: serde_json::Error },
}

impl std::fmt::Display for WallustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallustError::MissingPath => write!(f, "No wallust_colors_json path set in config"),
            WallustError::Io { path, source } => write!(f, "I/O error at {}: {}", path.display(), source),
            WallustError::Json { path, source } => write!(f, "JSON error in {}: {}", path.display(), source),
        }
    }
}
impl std::error::Error for WallustError {}

pub fn load_wallust_colors(cfg: &AppConfig) -> Result<WallmintColors, WallustError> {
    let path = cfg
        .paths
        .wallust_colors_json
        .clone()
        .ok_or(WallustError::MissingPath)?;

    load_wallust_colors_from_path(&path)
}

pub fn load_wallust_colors_from_path(path: &Path) -> Result<WallmintColors, WallustError> {
    let raw = std::fs::read_to_string(path).map_err(|e| WallustError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    let parsed: WallustFile = serde_json::from_str(&raw).map_err(|e| WallustError::Json {
        path: path.to_path_buf(),
        source: e,
    })?;

    Ok(parsed.colors)
}


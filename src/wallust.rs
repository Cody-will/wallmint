use crate::config::AppConfig;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct WallmintColors {
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    pub foreground: String,
    #[serde(default)]
    pub color0: String,
    #[serde(default)]
    pub color1: String,
    #[serde(default)]
    pub color2: String,
    #[serde(default)]
    pub color3: String,
    #[serde(default)]
    pub color4: String,
    #[serde(default)]
    pub color5: String,
    #[serde(default)]
    pub color6: String,
    #[serde(default)]
    pub color7: String,
    #[serde(default)]
    pub color8: String,
    #[serde(default)]
    pub color9: String,
    #[serde(default)]
    pub color10: String,
    #[serde(default)]
    pub color11: String,
    #[serde(default)]
    pub color12: String,
    #[serde(default)]
    pub color13: String,
    #[serde(default)]
    pub color14: String,
    #[serde(default)]
    pub color15: String,
}

#[derive(Debug, Deserialize, Default)]
struct SpecialBlock {
    #[serde(default)]
    background: String,
    #[serde(default)]
    foreground: String,
}

/// Accepts several wallust / pywal layouts.
#[derive(Debug, Deserialize)]
struct WallustFile {
    #[serde(default)]
    colors: Option<WallmintColors>,
    #[serde(default)]
    special: Option<SpecialBlock>,
    #[serde(default)]
    background: Option<String>,
    #[serde(default)]
    foreground: Option<String>,
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
            WallustError::Io { path, source } => {
                write!(f, "I/O error at {}: {}", path.display(), source)
            }
            WallustError::Json { path, source } => {
                write!(f, "JSON error in {}: {}", path.display(), source)
            }
        }
    }
}
impl std::error::Error for WallustError {}

pub fn load_wallust_colors(cfg: &AppConfig) -> Result<WallmintColors, WallustError> {
    let path = cfg
        .paths
        .resolved_wallust_colors()
        .ok_or(WallustError::MissingPath)?;
    load_wallust_colors_from_path(&path)
}

pub fn load_wallust_colors_from_path(path: &Path) -> Result<WallmintColors, WallustError> {
    let raw = std::fs::read_to_string(path).map_err(|e| WallustError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    if let Ok(parsed) = serde_json::from_str::<WallustFile>(&raw) {
        let mut colors = parsed.colors.unwrap_or_default();
        if let Some(special) = parsed.special {
            if colors.background.is_empty() {
                colors.background = special.background;
            }
            if colors.foreground.is_empty() {
                colors.foreground = special.foreground;
            }
        }
        if colors.background.is_empty() {
            if let Some(bg) = parsed.background {
                colors.background = bg;
            }
        }
        if colors.foreground.is_empty() {
            if let Some(fg) = parsed.foreground {
                colors.foreground = fg;
            }
        }
        if !colors.background.is_empty() || !colors.color0.is_empty() {
            return Ok(colors);
        }
    }

    serde_json::from_str::<WallmintColors>(&raw).map_err(|e| WallustError::Json {
        path: path.to_path_buf(),
        source: e,
    })
}


use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub paths: PathsConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    pub wallpaper_dir: PathBuf,
    pub config_dir: Option<PathBuf>,
    pub style_dir: Option<PathBuf>,
    pub config_path: Option<PathBuf>,
    pub wallust_colors_json: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub title: String,
    pub window: WindowConfig,
    pub preview: PreviewConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub height: i32,
    pub width: i32,
    pub resizable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewConfig {
    pub aspect_ratio: String, // e.g. "16:9" or "1:1"
    pub content_fit: String,  // e.g. "contain" | "cover" | "fill"
    pub vexpand: bool,
    pub hexpand: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        // Base dirs
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| config_dir.join(".cache"));
        let pictures_dir = dirs::picture_dir().unwrap_or_else(|| PathBuf::from("."));

        // App dirs
        let app_config_dir = config_dir.join("wallmint");
        let style_dir = app_config_dir.join("styles");
        let config_path = app_config_dir.join("config.json");

        // Wallust output (adjust if your file name differs)
        // You mentioned ~/.cache/wallust/colors.json earlier; keep that as default.
        let wallust_colors_json = cache_dir.join("wallust").join("colors.json");

        Self {
            paths: PathsConfig {
                wallpaper_dir: pictures_dir.join("Wallpapers"),
                config_dir: Some(app_config_dir),
                style_dir: Some(style_dir),
                config_path: Some(config_path),
                wallust_colors_json: Some(wallust_colors_json),
            },
            ui: UiConfig {
                title: "Wallmint".to_string(),
                window: WindowConfig {
                    width: 1100,
                    height: 720,
                    resizable: true,
                },
                preview: PreviewConfig {
                    aspect_ratio: "16:9".to_string(),
                    content_fit: "contain".to_string(),
                    vexpand: true,
                    hexpand: true,
                },
            },
        }
    }
}

impl PathsConfig {
    /// Resolve the config.json path (either explicit or default).
    pub fn resolved_config_path(&self) -> PathBuf {
        if let Some(p) = &self.config_path {
            return p.clone();
        }

        let base = self
            .config_dir
            .clone()
            .or_else(dirs::config_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wallmint");

        base.join("config.json")
    }

    /// Ensure required folders exist when we create/write defaults.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        if let Some(dir) = &self.config_dir {
            fs::create_dir_all(dir)?;
        }
        if let Some(dir) = &self.style_dir {
            fs::create_dir_all(dir)?;
        }
        // wallpaper_dir is user content; don’t force-create unless you want:
        // fs::create_dir_all(&self.wallpaper_dir)?;
        Ok(())
    }
}

impl AppConfig {
    /// Load config.json, or create it with defaults if missing.
    pub fn load_or_create() -> Result<Self, ConfigError> {
        let cfg = AppConfig::default();
        let path = cfg.paths.resolved_config_path();

        // Try read existing
        match fs::read_to_string(&path) {
            Ok(raw) => {
                let parsed: AppConfig = serde_json::from_str(&raw)
                    .map_err(|e| ConfigError::Json { path: path.clone(), source: e })?;

                // If older config omitted optionals, we can patch up missing derived paths:
                Ok(parsed.with_filled_paths())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Make directories and write defaults
                cfg.paths.ensure_dirs().map_err(|e| ConfigError::Io {
                    path: path.clone(),
                    source: e,
                })?;
                cfg.save_to(&path)?;
                Ok(cfg)
            }
            Err(e) => Err(ConfigError::Io { path, source: e }),
        }
    }

    /// Save to the resolved config path.
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = self.paths.resolved_config_path();
        self.save_to(&path)
    }

    /// Save to an explicit path.
    pub fn save_to(&self, path: &PathBuf) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| ConfigError::Io {
                path: path.clone(),
                source: e,
            })?;
        }
        let pretty = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::Json { path: path.clone(), source: e })?;
        fs::write(path, pretty).map_err(|e| ConfigError::Io {
            path: path.clone(),
            source: e,
        })?;
        Ok(())
    }

    /// Optional: fill missing `Option<PathBuf>` values based on defaults.
    /// Useful when you add new fields later and older configs don't include them.
    pub fn with_filled_paths(mut self) -> Self {
        let defaults = AppConfig::default();

        if self.paths.config_dir.is_none() {
            self.paths.config_dir = defaults.paths.config_dir;
        }
        if self.paths.style_dir.is_none() {
            self.paths.style_dir = defaults.paths.style_dir;
        }
        if self.paths.config_path.is_none() {
            self.paths.config_path = defaults.paths.config_path;
        }
        if self.paths.wallust_colors_json.is_none() {
            self.paths.wallust_colors_json = defaults.paths.wallust_colors_json;
        }

        // wallpaper_dir is non-optional; keep whatever is set in file.
        // But if you ever want to support empty, you could validate here.

        self
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io { path: PathBuf, source: std::io::Error },
    Json { path: PathBuf, source: serde_json::Error },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io { path, source } => {
                write!(f, "I/O error at {}: {}", path.display(), source)
            },
            ConfigError::Json { path, source } => {
                write!(f, "JSON error in {}: {}", path.display(), source)
            },
        }
    }
}

impl std::error::Error for ConfigError {}

/// Example: what the default config.json will look like
/// (You don't need this in your code; it's just here as reference.)
#[allow(dead_code)]
fn _example() {
    let cfg = AppConfig::default();
    println!("{}", serde_json::to_string_pretty(&cfg).unwrap());
}


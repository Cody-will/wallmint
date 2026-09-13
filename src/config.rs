use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub paths: PathsConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub hyprpaper: HyprpaperConfig,
    #[serde(default)]
    pub wallust: WallustRunConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    pub wallpaper_dir: PathBuf,
    #[serde(default)]
    pub config_dir: Option<PathBuf>,
    /// Directory that may contain style.css (legacy field).
    #[serde(default)]
    pub style_dir: Option<PathBuf>,
    /// Preferred: explicit path to style.css.
    #[serde(default)]
    pub style_path: Option<PathBuf>,
    #[serde(default)]
    pub config_path: Option<PathBuf>,
    #[serde(default)]
    pub wallust_colors_json: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default)]
    pub window: WindowConfig,
    #[serde(default)]
    pub preview: PreviewConfig,
}

fn default_title() -> String {
    "Wallmint".to_string()
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            title: default_title(),
            window: WindowConfig::default(),
            preview: PreviewConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    #[serde(default = "default_height")]
    pub height: i32,
    #[serde(default = "default_width")]
    pub width: i32,
    #[serde(default = "default_true")]
    pub resizable: bool,
}

fn default_height() -> i32 {
    720
}
fn default_width() -> i32 {
    1100
}
fn default_true() -> bool {
    true
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            height: default_height(),
            width: default_width(),
            resizable: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewConfig {
    #[serde(default = "default_aspect")]
    pub aspect_ratio: String,
    #[serde(default = "default_fit")]
    pub content_fit: String,
    #[serde(default = "default_true")]
    pub vexpand: bool,
    #[serde(default = "default_true")]
    pub hexpand: bool,
}

fn default_aspect() -> String {
    "16:9".to_string()
}
fn default_fit() -> String {
    "cover".to_string()
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: default_aspect(),
            content_fit: default_fit(),
            vexpand: true,
            hexpand: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyprpaperConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_hyprctl")]
    pub hyprctl_bin: String,
    /// Monitor name, or "*" for every monitor.
    #[serde(default = "default_monitor")]
    pub monitor: String,
    #[serde(default = "default_true")]
    pub preload: bool,
}

fn default_hyprctl() -> String {
    "hyprctl".to_string()
}
fn default_monitor() -> String {
    "*".to_string()
}

impl Default for HyprpaperConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hyprctl_bin: default_hyprctl(),
            monitor: default_monitor(),
            preload: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallustRunConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_wallust")]
    pub bin: String,
}

fn default_wallust() -> String {
    "wallust".to_string()
}

impl Default for WallustRunConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bin: default_wallust(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let cache_dir = dirs::cache_dir().unwrap_or_else(|| config_dir.join(".cache"));
        let pictures_dir = dirs::picture_dir().unwrap_or_else(|| PathBuf::from("."));

        let app_config_dir = config_dir.join("wallmint");
        let style_path = app_config_dir.join("style.css");
        let style_dir = app_config_dir.clone();
        let config_path = app_config_dir.join("config.json");
        let wallust_colors_json = cache_dir.join("wallust").join("colors.json");

        let walls = pictures_dir.join("walls");
        let wallpaper_dir = if walls.is_dir() {
            walls
        } else {
            pictures_dir.join("Wallpapers")
        };

        Self {
            paths: PathsConfig {
                wallpaper_dir,
                config_dir: Some(app_config_dir),
                style_dir: Some(style_dir),
                style_path: Some(style_path),
                config_path: Some(config_path),
                wallust_colors_json: Some(wallust_colors_json),
            },
            ui: UiConfig::default(),
            hyprpaper: HyprpaperConfig::default(),
            wallust: WallustRunConfig::default(),
        }
    }
}

impl PathsConfig {
    pub fn resolved_config_path(&self) -> PathBuf {
        if let Some(p) = &self.config_path {
            return expand_tilde(p);
        }

        let base = self
            .config_dir
            .clone()
            .or_else(dirs::config_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wallmint");

        expand_tilde(&base.join("config.json"))
    }

    pub fn resolved_style_path(&self) -> Option<PathBuf> {
        if let Some(p) = &self.style_path {
            return Some(expand_tilde(p));
        }
        if let Some(dir) = &self.style_dir {
            let candidate = expand_tilde(dir);
            if candidate.is_file() {
                return Some(candidate);
            }
            return Some(candidate.join("style.css"));
        }
        None
    }

    pub fn resolved_wallpaper_dir(&self) -> PathBuf {
        expand_tilde(&self.wallpaper_dir)
    }

    pub fn resolved_wallust_colors(&self) -> Option<PathBuf> {
        self.wallust_colors_json.as_ref().map(expand_tilde)
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        if let Some(dir) = &self.config_dir {
            fs::create_dir_all(expand_tilde(dir))?;
        }
        if let Some(dir) = &self.style_dir {
            let p = expand_tilde(dir);
            if p.extension().and_then(|e| e.to_str()) != Some("css") {
                fs::create_dir_all(&p)?;
            } else if let Some(parent) = p.parent() {
                fs::create_dir_all(parent)?;
            }
        }
        if let Some(p) = &self.style_path {
            if let Some(parent) = expand_tilde(p).parent() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }
}

impl AppConfig {
    pub fn load_or_create() -> Result<Self, ConfigError> {
        let defaults = AppConfig::default();
        let path = defaults.paths.resolved_config_path();

        match fs::read_to_string(&path) {
            Ok(raw) => {
                let parsed: AppConfig = serde_json::from_str(&raw).map_err(|e| ConfigError::Json {
                    path: path.clone(),
                    source: e,
                })?;
                let cfg = parsed.with_filled_paths().expanded();
                cfg.install_bundled_style_if_missing();
                Ok(cfg)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                defaults.paths.ensure_dirs().map_err(|e| ConfigError::Io {
                    path: path.clone(),
                    source: e,
                })?;
                defaults.save_to(&path)?;
                defaults.install_bundled_style_if_missing();
                Ok(defaults.expanded())
            }
            Err(e) => Err(ConfigError::Io { path, source: e }),
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = self.paths.resolved_config_path();
        self.save_to(&path)
    }

    pub fn save_to(&self, path: &PathBuf) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| ConfigError::Io {
                path: path.clone(),
                source: e,
            })?;
        }
        let pretty = serde_json::to_string_pretty(self).map_err(|e| ConfigError::Json {
            path: path.clone(),
            source: e,
        })?;
        fs::write(path, pretty).map_err(|e| ConfigError::Io {
            path: path.clone(),
            source: e,
        })?;
        Ok(())
    }

    pub fn with_filled_paths(mut self) -> Self {
        let defaults = AppConfig::default();

        if self.paths.config_dir.is_none() {
            self.paths.config_dir = defaults.paths.config_dir;
        }
        if self.paths.style_dir.is_none() {
            self.paths.style_dir = defaults.paths.style_dir;
        }
        if self.paths.style_path.is_none() {
            self.paths.style_path = defaults.paths.style_path;
        }
        if self.paths.config_path.is_none() {
            self.paths.config_path = defaults.paths.config_path;
        }
        if self.paths.wallust_colors_json.is_none() {
            self.paths.wallust_colors_json = defaults.paths.wallust_colors_json;
        }
        self
    }

    pub fn expanded(mut self) -> Self {
        self.paths.wallpaper_dir = expand_tilde(&self.paths.wallpaper_dir);
        self.paths.config_dir = self.paths.config_dir.map(|p| expand_tilde(&p));
        self.paths.style_dir = self.paths.style_dir.map(|p| expand_tilde(&p));
        self.paths.style_path = self.paths.style_path.map(|p| expand_tilde(&p));
        self.paths.config_path = self.paths.config_path.map(|p| expand_tilde(&p));
        self.paths.wallust_colors_json = self.paths.wallust_colors_json.map(|p| expand_tilde(&p));
        self
    }

    fn install_bundled_style_if_missing(&self) {
        if let Some(path) = self.paths.resolved_style_path() {
            if !path.exists() {
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(&path, include_str!("../assets/style.css"));
            }
        }
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
            }
            ConfigError::Json { path, source } => {
                write!(f, "JSON error in {}: {}", path.display(), source)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn expand_tilde(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    let raw = path.to_string_lossy();
    if raw == "~" {
        return dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    }
    if let Some(rest) = raw.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    path.to_path_buf()
}

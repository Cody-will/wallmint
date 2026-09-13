use crate::config::AppConfig;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

/// Set the wallpaper with hyprpaper and optionally run wallust.
pub fn apply(path: &Path, cfg: &AppConfig) -> Result<(), String> {
    let abs = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let path_str = abs.to_string_lossy().to_string();

    if cfg.hyprpaper.enabled {
        set_hyprpaper(&path_str, cfg)?;
    }

    if cfg.wallust.enabled {
        run_wallust(&path_str, cfg)?;
    }

    Ok(())
}

fn set_hyprpaper(path_str: &str, cfg: &AppConfig) -> Result<(), String> {
    let bin = cfg.hyprpaper.hyprctl_bin.as_str();
    let fit = match cfg.ui.preview.content_fit.to_ascii_lowercase().as_str() {
        "contain" => "contain",
        "fill" => "fill",
        _ => "cover",
    };

    // New hyprpaper dropped `preload`. Only try it when the user asked;
    // ignore "invalid hyprpaper request" so the set still happens.
    if cfg.hyprpaper.preload {
        let _ = run_logged(
            bin,
            &["hyprpaper", "preload", path_str],
            "hyprpaper preload",
        );
    }

    let monitors = resolve_monitors(&cfg.hyprpaper.monitor, bin);
    let mut last_err = None;

    for mon in monitors {
        // Current IPC: hyprctl hyprpaper wallpaper "MON,/abs/path,fit"
        // Empty MON is the fallback wallpaper.
        let arg = if mon.is_empty() {
            format!(",{path_str},{fit}")
        } else {
            format!("{mon},{path_str},{fit}")
        };

        match run_logged(bin, &["hyprpaper", "wallpaper", &arg], "hyprpaper wallpaper") {
            Ok(()) => return Ok(()),
            Err(e) => last_err = Some(e),
        }
    }

    Err(last_err.unwrap_or_else(|| "hyprpaper wallpaper failed".into()))
}

fn resolve_monitors(configured: &str, hyprctl: &str) -> Vec<String> {
    let configured = configured.trim();
    if configured.is_empty() {
        return vec![String::new()];
    }
    if configured != "*" {
        return vec![configured.to_string()];
    }

    let mut names = list_hypr_monitors(hyprctl);
    // Also try the empty-monitor fallback used by new hyprpaper.
    names.push(String::new());
    if names.is_empty() {
        names.push(String::new());
    }
    names
}

fn list_hypr_monitors(hyprctl: &str) -> Vec<String> {
    let output = Command::new(hyprctl).args(["-j", "monitors"]).output().ok();
    let Some(output) = output else {
        return Vec::new();
    };
    let Ok(v) = serde_json::from_slice::<Value>(&output.stdout) else {
        return Vec::new();
    };
    v.as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn run_wallust(path_str: &str, cfg: &AppConfig) -> Result<(), String> {
    run_logged(cfg.wallust.bin.as_str(), &["run", path_str], "wallust run")
}

fn run_logged(bin: &str, args: &[&str], label: &str) -> Result<(), String> {
    eprintln!("$ {bin} {}", args.join(" "));
    let output = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| format!("failed to start {label} ({bin}): {e}"))?;

    if !output.stdout.is_empty() {
        eprintln!("{label} stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("{label} stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    if !output.status.success() {
        return Err(format!(
            "{label} exited with {}: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stdout).trim()
        ));
    }
    Ok(())
}

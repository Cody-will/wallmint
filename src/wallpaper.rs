use crate::config::AppConfig;
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
    let monitor = if cfg.hyprpaper.monitor.trim().is_empty() {
        "*"
    } else {
        cfg.hyprpaper.monitor.as_str()
    };

    if cfg.hyprpaper.preload {
        run_logged(
            bin,
            &["hyprpaper", "preload", path_str],
            "hyprpaper preload",
        )?;
    }

    let wallpaper_arg = format!("{monitor},{path_str}");
    run_logged(
        bin,
        &["hyprpaper", "wallpaper", &wallpaper_arg],
        "hyprpaper wallpaper",
    )?;

    Ok(())
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
            "{label} exited with {}",
            output.status.code().unwrap_or(-1)
        ));
    }
    Ok(())
}

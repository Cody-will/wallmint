
use std::process::Command;
use serde_json::Value;

pub fn get_active_opacity() -> Option<f32> {
    let output = Command::new("hyprctl")
        .args(["-j", "getoption", "decoration:active_opacity"])
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!(
            "hyprctl failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    let v: Value = serde_json::from_str(&text).ok()?;

    if let Some(f) = v.get("float").and_then(|x| x.as_f64()) {
        return Some(f as f32);
    }

    if let Some(f) = v.get("value").and_then(|vv| vv.get("float")).and_then(|x| x.as_f64()) {
        return Some(f as f32);
    }

    eprintln!("hyprctl JSON did not contain float: {text}");
    None
}


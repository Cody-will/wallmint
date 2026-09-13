# Wallmint

GTK4 wallpaper picker for Hyprland.

Built with Rust + GTK4. Sets the wallpaper through **hyprpaper** and regenerates colors with **wallust**.

## Requirements (Arch)

```bash
sudo pacman -S gtk4 gtk4-layer-shell hyprpaper wallust
# rustup / rust package if cargo is missing
```

You also need a wallpaper folder. By default Wallmint looks at:

- `~/Pictures/walls` if that directory exists
- otherwise `~/Pictures/Wallpapers`

## Build and run

```bash
git clone https://github.com/Cody-will/wallmint.git
cd wallmint
cargo run --release
```

First launch writes `~/.config/wallmint/config.json` and copies `style.css` there.

## Keys

| Key | Action |
| --- | --- |
| `Left` / `h` | Previous wallpaper |
| `Right` / `l` | Next wallpaper |
| `s` | Set wallpaper (hyprpaper + wallust) |
| `Esc` / `Ctrl+Q` | Quit |

## Config

`~/.config/wallmint/config.json`

Useful fields:

```json
{
  "paths": {
    "wallpaper_dir": "/home/YOU/Pictures/walls",
    "style_path": "/home/YOU/.config/wallmint/style.css",
    "wallust_colors_json": "/home/YOU/.cache/wallust/colors.json"
  },
  "hyprpaper": {
    "enabled": true,
    "monitor": "*"
  },
  "wallust": {
    "enabled": true
  }
}
```

`monitor` can be a Hyprland output name (`eDP-1`, `DP-1`) or `*` for every monitor.

`~` in paths is expanded.

## Hyprland hint

Bind it however you like:

```
bind = SUPER, W, exec, wallmint
```

After `cargo install --path .` the binary lands in `~/.cargo/bin/wallmint`.

use crate::config::AppConfig;
use crate::theme::load_theme;
use gtk::gdk::Display;
use gtk::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub struct AppState {
    pub cfg: Arc<AppConfig>,
}

pub fn run(cfg: AppConfig) {
    let state = Arc::new(AppState {
        cfg: Arc::new(cfg),
    });

    let app = gtk::Application::builder()
        .application_id("dev.wallmint")
        .build();

    let quit = gtk::gio::SimpleAction::new("quit", None);
    let app_for_cb = app.clone();
    quit.connect_activate(move |_, _| {
        app_for_cb.quit();
    });
    app.add_action(&quit);
    app.set_accels_for_action("app.quit", &["Escape", "<Ctrl>Q"]);

    let state_for_activate = state.clone();
    app.connect_activate(move |app| {
        let theme = load_theme(&state_for_activate.cfg);
        load_css(&theme, Some(&state_for_activate.cfg));
        crate::ui::build_ui(app, state_for_activate.cfg.clone());
    });

    app.run();
}

/// Load bundled + optional user CSS, then overlay theme colors from wallust.
pub fn load_css(theme: &crate::theme::WallmintTheme, cfg: Option<&AppConfig>) {
    let Some(display) = Display::default() else {
        eprintln!("No display found; skipping CSS load");
        return;
    };

    let bundled = include_str!("../assets/style.css");
    let user_css = cfg
        .and_then(|c| c.paths.resolved_style_path())
        .and_then(|p| std::fs::read_to_string(p).ok())
        .filter(|s| !s.trim().is_empty());

    let base_css = user_css.unwrap_or_else(|| bundled.to_string());

    let base = gtk::CssProvider::new();
    base.load_from_data(&base_css);
    gtk::style_context_add_provider_for_display(
        &display,
        &base,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let generated = gtk::CssProvider::new();
    let vars = format!(
        r#"
        @define-color background {bg};
        @define-color foreground {fg};
        @define-color accent {accent};
        @define-color secondary {secondary};

        window {{
            background-color: alpha(@background, {alpha});
        }}
        "#,
        bg = theme.background,
        fg = theme.foreground,
        accent = theme.accent,
        secondary = theme.secondary,
        alpha = theme.background_alpha.clamp(0.15, 1.0),
    );

    generated.load_from_data(vars.as_str());
    gtk::style_context_add_provider_for_display(
        &display,
        &generated,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1,
    );
}

pub fn reload_theme_css(cfg: &AppConfig) {
    let theme = load_theme(cfg);
    load_css(&theme, Some(cfg));
}

#[allow(dead_code)]
pub fn style_exists(path: &Path) -> bool {
    path.is_file()
}

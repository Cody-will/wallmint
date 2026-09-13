use gtk;
use gtk::prelude::*;
use gtk::gdk::Display;
use crate::config::AppConfig;
use std::sync::Arc;
use crate::theme::load_theme;


pub struct AppState {
    pub cfg: Arc<AppConfig>,
}

pub fn run(cfg: AppConfig) {
    let state = Arc::new(AppState {cfg: Arc::new(cfg) });

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

    app.connect_activate(|app| {
        let theme = load_theme(&state.cfg);
        load_css(&theme, state.clone());
        crate::ui::build_ui(app, state.cfg.clone());
    });
    app.run();
}


pub fn load_css(theme: &crate::theme::WallmintTheme, state: Arc<AppState>) {
    let display = Display::default().expect("No display found");

    let base = gtk::CssProvider::new();
    let path = format!("{:?}", &state.cfg.paths.style_dir);
    base.load_from_data(&path);
    gtk::style_context_add_provider_for_display(
        &display,
        &base,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let generated = gtk::CssProvider::new();
    let vars = format!(
        r#"
        @define-color background {};
        @define-color foreground {};
        @define-color accent {};
        @define-color secondary {};

         :root {{
            --bg-opacity: {};
         }}
        "#,
        theme.background,
        theme.foreground,
        theme.accent,
        theme.secondary,
        theme.background_alpha,
    );

    generated.load_from_data(vars.as_str());
    gtk::style_context_add_provider_for_display(
        &display,
        &generated,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 1, 
    );
}




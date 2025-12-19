use gtk;
use gtk::prelude::*;
use gtk::gdk::Display;
use crate::config::load_theme;

pub fn run() {
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
        let theme = load_theme();
        load_css(&theme);
        crate::ui::build_ui(app, &theme);
    });
    app.run();
}


pub fn load_css(theme: &crate::config::WallmintTheme) {
    let display = Display::default().expect("No display found");

    let base = gtk::CssProvider::new();
    base.load_from_path("assets/style.css");
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




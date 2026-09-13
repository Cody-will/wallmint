use crate::carousel::create_carousel;
use crate::config::AppConfig;
use crate::keys::register_carousel_keys;
use gtk::prelude::*;
use gtk4_layer_shell::LayerShell;
use std::sync::Arc;

pub fn build_ui(app: &gtk::Application, cfg: Arc<AppConfig>) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(&cfg.ui.title)
        .default_width(cfg.ui.window.width)
        .default_height(cfg.ui.window.height)
        .resizable(cfg.ui.window.resizable)
        .build();

    window.init_layer_shell();
    window.set_namespace(Some("wallmint"));
    window.set_layer(gtk4_layer_shell::Layer::Overlay);
    // Exclusive so Left/Right/s work as soon as the overlay is shown.
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

    let root = gtk::Box::new(gtk::Orientation::Vertical, 8);
    root.add_css_class("root");

    let wallpaper_dir = cfg.paths.resolved_wallpaper_dir();
    let imgs = crate::carousel::load_images_from_folder(&wallpaper_dir);

    if imgs.is_empty() {
        let msg = gtk::Label::new(Some(&format!(
            "No images found in {}\nAdd png/jpg/webp files there, then restart Wallmint.",
            wallpaper_dir.display()
        )));
        msg.set_wrap(true);
        msg.add_css_class("photo-label");
        root.append(&msg);
        window.set_child(Some(&root));
        window.present();
        return;
    }

    let (carousel_widget, carousel_handle) = create_carousel(imgs, cfg.clone());
    root.append(&carousel_widget);

    window.set_child(Some(&root));
    register_carousel_keys(app, &window, carousel_handle);
    window.present();
}

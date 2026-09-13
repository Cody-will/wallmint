use gtk;
use gtk::prelude::*;
use gtk4_layer_shell::LayerShell;
use crate::carousel::create_carousel;
use crate::keys::register_carousel_keys;
use crate::config::AppConfig;
use std::sync::Arc;


pub fn build_ui(app: &gtk::Application, cfg: Arc<AppConfig>) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title(&cfg.ui.title)
        .default_width(cfg.ui.window.width)
        .default_height(cfg.ui.window.height)
        .build();

    window.init_layer_shell();
    window.set_layer(gtk4_layer_shell::Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

    let root = gtk::Box::new(gtk::Orientation::Vertical, 8);

    let photo_box = create_photo_box();
    root.append(&photo_box);

    let imgs = crate::carousel::load_images_from_folder(&cfg.paths.wallpaper_dir);
    let (carousel_widget, carousel_handle) = create_carousel(imgs);

    root.append(&carousel_widget);

    window.set_child(Some(&root));

    register_carousel_keys(app, &window, carousel_handle);

    window.present();
}




fn create_photo_box() -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 4);
    container.add_css_class("preview-container");

    let preview = gtk::DrawingArea::builder()
        .content_width(160)
        .content_height(90)
        .build();

    let label = gtk::Label::new(Some("Wallpaper"));
    label.add_css_class("photo-label");
    container.append(&preview);
    container.append(&label);

    container
}




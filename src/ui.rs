use gtk;
use gtk::prelude::*;
use gtk4_layer_shell::LayerShell;
use crate::carousel::create_carousel;
use crate::keys::register_carousel_keys;

pub fn build_ui(app: &gtk::Application, _theme: &crate::config::WallmintTheme) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Wallmint")
        .default_width(900)
        .default_height(600)
        .build();
    window.init_layer_shell();

    window.set_layer(gtk4_layer_shell::Layer::Overlay);

    window.set_anchor(gtk4_layer_shell::Edge::Left, false);
    window.set_anchor(gtk4_layer_shell::Edge::Right, false);
    window.set_anchor(gtk4_layer_shell::Edge::Top, false);
    window.set_anchor(gtk4_layer_shell::Edge::Bottom, false);

    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

    window.set_margin(gtk4_layer_shell::Edge::Top, 0);
    window.set_margin(gtk4_layer_shell::Edge::Bottom, 0);
    window.set_margin(gtk4_layer_shell::Edge::Left, 0);
    window.set_margin(gtk4_layer_shell::Edge::Right, 0);
    
    let root = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let photo_box = create_photo_box();
    
    root.append(&photo_box);
    window.set_child(Some(&root));

    let img = crate::carousel::load_images_from_folder("/home/cody/Pictures/walls");
    let (carousel_widget, carousel_handle) = create_carousel(img);

    window.set_child(Some(&carousel_widget));
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




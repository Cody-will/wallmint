
use gtk4 as gtk;
use gtk::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn is_image(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase())
            .as_deref(),
        Some("png") | Some("jpg") | Some("jpeg") | Some("webp") | Some("bmp")
    )
}

pub fn load_images_from_folder(folder: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(read) = fs::read_dir(folder) else { return out; };

    for entry in read.flatten() {
        let path = entry.path();
        if path.is_file() && is_image(&path) {
            out.push(path);
        }
    }

    out.sort();
    out
}

pub fn create_carousel(image_paths: Vec<PathBuf>) -> gtk::Widget {
    let strip = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    strip.add_css_class("carousel-container");
    strip.set_hexpand(true);
    strip.set_vexpand(false);

    for path in image_paths {
        let pic = gtk::Picture::new();
        pic.set_size_request(160, 90);
        pic.add_css_class("thumb");

        let file = gtk::gio::File::for_path(&path);
        match gtk::gdk::Texture::from_file(&file) {
            Ok(tex) => pic.set_paintable(Some(&tex)),
            Err(_) => pic.set_paintable(None::<&gtk::gdk::Paintable>),
        }

        strip.append(&pic);
    }

    let scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Never)
        .child(&strip)
        .build();

    scroller.set_propagate_natural_width(true);
    scroller.set_propagate_natural_height(true);

    scroller.upcast::<gtk::Widget>()
}


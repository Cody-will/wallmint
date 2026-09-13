use crate::config::AppConfig;
use gtk::prelude::*;
use std::sync::Arc;
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

fn is_image(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase())
            .as_deref(),
        Some("png") | Some("jpg") | Some("jpeg") | Some("webp") | Some("bmp") | Some("jxl")
    )
}

pub fn load_images_from_folder(folder: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let folder = folder.as_ref();
    let Ok(read) = fs::read_dir(folder) else {
        eprintln!("Could not read wallpaper dir: {}", folder.display());
        return out;
    };

    for entry in read.flatten() {
        let path = entry.path();
        if path.is_file() && is_image(&path) {
            out.push(path);
        }
    }

    out.sort();
    out
}

pub type CarouselHandle = Rc<RefCell<Carousel>>;

pub struct Carousel {
    all: Vec<gtk::gdk::Texture>,
    selected: usize,
    thumbs: [gtk::Picture; 5],
    preview: gtk::Picture,
    selected_path: PathBuf,
    all_path: Vec<PathBuf>,
    cfg: Arc<AppConfig>,
}

impl Carousel {
    fn len(&self) -> usize {
        self.all.len()
    }

    pub fn prev(&mut self) {
        if self.len() == 0 {
            return;
        }
        self.selected = (self.selected + self.len() - 1) % self.len();
        self.refresh();
    }

    pub fn next(&mut self) {
        if self.len() == 0 {
            return;
        }
        self.selected = (self.selected + 1) % self.len();
        self.refresh();
    }

    fn visible_indices(&self) -> [usize; 5] {
        let n = self.len().max(1);
        let s = self.selected % n;
        [
            (s + n - 2) % n,
            (s + n - 1) % n,
            s,
            (s + 1) % n,
            (s + 2) % n,
        ]
    }

    fn set_selected_path(&mut self) {
        if self.all_path.is_empty() {
            return;
        }
        self.selected_path = self.all_path[self.selected].clone();
        println!("path: {}", self.selected_path.display());
    }

    fn refresh(&mut self) {
        if self.len() == 0 {
            return;
        }

        let idx = self.visible_indices();

        for (slot, &i) in self.thumbs.iter().zip(idx.iter()) {
            slot.set_paintable(Some(&self.all[i]));
        }
        self.set_selected_path();
        self.preview.set_paintable(Some(&self.all[self.selected]));
    }

    pub fn set_paper(&self) {
        if self.all_path.is_empty() {
            eprintln!("No wallpapers loaded; nothing to set");
            return;
        }

        match crate::wallpaper::apply(&self.selected_path, &self.cfg) {
            Ok(()) => {
                crate::app::reload_theme_css(&self.cfg);
            }
            Err(e) => eprintln!("Failed to set wallpaper: {e}"),
        }
    }
}

pub fn create_carousel(
    image_paths: Vec<PathBuf>,
    cfg: Arc<AppConfig>,
) -> (gtk::Widget, CarouselHandle) {
    let mut all = Vec::new();
    let mut all_path = Vec::new();

    for path in image_paths {
        let file = gtk::gio::File::for_path(&path);
        match gtk::gdk::Texture::from_file(&file) {
            Ok(tex) => {
                all_path.push(path);
                all.push(tex);
            }
            Err(e) => eprintln!("Skipping {}: {e}", path.display()),
        }
    }

    let preview = gtk::Picture::new();
    preview.set_can_shrink(true);
    preview.set_content_fit(parse_content_fit(&cfg.ui.preview.content_fit));
    preview.set_vexpand(cfg.ui.preview.vexpand);
    preview.set_hexpand(cfg.ui.preview.hexpand);
    preview.add_css_class("preview");

    let thumb_row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    thumb_row.add_css_class("thumb-row");
    thumb_row.set_hexpand(true);

    let thumbs: [gtk::Picture; 5] = std::array::from_fn(|i| {
        let p = gtk::Picture::new();
        p.set_size_request(160, 90);
        p.set_content_fit(gtk::ContentFit::Cover);
        p.set_vexpand(true);
        p.set_hexpand(true);
        p.set_can_shrink(true);
        if i == 2 {
            p.add_css_class("thumb-active");
        } else {
            p.add_css_class("thumb");
        }
        thumb_row.append(&p);
        p
    });

    let root = gtk::Box::new(gtk::Orientation::Vertical, 12);
    root.append(&preview);
    root.append(&thumb_row);

    let selected_path = all_path.first().cloned().unwrap_or_default();

    let carousel = Rc::new(RefCell::new(Carousel {
        all,
        selected: 0,
        thumbs,
        preview,
        all_path,
        selected_path,
        cfg,
    }));

    carousel.borrow_mut().refresh();

    (root.upcast::<gtk::Widget>(), carousel)
}

fn parse_content_fit(s: &str) -> gtk::ContentFit {
    match s.to_ascii_lowercase().as_str() {
        "contain" => gtk::ContentFit::Contain,
        "fill" => gtk::ContentFit::Fill,
        "scale-down" | "scaledown" => gtk::ContentFit::ScaleDown,
        _ => gtk::ContentFit::Cover,
    }
}


use gtk;
use gtk::prelude::*;
use std::{cell::RefCell, path::Path, fs, path::PathBuf, rc::Rc};
use std::process::Command;
use crate::app::load_css;
use crate::theme::load_theme;

fn is_image(path: &Path) -> bool {
    return matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase())
            .as_deref(),
        Some("png") | Some("jpg") | Some("jpeg") | Some("webp") | Some("bmp")
    );
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

pub type CarouselHandle = Rc<RefCell<Carousel>>;

pub struct Carousel {
    all: Vec<gtk::gdk::Texture>,
    selected: usize,
    thumbs: [gtk::Picture; 5],
    preview: gtk::Picture,
    selected_path: PathBuf,
    all_path: Vec<PathBuf>,
}

impl Carousel {
    fn len(&self) -> usize { self.all.len() }

    pub fn prev(&mut self) {
        if self.len() == 0 { return; }
        self.selected = (self.selected + self.len() - 1) % self.len();
        self.refresh();
    }

    pub fn next(&mut self) {
        if self.len() == 0 { return; }
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
        self.selected_path = self.all_path[self.selected].clone();
        println!("path: {}", self.selected_path.display());
    }

    fn refresh(&mut self) {
        if self.len() == 0 { return; }

        let idx = self.visible_indices();

        for (slot, &i) in self.thumbs.iter().zip(idx.iter()) {
            slot.set_paintable(Some(&self.all[i]));
        }
        self.set_selected_path();

        self.preview.set_paintable(Some(&self.all[self.selected]));
    }

 

    pub fn set_paper(&self) {
        let path = std::fs::canonicalize(&self.selected_path)
            .unwrap_or_else(|_| self.selected_path.clone());
        let path_str = path.to_string_lossy(); 
        let wallpaper_arg = format!("wallpaper,eDP-1,{}", path_str);
        let preload_arg = format!("preload,{}", path_str);

        println!("{}", path_str); 

        // Send single comma-separated argument for preload
        let preload = Command::new("hyprctl")
            .args(["hyprpaper", &preload_arg])
            .output()
            .expect("failed to execute process");

        eprintln!("preload stdout: {}", String::from_utf8_lossy(&preload.stdout));
        eprintln!("preload stderr: {}", String::from_utf8_lossy(&preload.stderr));

        // Send single comma-separated argument for wallpaper
        let set = Command::new("hyprctl")
            .args(["hyprpaper", &wallpaper_arg])
            .output()
            .expect("failed to execute process");

        eprintln!("set stdout: {}", String::from_utf8_lossy(&set.stdout));
        eprintln!("set stderr: {}", String::from_utf8_lossy(&set.stderr));

        let colors = Command::new("wallust")
            .args(["run", &path_str])
            .output()
            .expect("Color change failed");

        eprintln!("colors: stdout: {}", String::from_utf8_lossy(&colors.stdout));
        eprintln!("colors: stderr: {}", String::from_utf8_lossy(&colors.stderr));
            
        let theme = load_theme(&self);
        load_css(&theme);
    }
}

pub fn create_carousel(image_paths: Vec<PathBuf>) -> (gtk::Widget, CarouselHandle) {
    let mut all = Vec::new();
    let mut all_path = Vec::new();
    
    for path in image_paths {
        all_path.push(path.clone());
        let file = gtk::gio::File::for_path(&path);
        if let Ok(tex) = gtk::gdk::Texture::from_file(&file) {
            all.push(tex);
        }
    }

    // 2) Build widgets
    let preview = gtk::Picture::new();
    preview.set_can_shrink(true);
    preview.set_content_fit(gtk::ContentFit::Cover);
    preview.set_vexpand(true);
    preview.set_hexpand(true);
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
    let selected_path = all_path[0].clone();

    // 3) Create state + initial render
    let carousel = Rc::new(RefCell::new(Carousel {
        all,
        selected: 0,
        thumbs,
        preview,
        all_path,
        selected_path,
    }));

    carousel.borrow_mut().refresh();

    (root.upcast::<gtk::Widget>(), carousel)
}



use gtk;
use gtk::prelude::*;

pub fn register_carousel_keys(app: &gtk::Application, window: &gtk::ApplicationWindow, carousel: crate::carousel::CarouselHandle) {
    // prev
    let prev = gtk::gio::SimpleAction::new("carousel_prev", None);
    {
        let carousel = carousel.clone();
        prev.connect_activate(move |_, _| carousel.borrow_mut().prev());
    }
    window.add_action(&prev);
    app.set_accels_for_action("win.carousel_prev", &["Left", "h"]);

    // next
    let next = gtk::gio::SimpleAction::new("carousel_next", None);
    {
        let carousel = carousel.clone();
        next.connect_activate(move |_, _| carousel.borrow_mut().next());
    }
    window.add_action(&next);
    app.set_accels_for_action("win.carousel_next", &["Right", "l"]);

    let set_wall = gtk::gio::SimpleAction::new("carousel_set_paper", None);
    {
        let carousel = carousel.clone();
        set_wall.connect_activate(move |_, _| carousel.borrow().set_paper());
    }
    window.add_action(&set_wall);
    app.set_accels_for_action("win.carousel_set_paper", &["s"]);
}

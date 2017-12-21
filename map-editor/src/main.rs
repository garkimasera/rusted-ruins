
extern crate rusted_ruins_common as common;

extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate cairo;

mod edit_map;
mod ui;
mod draw_map;

use gio::prelude::*;

pub fn main() {
    let application = gtk::Application::new("com.github.rusted-ruins-map-editor",
                                            gio::ApplicationFlags::empty())
        .expect("Initialization failed.");

    application.connect_startup(move |app| {
        ui::build_ui(app);
    });
    application.connect_activate(|_| {});
    use std::env::args;
    application.run(&args().collect::<Vec<_>>());
}


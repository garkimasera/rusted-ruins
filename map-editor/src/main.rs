extern crate rusted_ruins_common as common;
extern crate rusted_ruins_geom as geom;

mod edit_map;
#[macro_use]
mod ui;
mod draw_map;
mod file;
mod iconview;
mod pixbuf_holder;
mod property_controls;

use gio::prelude::*;
use std::env;
use std::path::PathBuf;

pub fn main() {
    let application = gtk::Application::new(
        "com.github.rusted-ruins-map-editor",
        gio::ApplicationFlags::empty(),
    )
    .expect("Initialization failed.");

    let mut app_dir = get_app_dir().expect("Could not found application directory");
    app_dir.push("paks");
    let mut pak_dirs = vec![app_dir];
    for mut addon_dir in get_addon_dir().into_iter() {
        addon_dir.push("paks");
        pak_dirs.push(addon_dir);
    }

    common::gobj::init(pak_dirs);
    application.connect_startup(move |app| {
        ui::build_ui(app);
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}

/// Get application directory
fn get_app_dir() -> Option<PathBuf> {
    if let Some(e) = env::var_os("RUSTED_RUINS_APP_DIR") {
        return Some(PathBuf::from(e));
    }

    if let Ok(mut exe_file) = env::current_exe() {
        exe_file.pop();
        exe_file.push("data");
        return Some(exe_file);
    }

    if let Ok(mut cdir) = env::current_dir() {
        cdir.push("data");
        return Some(cdir);
    }
    None
}

/// Get addon directories
fn get_addon_dir() -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Some(e) = env::var_os("RUSTED_RUINS_ADDON_DIR") {
        v.push(PathBuf::from(e));
    }
    v
}


use gtk;
use gtk::prelude::*;

#[derive(Clone)]
pub struct Ui {
    pub window: gtk::ApplicationWindow,
    pub map_drawing_area: gtk::DrawingArea,
}

macro_rules! get_object {
    ($builder:expr, $id:expr) => {
        if let Some(object) = $builder.get_object($id) {
            object
        } else {
            panic!("Builder Error: \"{}\" is not found", $id)
        }
    }
}

pub fn build_ui(application: &gtk::Application) {
    // Get widgets from glade file
    let builder = gtk::Builder::new_from_string(include_str!("ui.glade"));

    let ui = Ui {
        window:           get_object!(builder, "window1"),
        map_drawing_area: get_object!(builder, "map-drawing-area"),
    };

    let menu_quit: gtk::MenuItem = get_object!(builder, "menu-quit");

    ui.window.set_application(application);
    // Connect signals
    {
        let uic = ui.clone();
        ui.window.connect_delete_event(move |_, _| {
            uic.window.destroy();
            Inhibit(false)
        });
    }
    {
        ui.map_drawing_area.connect_draw(move |widget, context| {
            let width = widget.get_allocated_width();
            let height = widget.get_allocated_height();
            ::draw_map::draw_map(context, width, height);
            Inhibit(false)
        });
    }
    {
        let uic = ui.clone();
        menu_quit.connect_activate(move |_| {
            uic.window.destroy();
        });
    }

    ui.window.show_all();
}


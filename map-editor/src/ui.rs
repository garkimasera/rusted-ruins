
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use gdk;
use gtk;
use gtk::prelude::*;
use array2d::Vec2d;
use common::objholder::*;
use pixbuf_holder::PixbufHolder;
use edit_map::EditingMap;

const WRITE_BUTTON: u32 = 1;

#[derive(Clone)]
pub struct Ui {
    pub window: gtk::ApplicationWindow,
    pub map_drawing_area: gtk::DrawingArea,
    pub new_map_dialog: gtk::Dialog,
    pub iconview_tile:  gtk::IconView,
    pub liststore_tile: gtk::ListStore,
    pub adjustment_new_map_width: gtk::Adjustment,
    pub adjustment_new_map_height: gtk::Adjustment,
    pub adjustment_map_pos_x: gtk::Adjustment,
    pub adjustment_map_pos_y: gtk::Adjustment,
    pub pbh: Rc<PixbufHolder>,
    pub map: Rc<RefCell<EditingMap>>,
    pub selected_item: Rc<Cell<SelectedItem>>,
    pub on_drag: Rc<Cell<bool>>,
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
        new_map_dialog:   get_object!(builder, "new-map-dialog"),
        iconview_tile:    get_object!(builder, "iconview-tile"),
        liststore_tile:   get_object!(builder, "liststore-tile"),
        adjustment_new_map_width:  get_object!(builder, "adjustment-new-map-width"),
        adjustment_new_map_height: get_object!(builder, "adjustment-new-map-height"),
        adjustment_map_pos_x:      get_object!(builder, "adjustment-map-pos-x"),
        adjustment_map_pos_y:      get_object!(builder, "adjustment-map-pos-y"),
        pbh: Rc::new(PixbufHolder::new()),
        map: Rc::new(RefCell::new(EditingMap::new(16, 16))),
        selected_item: Rc::new(Cell::new(SelectedItem::Tile(TileIdx(0)))),
        on_drag: Rc::new(Cell::new(false)),
    };

    let menu_new:  gtk::MenuItem = get_object!(builder, "menu-new");
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
    { // Map drawing area (draw)
        let uic = ui.clone();
        ui.map_drawing_area.connect_draw(move |widget, context| {
            let width = widget.get_allocated_width();
            let height = widget.get_allocated_height();
            let map = uic.map.borrow();
            let pos = uic.get_map_pos();
            ::draw_map::draw_map(context, &*map, &*uic.pbh, width, height, pos);
            Inhibit(false)
        });
    }
    { // Map drawing area (button pressed)
        let uic = ui.clone();
        use gdk::EventMask;
        let mask = EventMask::BUTTON_PRESS_MASK | EventMask::BUTTON_RELEASE_MASK
            | EventMask::POINTER_MOTION_MASK;
        ui.map_drawing_area.add_events(mask.bits() as i32);
        ui.map_drawing_area.connect_button_press_event(move |_, eb| {
            on_map_clicked(&uic, eb);
            Inhibit(false)
        });
    }
    { // Map drawing area (button pressed)
        let uic = ui.clone();
        ui.map_drawing_area.connect_button_release_event(move |_, _| {
            uic.on_drag.set(false);
            Inhibit(false)
        });
    }
    { // Map drawing area (motion)
        let uic = ui.clone();
        ui.map_drawing_area.connect_motion_notify_event(move |drawing_area, em| {
            on_motion(&uic, em, drawing_area.get_allocated_width(), drawing_area.get_allocated_height());
            Inhibit(false)
        });
    }
    { // Menu (new)
        let uic = ui.clone();
        menu_new.connect_activate(move |_| {
            uic.new_map_dialog.show();
            let responce_id = uic.new_map_dialog.run();
            uic.new_map_dialog.hide();
            if responce_id == 1 {
                let width  = uic.adjustment_new_map_width.get_value() as u32;
                let height = uic.adjustment_new_map_height.get_value() as u32;
                uic.adjustment_map_pos_x.set_value(0.0);
                uic.adjustment_map_pos_y.set_value(0.0);
                uic.adjustment_map_pos_x.set_upper(width as f64);
                uic.adjustment_map_pos_y.set_upper(height as f64);
                let new_map = EditingMap::new(width, height);
                *uic.map.borrow_mut() = new_map;
                uic.map_redraw();
            }
        });
    }
    { // Menu (quit)
        let uic = ui.clone();
        menu_quit.connect_activate(move |_| {
            uic.window.destroy();
        });
    }
    {
        let uic = ui.clone();
        ui.adjustment_map_pos_x.connect_value_changed(move |_| {
            uic.map_redraw();
        });
    }
    {
        let uic = ui.clone();
        ui.adjustment_map_pos_y.connect_value_changed(move |_| {
            uic.map_redraw();
        });
    }

    set_iconview(&ui);
    ui.window.show_all();
}

fn set_iconview(ui: &Ui) {
    let pbh = &*ui.pbh;
    let objholder = ::common::gobj::get_objholder();
    { // Set tile icons
        ui.iconview_tile.set_pixbuf_column(0);
        ui.iconview_tile.set_text_column(1);
        let uic = ui.clone();
        ui.iconview_tile.connect_selection_changed(move |_| {
            if let Some(path) = uic.iconview_tile.get_selected_items().get(0) {
                uic.tile_selected(path.get_indices()[0]);
            }
        });
        
        let liststore_tile = &ui.liststore_tile;
        for (i, tile) in objholder.tile.iter().enumerate() {
            liststore_tile.insert_with_values(
                None,
                &[0, 1],
                &[pbh.get(TileIdx(i as u32)), &tile.id]);
        }
    }   
}

fn on_map_clicked(ui: &Ui, eb: &gdk::EventButton) {
    if eb.get_button() == WRITE_BUTTON {
        ui.on_drag.set(true);
        try_write(ui, eb.get_position());
    }
}

fn on_motion(ui: &Ui, em: &gdk::EventMotion, w: i32, h: i32) {
    let w = w as f64;
    let h = h as f64;
    let pos = em.get_position();
    if pos.0 < 0.0 || pos.1 < 0.0 || pos.0 > w || pos.1 > h { // Out of drawing widget
        return;
    }
    if ui.on_drag.get() {
        try_write(ui, pos);
    }
}

fn try_write(ui: &Ui, pos: (f64, f64)) {
    use common::basic::TILE_SIZE_I;
    let map_pos = ui.get_map_pos();
    let ix = (pos.0 / TILE_SIZE_I as f64) as i32;
    let iy = (pos.1 / TILE_SIZE_I as f64) as i32;
    let (ix, iy) = (ix + map_pos.0, iy + map_pos.1);
    if ix < ui.map.borrow().width as i32 && iy < ui.map.borrow().height as i32 {
        match ui.selected_item.get() {
            SelectedItem::Tile(idx) => {
                ui.map.borrow_mut().set_tile(Vec2d::new(ix, iy), idx);
            }
        }
        ui.map_redraw();
    }
}

impl Ui {
    fn tile_selected(&self, i: i32) {
        self.selected_item.set(SelectedItem::Tile(TileIdx(i as u32)));
    }

    pub fn map_redraw(&self) {
        self.map_drawing_area.queue_draw();
    }

    pub fn get_map_pos(&self) -> (i32, i32) {
        let pos_x = self.adjustment_map_pos_x.get_value() as i32;
        let pos_y = self.adjustment_map_pos_y.get_value() as i32;
        (pos_x, pos_y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectedItem {
    Tile(TileIdx),
}


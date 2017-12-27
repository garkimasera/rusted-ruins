
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use gdk;
use gtk;
use gtk::prelude::*;
use array2d::Vec2d;
use common::objholder::*;
use pixbuf_holder::PixbufHolder;
use edit_map::EditingMap;
use property_controls::PropertyControls;

const WRITE_BUTTON: u32 = 1;

#[derive(Clone)]
pub struct Ui {
    pub window: gtk::ApplicationWindow,
    pub map_drawing_area: gtk::DrawingArea,
    pub new_map_dialog: gtk::Dialog,
    pub new_map_id:     gtk::Entry,
    pub iconview_tile:  gtk::IconView,
    pub iconview_wall:  gtk::IconView,
    pub liststore_tile: gtk::ListStore,
    pub liststore_wall: gtk::ListStore,
    pub adjustment_new_map_width: gtk::Adjustment,
    pub adjustment_new_map_height: gtk::Adjustment,
    pub adjustment_map_pos_x: gtk::Adjustment,
    pub adjustment_map_pos_y: gtk::Adjustment,
    pub property_controls: PropertyControls,
    pub pbh: Rc<PixbufHolder>,
    pub map: Rc<RefCell<EditingMap>>,
    pub selected_item: Rc<Cell<SelectedItem>>,
    pub on_drag: Rc<Cell<bool>>,
    pub filepath: Rc<RefCell<Option<PathBuf>>>,
    /// If it is false, some signal will not be processed
    pub signal_mode: Rc<Cell<bool>>,
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
        new_map_id:       get_object!(builder, "new-map-id"),
        iconview_tile:    get_object!(builder, "iconview-tile"),
        iconview_wall:    get_object!(builder, "iconview-wall"),
        liststore_tile:   get_object!(builder, "liststore-tile"),
        liststore_wall:   get_object!(builder, "liststore-wall"),
        adjustment_new_map_width:  get_object!(builder, "adjustment-new-map-width"),
        adjustment_new_map_height: get_object!(builder, "adjustment-new-map-height"),
        adjustment_map_pos_x:      get_object!(builder, "adjustment-map-pos-x"),
        adjustment_map_pos_y:      get_object!(builder, "adjustment-map-pos-y"),
        property_controls: PropertyControls::build(&builder),
        pbh: Rc::new(PixbufHolder::new()),
        map: Rc::new(RefCell::new(EditingMap::new("newmap", 16, 16))),
        selected_item: Rc::new(Cell::new(SelectedItem::Tile(TileIdx(0)))),
        on_drag: Rc::new(Cell::new(false)),
        filepath: Rc::new(RefCell::new(None)),
        signal_mode: Rc::new(Cell::new(true)),
    };

    let menu_new:     gtk::MenuItem = get_object!(builder, "menu-new");
    let menu_open:    gtk::MenuItem = get_object!(builder, "menu-open");
    let menu_save:    gtk::MenuItem = get_object!(builder, "menu-save");
    let menu_save_as: gtk::MenuItem = get_object!(builder, "menu-save-as");
    let menu_quit:    gtk::MenuItem = get_object!(builder, "menu-quit");

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
                let new_map_id = uic.new_map_id.get_text().unwrap_or("newmap".into());
                let new_map = EditingMap::new(&new_map_id, width, height);
                *uic.map.borrow_mut() = new_map;
                uic.set_signal_mode(false);
                uic.property_controls.update(&*uic.map.borrow());
                uic.set_signal_mode(true);
                uic.map_redraw();
            }
        });
    }
    { // Menu (open)
        let uic = ui.clone();
        menu_open.connect_activate(move |_| {
            if let Some(path) = file_open(&uic) {
                match ::file::load_from_file(&path) {
                    Ok(mapobj) => {
                        {
                            *uic.map.borrow_mut() = EditingMap::from(mapobj);
                        }
                        uic.set_signal_mode(false);
                        uic.property_controls.update(&*uic.map.borrow());
                        uic.set_signal_mode(true);
                    }
                    Err(_) => (),
                }
            }
        });
    }
    { // Menu (save)
        let uic = ui.clone();
        menu_save.connect_activate(move |_| {
            let path = if let Some(path) = (*uic.filepath).clone().into_inner() {
                path
            } else {
                if let Some(path) = file_save_as(&uic) {
                    path
                } else {
                    return;
                }
            };
            save_to(&uic, path);
        });
    }
    { // Menu (save as)
        let uic = ui.clone();
        menu_save_as.connect_activate(move |_| {
            if let Some(path) = file_save_as(&uic) {
                save_to(&uic, path)
            }
        });
    }
    { // Menu (quit)
        let uic = ui.clone();
        menu_quit.connect_activate(move |_| {
            uic.window.destroy();
        });
    }
    { // Scroll (x)
        let uic = ui.clone();
        ui.adjustment_map_pos_x.connect_value_changed(move |_| {
            uic.map_redraw();
        });
    }
    { // Scroll (y)
        let uic = ui.clone();
        ui.adjustment_map_pos_y.connect_value_changed(move |_| {
            uic.map_redraw();
        });
    }

    ::property_controls::connect_for_property_controls(&ui);
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
    { // Set wall icons
        ui.iconview_wall.set_pixbuf_column(0);
        ui.iconview_wall.set_text_column(1);
        let uic = ui.clone();
        ui.iconview_wall.connect_selection_changed(move |_| {
            if let Some(path) = uic.iconview_wall.get_selected_items().get(0) {
                uic.wall_selected(path.get_indices()[0]);
            }
        });
        
        let liststore_wall = &ui.liststore_wall;
        for (i, wall) in objholder.wall.iter().enumerate() {
            liststore_wall.insert_with_values(
                None,
                &[0, 1],
                &[pbh.get(WallIdx(i as u32)), &wall.id]);
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

fn file_open(ui: &Ui) -> Option<PathBuf> {
    let file_chooser = gtk::FileChooserDialog::new(
        Some("Open map object file"), Some(&ui.window), gtk::FileChooserAction::Open);
    file_chooser.add_buttons(&[
        ("Open", gtk::ResponseType::Ok.into()),
        ("Cancel", gtk::ResponseType::Cancel.into()),
    ]);
    file_chooser.add_filter(&create_file_filter());
    if file_chooser.run() == gtk::ResponseType::Ok.into() {
        let filename = file_chooser.get_filename().expect("Couldn't get filename");
        file_chooser.destroy();
        return Some(filename);
    }
    file_chooser.destroy();
    None
}

fn file_save_as(ui: &Ui) -> Option<PathBuf> {
    let file_chooser = gtk::FileChooserDialog::new(
        Some("Save map object"), Some(&ui.window), gtk::FileChooserAction::Save);
    file_chooser.add_buttons(&[
        ("Save", gtk::ResponseType::Ok.into()),
        ("Cancel", gtk::ResponseType::Cancel.into()),
    ]);
    file_chooser.add_filter(&create_file_filter());
    if file_chooser.run() == gtk::ResponseType::Ok.into() {
        let filename = file_chooser.get_filename().expect("Couldn't get filename");
        file_chooser.destroy();
        return Some(filename);
    }
    file_chooser.destroy();
    None
}

fn save_to(ui: &Ui, path: PathBuf) {
    let mapobj = ui.map.borrow().create_mapobj();
    let _ = ::file::save_to_file(&path, mapobj);
}

fn create_file_filter() -> gtk::FileFilter {
    let f = gtk::FileFilter::new();
    f.add_pattern("*.pak");
    gtk::FileFilterExt::set_name(&f, "Rusted Ruins pak file");
    f
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
            SelectedItem::Wall(idx) => {
                ui.map.borrow_mut().set_wall(Vec2d::new(ix, iy), Some(idx));
            }
        }
        ui.map_redraw();
    }
}

impl Ui {
    fn tile_selected(&self, i: i32) {
        self.selected_item.set(SelectedItem::Tile(TileIdx(i as u32)));
    }

    fn wall_selected(&self, i: i32) {
        self.selected_item.set(SelectedItem::Wall(WallIdx(i as u32)));
    }

    pub fn map_redraw(&self) {
        self.map_drawing_area.queue_draw();
    }

    pub fn get_map_pos(&self) -> (i32, i32) {
        let pos_x = self.adjustment_map_pos_x.get_value() as i32;
        let pos_y = self.adjustment_map_pos_y.get_value() as i32;
        (pos_x, pos_y)
    }

    pub fn set_signal_mode(&self, mode: bool) {
        self.signal_mode.set(mode);
    }

    pub fn get_signal_mode(&self) -> bool {
        self.signal_mode.get()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectedItem {
    Tile(TileIdx), Wall(WallIdx),
}


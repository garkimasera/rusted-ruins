
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
use iconview::IconView;
use property_controls::PropertyControls;

const WRITE_BUTTON: u32 = 1;

#[derive(Clone)]
pub struct Ui {
    pub window: gtk::ApplicationWindow,
    pub map_drawing_area: gtk::DrawingArea,
    pub new_map_dialog: gtk::Dialog,
    pub new_map_id:     gtk::Entry,
    pub adjustment_new_map_width: gtk::Adjustment,
    pub adjustment_new_map_height: gtk::Adjustment,
    pub adjustment_map_pos_x: gtk::Adjustment,
    pub adjustment_map_pos_y: gtk::Adjustment,
    pub label_cursor_pos:    gtk::Label,
    pub label_selected_item: gtk::Label,
    pub iconview: IconView,
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
        adjustment_new_map_width:  get_object!(builder, "adjustment-new-map-width"),
        adjustment_new_map_height: get_object!(builder, "adjustment-new-map-height"),
        adjustment_map_pos_x:      get_object!(builder, "adjustment-map-pos-x"),
        adjustment_map_pos_y:      get_object!(builder, "adjustment-map-pos-y"),
        label_cursor_pos:    get_object!(builder, "label-cursor-pos"),
        label_selected_item: get_object!(builder, "label-selected-item"),
        iconview: IconView::build(&builder),
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
    ::iconview::set_iconview(&ui);
    ui.window.show_all();
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
    // Update cursor position display
    let (ix, iy) = ui.cursor_to_tile_pos(pos);
    let text = format!("({},{})", ix, iy);
    ui.label_cursor_pos.set_text(&text);
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
    let (ix, iy) = ui.cursor_to_tile_pos(pos);
    if ix < ui.map.borrow().width as i32 && iy < ui.map.borrow().height as i32 {
        match ui.selected_item.get() {
            SelectedItem::Tile(idx) => {
                ui.map.borrow_mut().set_tile(Vec2d::new(ix, iy), idx);
            }
            SelectedItem::Wall(idx) => {
                ui.map.borrow_mut().set_wall(Vec2d::new(ix, iy), Some(idx));
            }
            SelectedItem::Deco(idx) => {
                ui.map.borrow_mut().set_deco(Vec2d::new(ix, iy), Some(idx));
            }
        }
        ui.map_redraw();
    }
}

impl Ui {
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

    pub fn cursor_to_tile_pos(&self, pos: (f64, f64)) -> (i32, i32) {
        let map_pos = self.get_map_pos();
        use common::basic::TILE_SIZE_I;
        let ix = (pos.0 / TILE_SIZE_I as f64) as i32;
        let iy = (pos.1 / TILE_SIZE_I as f64) as i32;
        (ix + map_pos.0, iy + map_pos.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectedItem {
    Tile(TileIdx), Wall(WallIdx), Deco(DecoIdx),
}


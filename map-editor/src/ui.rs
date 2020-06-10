use crate::edit_map::EditingMap;
use crate::iconview::IconView;
use crate::pixbuf_holder::PixbufHolder;
use crate::property_controls::PropertyControls;
use common::basic::TILE_SIZE_I;
use common::gamedata::ItemGen;
use common::objholder::*;
use geom::Vec2d;
use gtk::prelude::*;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::path::PathBuf;
use std::rc::Rc;

const WRITE_BUTTON: u32 = 1;
const CENTERING_BUTTON: u32 = 2;
const ERASE_BUTTON: u32 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DragMode {
    None,
    Write,
    Erase,
}

#[derive(Clone)]
pub struct Ui {
    pub window: gtk::ApplicationWindow,
    pub map_drawing_area: gtk::DrawingArea,
    pub new_map_dialog: gtk::Dialog,
    pub resize_dialog: gtk::Dialog,
    pub new_map_id: gtk::Entry,
    pub adjustment_map_width: gtk::Adjustment,
    pub adjustment_map_height: gtk::Adjustment,
    pub adjustment_map_pos_x: gtk::Adjustment,
    pub adjustment_map_pos_y: gtk::Adjustment,
    pub adjustment_offset_x: gtk::Adjustment,
    pub adjustment_offset_y: gtk::Adjustment,
    pub label_cursor_pos: gtk::Label,
    pub label_selected_item: gtk::Label,
    pub radiobutton_layer0: gtk::RadioButton,
    pub radiobutton_layer1: gtk::RadioButton,
    pub radiobutton_layer2: gtk::RadioButton,
    pub radiobutton_layer3: gtk::RadioButton,
    pub checkbutton_layer0: gtk::CheckButton,
    pub checkbutton_layer1: gtk::CheckButton,
    pub checkbutton_layer2: gtk::CheckButton,
    pub checkbutton_layer3: gtk::CheckButton,
    pub radiobutton_rect: gtk::RadioButton,
    pub iconview: IconView,
    pub property_controls: PropertyControls,
    pub pbh: Rc<PixbufHolder>,
    pub map: Rc<RefCell<EditingMap>>,
    pub selected_item: Rc<Cell<SelectedItem>>,
    pub drag_mode: Rc<Cell<DragMode>>,
    pub filepath: Rc<RefCell<Option<PathBuf>>>,
    /// If it is false, some signal will not be processed
    pub signal_mode: Rc<Cell<bool>>,
    /// Shift key state
    pub shift: Rc<Cell<bool>>,
    /// Current layer to draw
    pub current_layer: Rc<Cell<usize>>,
    pub layer_visible: Rc<RefCell<[bool; 4]>>,
    pub drag_start: Rc<Cell<Option<Vec2d>>>,
}

macro_rules! get_object {
    ($builder:expr, $id:expr) => {
        if let Some(object) = $builder.get_object($id) {
            object
        } else {
            panic!("Builder Error: \"{}\" is not found", $id)
        }
    };
}

pub fn build_ui(application: &gtk::Application) {
    // Get widgets from glade file
    let builder = gtk::Builder::new_from_string(include_str!("ui.glade"));

    let ui = Ui {
        window: get_object!(builder, "window1"),
        map_drawing_area: get_object!(builder, "map-drawing-area"),
        new_map_dialog: get_object!(builder, "new-map-dialog"),
        resize_dialog: get_object!(builder, "resize-dialog"),
        new_map_id: get_object!(builder, "new-map-id"),
        adjustment_map_width: get_object!(builder, "adjustment-map-width"),
        adjustment_map_height: get_object!(builder, "adjustment-map-height"),
        adjustment_map_pos_x: get_object!(builder, "adjustment-map-pos-x"),
        adjustment_map_pos_y: get_object!(builder, "adjustment-map-pos-y"),
        adjustment_offset_x: get_object!(builder, "adjustment-offset-x"),
        adjustment_offset_y: get_object!(builder, "adjustment-offset-y"),
        label_cursor_pos: get_object!(builder, "label-cursor-pos"),
        label_selected_item: get_object!(builder, "label-selected-item"),
        radiobutton_layer0: get_object!(builder, "radiobutton-layer0"),
        radiobutton_layer1: get_object!(builder, "radiobutton-layer1"),
        radiobutton_layer2: get_object!(builder, "radiobutton-layer2"),
        radiobutton_layer3: get_object!(builder, "radiobutton-layer3"),
        checkbutton_layer0: get_object!(builder, "checkbutton-layer0"),
        checkbutton_layer1: get_object!(builder, "checkbutton-layer1"),
        checkbutton_layer2: get_object!(builder, "checkbutton-layer2"),
        checkbutton_layer3: get_object!(builder, "checkbutton-layer3"),
        radiobutton_rect: get_object!(builder, "radiobutton-rect"),
        iconview: IconView::build(&builder),
        property_controls: PropertyControls::build(&builder),
        pbh: Rc::new(PixbufHolder::new()),
        map: Rc::new(RefCell::new(EditingMap::new("newmap", 16, 16))),
        selected_item: Rc::new(Cell::new(SelectedItem::Tile(TileIdx::default()))),
        drag_mode: Rc::new(Cell::new(DragMode::None)),
        filepath: Rc::new(RefCell::new(None)),
        signal_mode: Rc::new(Cell::new(true)),
        shift: Rc::new(Cell::new(false)),
        current_layer: Rc::new(Cell::new(0)),
        layer_visible: Rc::new(RefCell::new([true; 4])),
        drag_start: Rc::new(Cell::new(None)),
    };

    let menu_new: gtk::MenuItem = get_object!(builder, "menu-new");
    let menu_open: gtk::MenuItem = get_object!(builder, "menu-open");
    let menu_save: gtk::MenuItem = get_object!(builder, "menu-save");
    let menu_save_as: gtk::MenuItem = get_object!(builder, "menu-save-as");
    let menu_quit: gtk::MenuItem = get_object!(builder, "menu-quit");
    let menu_resize: gtk::MenuItem = get_object!(builder, "menu-resize");

    ui.window.set_application(Some(application));
    // Connect signals
    {
        let uic = ui.clone();
        ui.window.connect_delete_event(move |_, _| {
            uic.window.destroy();
            Inhibit(false)
        });
    }
    {
        // Map drawing area (draw)
        let uic = ui.clone();
        ui.map_drawing_area.connect_draw(move |widget, context| {
            let width = widget.get_allocated_width();
            let height = widget.get_allocated_height();
            let map = uic.map.borrow();
            let pos = uic.get_map_pos();
            crate::draw_map::draw_map(
                context,
                &*map,
                &*uic.pbh,
                width,
                height,
                pos,
                *uic.layer_visible.borrow(),
            );
            Inhibit(false)
        });
    }
    {
        // Map drawing area (button pressed)
        let uic = ui.clone();
        use gdk::EventMask;
        let mask = EventMask::BUTTON_PRESS_MASK
            | EventMask::BUTTON_RELEASE_MASK
            | EventMask::POINTER_MOTION_MASK;
        ui.map_drawing_area.add_events(mask);
        ui.map_drawing_area
            .connect_button_press_event(move |_, eb| {
                on_map_clicked(&uic, eb);
                Inhibit(false)
            });
        let uic = ui.clone();
        ui.map_drawing_area
            .connect_button_release_event(move |_, eb| {
                on_button_released(&uic, eb);
                Inhibit(false)
            });
    }
    {
        // Map drawing area (button pressed)
        let uic = ui.clone();
        ui.map_drawing_area
            .connect_button_release_event(move |_, _| {
                uic.drag_mode.set(DragMode::None);
                Inhibit(false)
            });
    }
    {
        // Map drawing area (motion)
        let uic = ui.clone();
        ui.map_drawing_area
            .connect_motion_notify_event(move |drawing_area, em| {
                on_motion(
                    &uic,
                    em,
                    drawing_area.get_allocated_width(),
                    drawing_area.get_allocated_height(),
                );
                Inhibit(false)
            });
    }
    {
        // Menu (new)
        let uic = ui.clone();
        menu_new.connect_activate(move |_| {
            uic.new_map_dialog.show();
            let responce_id = uic.new_map_dialog.run();
            uic.new_map_dialog.hide();
            if responce_id == gtk::ResponseType::Other(1) {
                let width = uic.adjustment_map_width.get_value() as u32;
                let height = uic.adjustment_map_height.get_value() as u32;
                uic.reset_map_size(width, height);
                let new_map_id = uic.new_map_id.get_text().unwrap_or("newmap".into());
                let new_map = EditingMap::new(&new_map_id, width, height);
                *uic.map.borrow_mut() = new_map;
                uic.set_signal_mode(false);
                uic.property_controls.update(&*uic.map.borrow());
                uic.set_signal_mode(true);
                uic.map_redraw();
                *uic.filepath.borrow_mut() = None;
            }
        });
    }
    {
        // Menu (open)
        let uic = ui.clone();
        menu_open.connect_activate(move |_| {
            if let Some(path) = file_open(&uic) {
                match crate::file::load_from_file(&path) {
                    Ok(mapobj) => {
                        {
                            *uic.map.borrow_mut() = EditingMap::from(mapobj);
                        }
                        uic.set_signal_mode(false);
                        uic.property_controls.update(&*uic.map.borrow());
                        uic.set_signal_mode(true);
                        uic.reset_map_size(uic.map.borrow().width, uic.map.borrow().height);
                        *uic.filepath.borrow_mut() = Some(path);
                    }
                    Err(e) => {
                        show_err_dialog(&uic, &e.to_string());
                    }
                }
            }
        });
    }
    {
        // Menu (save)
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
            match save_to(&uic, path.clone()) {
                Ok(_) => {
                    *uic.filepath.borrow_mut() = Some(path);
                }
                Err(e) => {
                    show_err_dialog(&uic, &e.to_string());
                }
            }
        });
    }
    {
        // Menu (save as)
        let uic = ui.clone();
        menu_save_as.connect_activate(move |_| {
            if let Some(path) = file_save_as(&uic) {
                match save_to(&uic, path.clone()) {
                    Ok(_) => {
                        *uic.filepath.borrow_mut() = Some(path);
                    }
                    Err(e) => {
                        show_err_dialog(&uic, &e.to_string());
                    }
                }
            }
        });
    }
    {
        // Menu (quit)
        let uic = ui.clone();
        menu_quit.connect_activate(move |_| {
            uic.window.destroy();
        });
    }
    {
        // Menu (resize)
        let uic = ui.clone();
        menu_resize.connect_activate(move |_| {
            uic.adjustment_map_width
                .set_value(uic.map.borrow().width as f64);
            uic.adjustment_map_height
                .set_value(uic.map.borrow().height as f64);
            uic.resize_dialog.show();
            let responce_id = uic.resize_dialog.run();
            uic.resize_dialog.hide();
            if responce_id == gtk::ResponseType::Other(1) {
                let width = uic.adjustment_map_width.get_value() as u32;
                let height = uic.adjustment_map_height.get_value() as u32;
                let offset_x = uic.adjustment_offset_x.get_value() as i32;
                let offset_y = uic.adjustment_offset_y.get_value() as i32;
                uic.adjustment_map_pos_x.set_value(0.0);
                uic.adjustment_map_pos_y.set_value(0.0);
                uic.adjustment_map_pos_x.set_upper(width as f64);
                uic.adjustment_map_pos_y.set_upper(height as f64);
                uic.map
                    .borrow_mut()
                    .resize(width, height, offset_x, offset_y);
                uic.map_redraw();
            }
        });
    }
    {
        // Scroll (x)
        let uic = ui.clone();
        ui.adjustment_map_pos_x.connect_value_changed(move |_| {
            uic.map_redraw();
        });
    }
    {
        // Scroll (y)
        let uic = ui.clone();
        ui.adjustment_map_pos_y.connect_value_changed(move |_| {
            uic.map_redraw();
        });
    }
    {
        // Key press
        use gdk::enums::key::{Shift_L, Shift_R};
        let uic = ui.clone();
        ui.window.connect_key_press_event(move |_, event_key| {
            let keyval = event_key.get_keyval();
            if keyval == Shift_L || keyval == Shift_R {
                uic.shift.set(true);
            }
            Inhibit(false)
        });
        let uic = ui.clone();
        ui.window.connect_key_release_event(move |_, event_key| {
            let keyval = event_key.get_keyval();
            if keyval == Shift_L || keyval == Shift_R {
                uic.shift.set(false);
            }
            Inhibit(false)
        });
    }
    {
        // Layer bottons
        let uic = ui.clone();
        ui.radiobutton_layer0.connect_toggled(move |_| {
            // Layer 0
            uic.current_layer.set(0);
        });
        let uic = ui.clone();
        ui.radiobutton_layer1.connect_toggled(move |_| {
            // Layer 1
            uic.current_layer.set(1);
        });
        let uic = ui.clone();
        ui.radiobutton_layer2.connect_toggled(move |_| {
            // Layer 2
            uic.current_layer.set(2);
        });
        let uic = ui.clone();
        ui.radiobutton_layer3.connect_toggled(move |_| {
            // Layer 3
            uic.current_layer.set(3);
        });
    }
    {
        // Layer check bottons
        let uic = ui.clone();
        ui.checkbutton_layer0.connect_toggled(move |b| {
            // Layer 0
            uic.layer_visible.borrow_mut()[0] = b.get_active();
            uic.map_redraw();
        });
        let uic = ui.clone();
        ui.checkbutton_layer1.connect_toggled(move |b| {
            // Layer 1
            uic.layer_visible.borrow_mut()[1] = b.get_active();
            uic.map_redraw();
        });
        let uic = ui.clone();
        ui.checkbutton_layer2.connect_toggled(move |b| {
            // Layer 2
            uic.layer_visible.borrow_mut()[2] = b.get_active();
            uic.map_redraw();
        });
        let uic = ui.clone();
        ui.checkbutton_layer3.connect_toggled(move |b| {
            // Layer 3
            uic.layer_visible.borrow_mut()[3] = b.get_active();
            uic.map_redraw();
        });
    }

    crate::property_controls::connect_for_property_controls(&ui);
    crate::iconview::set_iconview(&ui);
    ui.window.show_all();
}

fn on_map_clicked(ui: &Ui, eb: &gdk::EventButton) {
    let button = eb.get_button();
    if button == WRITE_BUTTON {
        ui.drag_start
            .set(Some(Vec2d::from(ui.cursor_to_tile_pos(eb.get_position()))));
        if !ui.radiobutton_rect.get_active() {
            ui.drag_mode.set(DragMode::Write);
            try_write(ui, eb.get_position());
        }
    } else if button == CENTERING_BUTTON {
        centering_to(ui, ui.cursor_to_tile_pos(eb.get_position()));
    } else if button == ERASE_BUTTON {
        ui.drag_mode.set(DragMode::Erase);
        try_erase(ui, eb.get_position());
    }
}

fn on_button_released(ui: &Ui, eb: &gdk::EventButton) {
    let button = eb.get_button();
    if button == WRITE_BUTTON {
        let start = if let Some(start) = ui.drag_start.replace(None) {
            start
        } else {
            return;
        };
        let end = Vec2d::from(ui.cursor_to_tile_pos(eb.get_position()));
        if ui.radiobutton_rect.get_active() {
            try_write_rect(ui, start, end);
        }
    }
}

fn on_motion(ui: &Ui, em: &gdk::EventMotion, w: i32, h: i32) {
    let w = w as f64;
    let h = h as f64;
    let pos = em.get_position();
    if pos.0 < 0.0 || pos.1 < 0.0 || pos.0 > w || pos.1 > h {
        // Out of drawing widget
        return;
    }
    match ui.drag_mode.get() {
        DragMode::Write => {
            try_write(ui, pos);
        }
        DragMode::Erase => {
            try_erase(ui, pos);
        }
        _ => (),
    }
    // Update cursor position display
    let (ix, iy) = ui.cursor_to_tile_pos(pos);
    let text = format!("({},{})", ix, iy);
    ui.label_cursor_pos.set_text(&text);
}

fn file_open(ui: &Ui) -> Option<PathBuf> {
    let file_chooser = gtk::FileChooserDialog::new(
        Some("Open map object file"),
        Some(&ui.window),
        gtk::FileChooserAction::Open,
    );
    file_chooser.add_buttons(&[
        ("Open", gtk::ResponseType::Ok.into()),
        ("Cancel", gtk::ResponseType::Cancel.into()),
    ]);
    file_chooser.add_filter(&create_file_filter());
    if file_chooser.run() == gtk::ResponseType::Ok {
        let filename = file_chooser.get_filename().expect("Couldn't get filename");
        file_chooser.destroy();
        ui.map_redraw();
        return Some(filename);
    }
    file_chooser.destroy();
    None
}

fn file_save_as(ui: &Ui) -> Option<PathBuf> {
    let file_chooser = gtk::FileChooserDialog::new(
        Some("Save map object"),
        Some(&ui.window),
        gtk::FileChooserAction::Save,
    );
    file_chooser.add_buttons(&[
        ("Save", gtk::ResponseType::Ok.into()),
        ("Cancel", gtk::ResponseType::Cancel.into()),
    ]);
    file_chooser.add_filter(&create_file_filter());
    if file_chooser.run() == gtk::ResponseType::Ok {
        let filename = file_chooser.get_filename().expect("Couldn't get filename");
        file_chooser.destroy();
        return Some(filename);
    }
    file_chooser.destroy();
    None
}

fn save_to(ui: &Ui, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mapobj = ui.map.borrow().create_mapobj();
    crate::file::save_to_file(&path, mapobj)
}

fn create_file_filter() -> gtk::FileFilter {
    let f = gtk::FileFilter::new();
    f.add_pattern("*.pak");
    f.set_name(Some("Rusted Ruins pak file"));
    f
}

fn try_write(ui: &Ui, pos: (f64, f64)) {
    let (ix, iy) = ui.cursor_to_tile_pos(pos);
    if ix < ui.map.borrow().width as i32 && iy < ui.map.borrow().height as i32 {
        match ui.selected_item.get() {
            SelectedItem::Tile(idx) => {
                if ui.shift.get() {
                    ui.map
                        .borrow_mut()
                        .tile_layer_draw(Vec2d(ix, iy), idx, ui.current_layer.get());
                } else {
                    ui.map
                        .borrow_mut()
                        .set_tile(Vec2d(ix, iy), idx, ui.current_layer.get());
                }
            }
            SelectedItem::Wall(idx) => {
                ui.map.borrow_mut().set_wall(Vec2d(ix, iy), Some(idx));
            }
            SelectedItem::Deco(idx) => {
                ui.map.borrow_mut().set_deco(Vec2d(ix, iy), Some(idx));
            }
            SelectedItem::Item(idx) => {
                let id = common::gobj::idx_to_id(idx).to_owned();
                ui.map
                    .borrow_mut()
                    .set_item(Vec2d(ix, iy), Some(ItemGen { id }));
            }
            SelectedItem::SelectTile => {
                ui.property_controls.selected_tile.set(Vec2d(ix, iy));
                ui.property_controls
                    .label_selected_tile
                    .set_text(&format!("Selected Tile ({}, {})", ix, iy));
                ui.set_signal_mode(false);
                ui.property_controls.update(&*ui.map.borrow());
                ui.set_signal_mode(true);
            }
        }
        ui.map_redraw();
    }
}

fn try_write_rect(ui: &Ui, start: Vec2d, end: Vec2d) {
    use std::cmp::{max, min};
    let width = ui.map.borrow().width as i32;
    let height = ui.map.borrow().height as i32;
    let start = Vec2d::new(
        min(max(start.0, 0), width - 1),
        min(max(start.1, 0), height - 1),
    );
    let end = Vec2d::new(
        min(max(end.0, 0), width - 1),
        min(max(end.1, 0), height - 1),
    );
    let (start, end) = (
        Vec2d::new(min(start.0, end.0), min(start.1, end.1)),
        Vec2d::new(max(start.0, end.0), max(start.1, end.1)),
    );
    for p in geom::RectIter::new(start, end) {
        match ui.selected_item.get() {
            SelectedItem::Tile(idx) => {
                ui.map.borrow_mut().set_tile(p, idx, ui.current_layer.get());
            }
            SelectedItem::Wall(idx) => {
                ui.map.borrow_mut().set_wall(p, Some(idx));
            }
            SelectedItem::Deco(idx) => {
                ui.map.borrow_mut().set_deco(p, Some(idx));
            }
            _ => (),
        }
    }
    ui.map_redraw();
}

fn try_erase(ui: &Ui, pos: (f64, f64)) {
    let (ix, iy) = ui.cursor_to_tile_pos(pos);
    if !(ix < ui.map.borrow().width as i32 && iy < ui.map.borrow().height as i32) {
        return;
    }
    match ui.selected_item.get() {
        SelectedItem::Tile(_) => {
            ui.map
                .borrow_mut()
                .erase_layer(Vec2d(ix, iy), ui.current_layer.get());
        }
        SelectedItem::Item(_) => {
            ui.map.borrow_mut().set_item(Vec2d(ix, iy), None);
        }
        _ => {
            ui.map.borrow_mut().erase(Vec2d(ix, iy));
        }
    }
    ui.map_redraw();
}

fn centering_to(ui: &Ui, pos: (i32, i32)) {
    let area_w = ui.map_drawing_area.get_allocated_width();
    let area_h = ui.map_drawing_area.get_allocated_height();
    let x = pos.0 - area_w / (TILE_SIZE_I * 2);
    let y = pos.1 - area_h / (TILE_SIZE_I * 2);
    ui.adjustment_map_pos_x.set_value(x as f64);
    ui.adjustment_map_pos_y.set_value(y as f64);
    ui.map_redraw();
}

fn show_err_dialog(ui: &Ui, msg: &str) {
    let dialog = gtk::MessageDialog::new(
        Some(&ui.window),
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        msg,
    );
    dialog.show();
    dialog.run();
    dialog.destroy();
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

    pub fn reset_map_size(&self, width: u32, height: u32) {
        self.adjustment_map_pos_x.set_value(0.0);
        self.adjustment_map_pos_y.set_value(0.0);
        self.adjustment_map_pos_x.set_upper(width as f64);
        self.adjustment_map_pos_y.set_upper(height as f64);
    }

    pub fn set_signal_mode(&self, mode: bool) {
        self.signal_mode.set(mode);
    }

    pub fn get_signal_mode(&self) -> bool {
        self.signal_mode.get()
    }

    pub fn cursor_to_tile_pos(&self, pos: (f64, f64)) -> (i32, i32) {
        let map_pos = self.get_map_pos();
        let ix = (pos.0 / TILE_SIZE_I as f64) as i32;
        let iy = (pos.1 / TILE_SIZE_I as f64) as i32;
        (ix + map_pos.0, iy + map_pos.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectedItem {
    Tile(TileIdx),
    Wall(WallIdx),
    Deco(DecoIdx),
    Item(ItemIdx),
    SelectTile,
}

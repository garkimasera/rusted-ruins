use crate::edit_map::EditingMap;
use crate::ui::{SelectedItem, Ui};
use common::maptemplate::MapTemplateBoundaryBehavior;
use geom::Coords;
use gtk::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct PropertyControls {
    pub map_id: gtk::Entry,
    pub region_map: gtk::CheckButton,
    pub entrance_pos: gtk::Entry,
    pub music: gtk::Entry,
    pub boundary_n_none: gtk::RadioButton,
    pub boundary_n_next: gtk::RadioButton,
    pub boundary_n_prev: gtk::RadioButton,
    pub boundary_n_region: gtk::RadioButton,
    pub boundary_s_none: gtk::RadioButton,
    pub boundary_s_next: gtk::RadioButton,
    pub boundary_s_prev: gtk::RadioButton,
    pub boundary_s_region: gtk::RadioButton,
    pub boundary_e_none: gtk::RadioButton,
    pub boundary_e_next: gtk::RadioButton,
    pub boundary_e_prev: gtk::RadioButton,
    pub boundary_e_region: gtk::RadioButton,
    pub boundary_w_none: gtk::RadioButton,
    pub boundary_w_next: gtk::RadioButton,
    pub boundary_w_prev: gtk::RadioButton,
    pub boundary_w_region: gtk::RadioButton,
    pub button_select_tile_mode: gtk::Button,
    pub label_selected_tile: gtk::Label,
    pub entry_item_id: gtk::Entry,
    pub selected_tile: Rc<Cell<Coords>>,
}

impl PropertyControls {
    pub fn build(builder: &gtk::Builder) -> PropertyControls {
        PropertyControls {
            map_id: get_object!(builder, "property-map-id"),
            region_map: get_object!(builder, "property-region-map"),
            entrance_pos: get_object!(builder, "property-entrance-pos"),
            music: get_object!(builder, "property-music"),
            boundary_n_none: get_object!(builder, "property-boundary-n-none"),
            boundary_n_next: get_object!(builder, "property-boundary-n-next"),
            boundary_n_prev: get_object!(builder, "property-boundary-n-prev"),
            boundary_n_region: get_object!(builder, "property-boundary-n-region"),
            boundary_s_none: get_object!(builder, "property-boundary-s-none"),
            boundary_s_next: get_object!(builder, "property-boundary-s-next"),
            boundary_s_prev: get_object!(builder, "property-boundary-s-prev"),
            boundary_s_region: get_object!(builder, "property-boundary-s-region"),
            boundary_e_none: get_object!(builder, "property-boundary-e-none"),
            boundary_e_next: get_object!(builder, "property-boundary-e-next"),
            boundary_e_prev: get_object!(builder, "property-boundary-e-prev"),
            boundary_e_region: get_object!(builder, "property-boundary-e-region"),
            boundary_w_none: get_object!(builder, "property-boundary-w-none"),
            boundary_w_next: get_object!(builder, "property-boundary-w-next"),
            boundary_w_prev: get_object!(builder, "property-boundary-w-prev"),
            boundary_w_region: get_object!(builder, "property-boundary-w-region"),
            button_select_tile_mode: get_object!(builder, "button-select-tile-mode"),
            label_selected_tile: get_object!(builder, "label-selected-tile"),
            entry_item_id: get_object!(builder, "entry-item-id"),
            selected_tile: Rc::new(Cell::new(Coords(0, 0))),
        }
    }

    pub fn update(&self, map: &EditingMap) {
        self.map_id.set_text(&map.property.id);
        self.region_map.set_active(map.property.is_region_map);
        if let Some(entrance) = map.property.entrance.get(0) {
            self.entrance_pos
                .set_text(&format!("{},{}", entrance.0, entrance.1));
        }
        self.music.set_text(&map.property.music);
        match map.property.boundary.n {
            MapTemplateBoundaryBehavior::None => {
                self.boundary_n_none.set_active(true);
            }
            MapTemplateBoundaryBehavior::NextFloor => {
                self.boundary_n_next.set_active(true);
            }
            MapTemplateBoundaryBehavior::PrevFloor => {
                self.boundary_n_prev.set_active(true);
            }
            MapTemplateBoundaryBehavior::Exit => {
                self.boundary_n_region.set_active(true);
            }
        }
        match map.property.boundary.s {
            MapTemplateBoundaryBehavior::None => {
                self.boundary_s_none.set_active(true);
            }
            MapTemplateBoundaryBehavior::NextFloor => {
                self.boundary_s_next.set_active(true);
            }
            MapTemplateBoundaryBehavior::PrevFloor => {
                self.boundary_s_prev.set_active(true);
            }
            MapTemplateBoundaryBehavior::Exit => {
                self.boundary_s_region.set_active(true);
            }
        }
        match map.property.boundary.e {
            MapTemplateBoundaryBehavior::None => {
                self.boundary_e_none.set_active(true);
            }
            MapTemplateBoundaryBehavior::NextFloor => {
                self.boundary_e_next.set_active(true);
            }
            MapTemplateBoundaryBehavior::PrevFloor => {
                self.boundary_e_prev.set_active(true);
            }
            MapTemplateBoundaryBehavior::Exit => {
                self.boundary_e_region.set_active(true);
            }
        }
        match map.property.boundary.w {
            MapTemplateBoundaryBehavior::None => {
                self.boundary_w_none.set_active(true);
            }
            MapTemplateBoundaryBehavior::NextFloor => {
                self.boundary_w_next.set_active(true);
            }
            MapTemplateBoundaryBehavior::PrevFloor => {
                self.boundary_w_prev.set_active(true);
            }
            MapTemplateBoundaryBehavior::Exit => {
                self.boundary_w_region.set_active(true);
            }
        }
        if let Some(item_gen) = map.get_item(self.selected_tile.get()) {
            self.entry_item_id.set_text(&item_gen.id);
        } else {
            self.entry_item_id.set_text("");
        }
    }
}

pub fn connect_for_property_controls(ui: &Ui) {
    // Id editing
    let uic = ui.clone();
    ui.property_controls.map_id.connect_changed(move |widget| {
        if uic.get_signal_mode() {
            let text = widget.text();
            uic.map.borrow_mut().property.id = text.into();
        }
    });

    let uic = ui.clone();
    ui.property_controls
        .region_map
        .connect_toggled(move |widget| {
            if uic.get_signal_mode() {
                let mode = widget.is_active();
                uic.map.borrow_mut().property.is_region_map = mode;
                uic.iconview.refilter(mode);
            }
        });

    // entrancxe editting
    let uic = ui.clone();
    ui.property_controls
        .entrance_pos
        .connect_changed(move |widget| {
            if uic.get_signal_mode() {
                let text = widget.text();
                uic.map.borrow_mut().property.entrance.clear();
                if let Ok(entrance) = text.parse::<Coords>() {
                    uic.map.borrow_mut().property.entrance.push(entrance);
                }
            }
        });

    // music editting
    let uic = ui.clone();
    ui.property_controls.music.connect_changed(move |widget| {
        if uic.get_signal_mode() {
            let text = widget.text();
            uic.map.borrow_mut().property.music = text.into();
        }
    });

    connect_for_boundary_radio_bottons(ui);
    connect_for_tile_edit_controls(ui);
}

fn connect_for_boundary_radio_bottons(ui: &Ui) {
    let uic = ui.clone();
    ui.property_controls
        .boundary_n_none
        .connect_toggled(move |_| {
            // N
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.n = MapTemplateBoundaryBehavior::None;
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_n_next
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.n = MapTemplateBoundaryBehavior::NextFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_n_prev
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.n = MapTemplateBoundaryBehavior::PrevFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_n_region
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.n = MapTemplateBoundaryBehavior::Exit
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_s_none
        .connect_toggled(move |_| {
            // S
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.s = MapTemplateBoundaryBehavior::None;
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_s_next
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.s = MapTemplateBoundaryBehavior::NextFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_s_prev
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.s = MapTemplateBoundaryBehavior::PrevFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_s_region
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.s = MapTemplateBoundaryBehavior::Exit
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_e_none
        .connect_toggled(move |_| {
            // E
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.e = MapTemplateBoundaryBehavior::None;
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_e_next
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.e = MapTemplateBoundaryBehavior::NextFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_e_prev
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.e = MapTemplateBoundaryBehavior::PrevFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_e_region
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.e = MapTemplateBoundaryBehavior::Exit
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_w_none
        .connect_toggled(move |_| {
            // W
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.w = MapTemplateBoundaryBehavior::None;
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_w_next
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.w = MapTemplateBoundaryBehavior::NextFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_w_prev
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.w = MapTemplateBoundaryBehavior::PrevFloor
            }
        });
    let uic = ui.clone();
    ui.property_controls
        .boundary_w_region
        .connect_toggled(move |_| {
            if uic.get_signal_mode() {
                uic.map.borrow_mut().property.boundary.w = MapTemplateBoundaryBehavior::Exit
            }
        });
}

fn connect_for_tile_edit_controls(ui: &Ui) {
    let uic = ui.clone();
    ui.property_controls
        .button_select_tile_mode
        .connect_clicked(move |_| {
            uic.selected_item.set(SelectedItem::SelectTile);
        });
    let uic = ui.clone();
    ui.property_controls
        .entry_item_id
        .connect_changed(move |widget| {
            if uic.get_signal_mode() {
                let text = widget.text();
                let item_gen = if text == "" {
                    None
                } else {
                    use common::gamedata::ItemGen;
                    Some(ItemGen {
                        id: text.to_string(),
                    })
                };
                uic.map
                    .borrow_mut()
                    .set_item(uic.property_controls.selected_tile.get(), item_gen);
            }
        });
}


use gtk;
use gtk::prelude::*;
use edit_map::EditingMap;
use ui::Ui;

#[derive(Clone)]
pub struct PropertyControls {
    pub map_id: gtk::Entry,
    pub region_map: gtk::CheckButton,
}

impl PropertyControls {
    pub fn build(builder: &gtk::Builder) -> PropertyControls {
        PropertyControls {
            map_id:     get_object!(builder, "property-map-id"),
            region_map: get_object!(builder, "property-region-map"),
        }
    }

    pub fn update(&self, map: &EditingMap) {
        self.map_id.set_text(&map.property.id);
        self.region_map.set_active(map.property.is_region_map);
    }
}

pub fn connect_for_property_controls(ui: &Ui) {
    // Id editing
    let uic = ui.clone(); 
    ui.property_controls.map_id.connect_changed(move |widget| {
        if uic.get_signal_mode() {
            let text = widget.get_text().unwrap_or("".to_owned());
            uic.map.borrow_mut().property.id = text;
        }
    });

    let uic = ui.clone();
    ui.property_controls.region_map.connect_toggled(move |widget| {
        if uic.get_signal_mode() {
            let mode = widget.get_active();
            uic.map.borrow_mut().property.is_region_map = mode;
            uic.iconview.refilter(mode);
        }
    });
}


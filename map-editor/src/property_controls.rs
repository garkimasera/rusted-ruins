
use gtk;
use gtk::prelude::*;
use edit_map::EditingMap;
use ui::Ui;

#[derive(Clone)]
pub struct PropertyControls {
    pub map_id: gtk::Entry,
}

impl PropertyControls {
    pub fn build(builder: &gtk::Builder) -> PropertyControls {
        PropertyControls {
            map_id: get_object!(builder, "property-map-id"),
        }
    }

    pub fn update(&self, map: &EditingMap) {
        self.map_id.set_text(&map.property.id);
    }
}

pub fn connect_for_property_controls(ui: &Ui) {
    { // Id editing
        let uic = ui.clone();
        ui.property_controls.map_id.connect_changed(move |widget| {
            if uic.get_signal_mode() {
                let text = widget.get_text().unwrap_or("".to_owned());
                uic.map.borrow_mut().property.id = text;
            }
        });
    }
}


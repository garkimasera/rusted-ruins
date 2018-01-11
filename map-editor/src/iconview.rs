
use gtk;
use gtk::prelude::*;
use ui::Ui;
use ui::SelectedItem;
use common::objholder::*;

#[derive(Clone)]
pub struct IconView {
    pub iconview_tile:  gtk::IconView,
    pub iconview_wall:  gtk::IconView,
    pub liststore_tile: gtk::ListStore,
    pub liststore_wall: gtk::ListStore,
}

impl IconView {
    pub fn build(builder: &gtk::Builder) -> IconView {
        IconView {
            iconview_tile:  get_object!(builder, "iconview-tile"),
            iconview_wall:  get_object!(builder, "iconview-wall"),
            liststore_tile: get_object!(builder, "liststore-tile"),
            liststore_wall: get_object!(builder, "liststore-wall"),
        }
    }
}

pub fn set_iconview(ui: &Ui) {
    let iconview = &ui.iconview;
    { // Set tile icons
        iconview.iconview_tile.set_pixbuf_column(0);
        iconview.iconview_tile.set_text_column(1);
        let uic = ui.clone();
        iconview.iconview_tile.connect_selection_changed(move |_| {
            if let Some(path) = uic.iconview.iconview_tile.get_selected_items().get(0) {
                uic.tile_selected(path.get_indices()[0]);
            }
        });
    }
    { // Set wall icons
        iconview.iconview_wall.set_pixbuf_column(0);
        iconview.iconview_wall.set_text_column(1);
        let uic = ui.clone();
        iconview.iconview_wall.connect_selection_changed(move |_| {
            if let Some(path) = uic.iconview.iconview_wall.get_selected_items().get(0) {
                uic.wall_selected(path.get_indices()[0]);
            }
        });
    }
    update_liststore(ui);
}

pub fn update_liststore(ui: &Ui) {
    let objholder = ::common::gobj::get_objholder();
    let pbh = &*ui.pbh;
    
    let liststore_tile = &ui.iconview.liststore_tile;
    for (i, tile) in objholder.tile.iter().enumerate() {
        liststore_tile.insert_with_values(
            None,
            &[0, 1],
            &[pbh.get(TileIdx(i as u32)), &tile.id]);
    }
    let liststore_wall = &ui.iconview.liststore_wall;
    for (i, wall) in objholder.wall.iter().enumerate() {
        liststore_wall.insert_with_values(
            None,
            &[0, 1],
            &[pbh.get(WallIdx(i as u32)), &wall.id]);
    }
}

impl Ui {
    fn tile_selected(&self, i: i32) {
        self.selected_item.set(SelectedItem::Tile(TileIdx(i as u32)));
    }

    fn wall_selected(&self, i: i32) {
        self.selected_item.set(SelectedItem::Wall(WallIdx(i as u32)));
    }
}

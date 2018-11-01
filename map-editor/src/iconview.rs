
use std::cell::Cell;
use gtk;
use gtk::prelude::*;
use ui::Ui;
use ui::SelectedItem;
use common::objholder::*;
use common::gobj;

thread_local!(static IS_REGION_MAP: Cell<bool> = Cell::new(false));

#[derive(Clone)]
pub struct IconView {
    pub iconview_tile:  gtk::IconView,
    pub iconview_wall:  gtk::IconView,
    pub iconview_deco:  gtk::IconView,
    pub liststore_tile: gtk::ListStore,
    pub liststore_wall: gtk::ListStore,
    pub liststore_deco: gtk::ListStore,
    pub filter_tile:    gtk::TreeModelFilter,
    pub filter_wall:    gtk::TreeModelFilter,
    pub filter_deco:    gtk::TreeModelFilter,
}

impl IconView {
    pub fn build(builder: &gtk::Builder) -> IconView {
        let liststore_tile = get_object!(builder, "liststore-tile");
        let liststore_wall = get_object!(builder, "liststore-wall");
        let liststore_deco = get_object!(builder, "liststore-deco");
        let filter_tile: gtk::TreeModelFilter = get_object!(builder, "filter-tile");
        let filter_wall: gtk::TreeModelFilter = get_object!(builder, "filter-wall");
        let filter_deco: gtk::TreeModelFilter = get_object!(builder, "filter-deco");
        filter_tile.set_visible_func(|m, i| item_filter(m, i) );
        filter_wall.set_visible_func(|m, i| item_filter(m, i) );
        filter_deco.set_visible_func(|m, i| item_filter(m, i) );
        
        IconView {
            iconview_tile: get_object!(builder, "iconview-tile"),
            iconview_wall: get_object!(builder, "iconview-wall"),
            iconview_deco: get_object!(builder, "iconview-deco"),
            liststore_tile,
            liststore_wall,
            liststore_deco,
            filter_tile,
            filter_wall,
            filter_deco,
        }
    }

    pub fn refilter(&self, is_region_map: bool) {
        IS_REGION_MAP.with(|a| a.set(is_region_map));
        self.filter_tile.refilter();
        self.filter_wall.refilter();
        self.filter_deco.refilter();
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
                let iter = uic.iconview.filter_tile.get_iter(&path).unwrap();
                let id: String = uic.iconview.filter_tile.get_value(&iter, 1).get().unwrap();
                uic.item_selected(SelectedItem::Tile(gobj::id_to_idx::<TileIdx>(&id)));
            }
        });
    }
    { // Set wall icons
        iconview.iconview_wall.set_pixbuf_column(0);
        iconview.iconview_wall.set_text_column(1);
        let uic = ui.clone();
        iconview.iconview_wall.connect_selection_changed(move |_| {
            if let Some(path) = uic.iconview.iconview_wall.get_selected_items().get(0) {
                let iter = uic.iconview.filter_wall.get_iter(&path).unwrap();
                let id: String = uic.iconview.filter_wall.get_value(&iter, 1).get().unwrap();
                uic.item_selected(SelectedItem::Wall(gobj::id_to_idx::<WallIdx>(&id)));
            }
        });
    }
    { // Set deco icons
        iconview.iconview_deco.set_pixbuf_column(0);
        iconview.iconview_deco.set_text_column(1);
        let uic = ui.clone();
        iconview.iconview_deco.connect_selection_changed(move |_| {
            if let Some(path) = uic.iconview.iconview_deco.get_selected_items().get(0) {
                let iter = uic.iconview.filter_deco.get_iter(&path).unwrap();
                let id: String = uic.iconview.filter_deco.get_value(&iter, 1).get().unwrap();
                uic.item_selected(SelectedItem::Deco(gobj::id_to_idx::<DecoIdx>(&id)));
            }
        });
    }
    update_liststore(ui);
    iconview.refilter(false);
}

fn update_liststore(ui: &Ui) {
    let objholder = ::common::gobj::get_objholder();
    let pbh = &*ui.pbh;
    
    let liststore_tile = &ui.iconview.liststore_tile;
    for (i, tile) in objholder.tile.iter().enumerate() {
        liststore_tile.insert_with_values(
            None,
            &[0, 1],
            &[&pbh.get(TileIdx(i as u32)).icon, &tile.id]);
    }
    let liststore_wall = &ui.iconview.liststore_wall;
    for (i, wall) in objholder.wall.iter().enumerate() {
        liststore_wall.insert_with_values(
            None,
            &[0, 1],
            &[&pbh.get(WallIdx(i as u32)).icon, &wall.id]);
    }
    let liststore_deco = &ui.iconview.liststore_deco;
    for (i, deco) in objholder.deco.iter().enumerate() {
        liststore_deco.insert_with_values(
            None,
            &[0, 1],
            &[&pbh.get(DecoIdx(i as u32)).icon, &deco.id]);
    }
}

impl Ui {
    fn item_selected(&self, item: SelectedItem) {
        self.selected_item.set(item);

        let new_text = match item {
            SelectedItem::Tile(idx) => {
                format!("{} (tile)", gobj::idx_to_id(idx))
            }
            SelectedItem::Wall(idx) => {
                format!("{} (wall)", gobj::idx_to_id(idx))
            }
            SelectedItem::Deco(idx) => {
                format!("{} (deco)", gobj::idx_to_id(idx))
            }
            _ => { return; },
        };
        self.label_selected_item.set_text(&new_text);
    }
}

fn item_filter(m: &gtk::TreeModel, i: &gtk::TreeIter) -> bool {
    let id: String = m.get_value(&i, 1).get().unwrap();
    if id == "!" { return true }
    if IS_REGION_MAP.with(|a| a.get()) {
        judge_rm_item(&id)
    } else {
        !judge_rm_item(&id)
    }
}

fn judge_rm_item(id: &str) -> bool {
    if id.starts_with("!rm.") || id.starts_with("rm.") {
        true
    } else {
        false
    }
}


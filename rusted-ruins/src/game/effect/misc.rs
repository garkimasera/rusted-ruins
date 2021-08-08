use crate::game::extrait::*;
use crate::game::Game;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use geom::*;

pub fn gen_item(game: &mut Game<'_>, id: &str, pos: Vec2d) {
    let idx: ItemIdx = if let Some(idx) = gobj::id_to_idx_checked(id) {
        idx
    } else {
        warn!("unknown object id \"{}\"", id);
        return;
    };

    let item_obj = gobj::get_obj(idx);

    if let Some(&ItemObjAttr::Plant {
        required_fertility, ..
    }) = find_attr!(item_obj, ItemObjAttr::Plant)
    {
        if required_fertility > game.gd.get_current_map().tile_fertility(pos) {
            return;
        }
    }

    let item = crate::game::item::gen::gen_item_from_idx(idx, 1);
    game.gd.add_item_on_tile(pos, item, 1);
}

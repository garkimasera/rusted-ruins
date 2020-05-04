use common::gamedata::*;
use common::gobj;

pub fn use_item(gd: &mut GameData, il: ItemLocation, cid: CharaId) {
    let item = gd.get_item(il);
    let item_obj = gobj::get_obj(item.0.idx);

    match item_obj.use_effect {
        UseEffect::None => panic!("use invalid item"),
        UseEffect::Deed => {
            assert_eq!(cid, CharaId::Player);
        }
    }
}

use crate::game::extrait::*;
use common::gamedata::*;

pub fn get_item<T: Into<ItemMoveNum>>(
    gd: &mut GameData,
    item_location: ItemLocation,
    cid: CharaId,
    n: T,
) {
    let item = gd.get_item(item_location);
    let obj = item.0.obj();
    let src_list = gd.get_item_list_mut(item_location.0);
    let n = match n.into() {
        ItemMoveNum::Partial(n) => n,
        ItemMoveNum::All => src_list.get_number(item_location.1),
    };

    // If item is gold and dest is player, increases player money.
    if obj.id == "!gold" {
        gd.remove_item(item_location, n);
        if cid == CharaId::Player {
            gd.player.add_money(n.into());
        }
        return;
    }

    let dest = ItemListLocation::Chara { cid };
    gd.move_item(item_location, dest, n);
    gd.chara.get_mut(cid).update();
}

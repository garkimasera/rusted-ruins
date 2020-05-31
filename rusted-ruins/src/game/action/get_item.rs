use common::gamedata::*;

pub fn get_item<T: Into<ItemMoveNum>>(
    gd: &mut GameData,
    item_location: ItemLocation,
    dest: CharaId,
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
        if dest == CharaId::Player {
            gd.player.add_money(n.into());
        }
        return;
    }

    let dest = ItemListLocation::Chara {
        cid: CharaId::Player,
    };
    gd.move_item(item_location, dest, n);
}

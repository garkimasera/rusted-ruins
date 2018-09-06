
use game::extrait::*;
use common::gamedata::*;

pub fn buy_item(gd: &mut GameData, il: ItemLocation) {
    let price = gd.get_item(il).0.price();
    if gd.player.has_money(price) {
        gd.player.sub_money(price);
        gd.move_item(il, ItemListLocation::Chara { cid: CharaId::Player }, 1);
    }
}

pub fn sell_item(gd: &mut GameData, il: ItemLocation) {
    let price = gd.get_item(il).0.selling_price();
    gd.player.add_money(price);
    gd.remove_item(il, 1);
}


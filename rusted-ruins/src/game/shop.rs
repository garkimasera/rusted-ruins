use crate::game::extrait::*;
use crate::game::item::gen::gen_item_by_level;
use common::gamedata::*;
use rules::RULES;

pub fn buy_item(gd: &mut GameData, il: ItemLocation) {
    let price = gd.get_item(il).0.price();
    if gd.player.has_money(price) {
        gd.player.sub_money(price);
        gd.move_item(
            il,
            ItemListLocation::Chara {
                cid: CharaId::Player,
            },
            1,
        );
    } else {
        game_log_i!("shop-lack-of-money"; chara=gd.chara.get(CharaId::Player));
    }
}

pub fn sell_item(gd: &mut GameData, il: ItemLocation) {
    let price = gd.get_item(il).0.selling_price();
    gd.player.add_money(price);
    gd.remove_item(il, 1);
}

/// Update items on a shop
pub fn update_items_on_shop(shop: &mut Shop) {
    shop.items.clear();

    let n_gen_item = rng::gen_range(RULES.town.min_shop_items, RULES.town.max_shop_items);

    for _ in 0..n_gen_item {
        shop.items.append(gen_shop_item(shop.level, &shop.kind), 1);
    }
}

/// Generate new item at shops
fn gen_shop_item(floor_level: u32, shop_kind: &ShopKind) -> Item {
    let f = |item_obj: &ItemObject| match shop_kind {
        ShopKind::Equipment => match item_obj.kind {
            ItemKind::Weapon(_) | ItemKind::Armor(_) => 1.0,
            _ => 0.0,
        },
        ShopKind::Potion => match item_obj.kind {
            ItemKind::Potion => 1.0,
            _ => 0.0,
        },
        ShopKind::Food => match item_obj.kind {
            ItemKind::Food => 1.0,
            _ => 0.0,
        },
    };
    gen_item_by_level(floor_level, f, true)
}

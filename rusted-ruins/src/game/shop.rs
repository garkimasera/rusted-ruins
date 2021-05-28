use crate::game::extrait::*;
use crate::game::item::gen::gen_item_by_level;
use common::gamedata::*;
use common::item_selector::ItemSelector;
use common::sitegen::ShopGenData;
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
        gd.chara.get_mut(CharaId::Player).update();
    } else {
        game_log_i!("shop-lack-of-money"; chara=gd.chara.get(CharaId::Player));
    }
}

pub fn sell_item(gd: &mut GameData, il: ItemLocation) {
    let price = gd.get_item(il).0.selling_price();
    gd.player.add_money(price);
    gd.remove_item(il, 1);
    gd.chara.get_mut(CharaId::Player).update();
}

/// Update items on a shop
pub fn update_items_on_shop(shop: &mut Shop, shop_gen: &ShopGenData) {
    shop.items.clear();

    let n_gen_item = rng::gen_range(RULES.town.min_shop_items..RULES.town.max_shop_items);

    for _ in 0..n_gen_item {
        if let Some(item) = gen_shop_item(shop.level, shop_gen) {
            shop.items.append(item, 1);
        }
    }
}

/// Generate new item at shops
fn gen_shop_item(floor_level: u32, shop_gen: &ShopGenData) -> Option<Item> {
    let item_selector: ItemSelector = if shop_gen.shop_kind.is_empty() {
        shop_gen
            .selector
            .parse()
            .unwrap_or_else(|e| panic!("invalid selector for shop in site_gen object\n{}", e))
    } else {
        RULES
            .town
            .shop_kinds
            .get(&shop_gen.shop_kind)
            .cloned()
            .unwrap_or_else(|| panic!("unknown shop kind\n{}", shop_gen.shop_kind))
    };
    let f = |item_obj: &ItemObject| {
        if item_selector.is(item_obj) {
            1.0
        } else {
            0.0
        }
    };
    gen_item_by_level(floor_level, f, true)
}

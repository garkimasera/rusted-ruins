use crate::game::extrait::*;
use crate::game::item::gen::gen_item_by_level;
use common::gamedata::*;
use common::obj::SiteGenObject;
use common::sitegen::NpcGenId;
use common::sitegen::ShopGenData;
use rules::RULES;

pub fn buy_item(gd: &mut GameData, il: ItemLocation) {
    let player = gd.chara.get(CharaId::Player);
    let player_negotiation = player.skill_level(SkillKind::Negotiation);
    let player_lv = player.lv;
    let price = gd.get_item(il).0.buying_price(player_negotiation);
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

        let exp = std::cmp::min(
            price as u32 / ((player_negotiation + 1) * 5),
            RULES.exp.negotiation_max,
        );
        gd.chara.get_mut(CharaId::Player).add_skill_exp(
            SkillKind::Negotiation,
            exp as u32,
            player_lv,
        );
    } else {
        game_log!("shop-lack-of-money"; chara=gd.chara.get(CharaId::Player));
    }
}

pub fn sell_item(gd: &mut GameData, il: ItemLocation) {
    let player = gd.chara.get(CharaId::Player);
    let player_negotiation = player.skill_level(SkillKind::Negotiation);
    let player_lv = player.lv;
    let price = gd.get_item(il).0.selling_price(player_negotiation);
    gd.player.add_money(price);
    gd.remove_item(il, 1);
    gd.chara.get_mut(CharaId::Player).update();

    let exp = std::cmp::min(
        price as u32 / ((player_negotiation + 1) * 5),
        RULES.exp.negotiation_max,
    );
    gd.chara
        .get_mut(CharaId::Player)
        .add_skill_exp(SkillKind::Negotiation, exp as u32, player_lv);
}

/// Update shop states
pub fn update_shops(gd: &mut GameData, sid: SiteId, sg: &SiteGenObject) {
    let site = gd.region.get_site_mut(sid);
    for npc_gen_id in sg.shops.keys() {
        let shop = Shop {
            items: ItemList::default(),
            level: 1,
            custom_shop_gen: None,
        };
        let cid = match *npc_gen_id {
            NpcGenId::Site(id) => CharaId::OnSite { sid, id },
            NpcGenId::Unique(id) => CharaId::Unique { id },
        };
        site.add_shop(cid, shop);
    }

    for (cid, shop) in site.iter_shops_mut() {
        let shop_gen = if let Some(shop_gen) = get_shop_gen(*cid, sid, sg) {
            shop_gen
        } else if let Some(shop_gen) = &shop.custom_shop_gen {
            shop_gen.clone()
        } else {
            continue;
        };
        update_items_on_shop(shop, &shop_gen);
    }
}

fn get_shop_gen(cid: CharaId, sid: SiteId, sg: &SiteGenObject) -> Option<ShopGenData> {
    let npc_gen_id = match cid {
        CharaId::OnSite { sid: sid_a, id } => {
            if sid != sid_a {
                return None;
            }
            NpcGenId::Site(id)
        }
        CharaId::Unique { id } => NpcGenId::Unique(id),
        _ => {
            return None;
        }
    };
    sg.shops.get(&npc_gen_id).cloned()
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
    let item_selector = if shop_gen.shop_kind.is_empty() {
        shop_gen.item_selector.clone()
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

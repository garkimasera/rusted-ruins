use crate::game::effect::do_effect;
use crate::game::target::Target;
use crate::game::{Game, InfoGetter};
use common::gamedata::*;
use common::gobj;

pub fn use_item(game: &mut Game<'_>, il: ItemLocation, cid: CharaId, target: Target) {
    let item = game.gd.get_item(il);
    let item_obj = gobj::get_obj(item.0.idx);

    let use_effect = item_obj
        .attrs
        .iter()
        .filter_map(|attr| match attr {
            ItemObjAttr::Use(use_effect) => Some(use_effect),
            _ => None,
        })
        .next()
        .unwrap();

    match use_effect {
        UseEffect::Effect(effect) => {
            let mut effect = effect.clone();
            for effect_kind in &mut effect.kind {
                if let EffectKind::SkillLearning { skills } = effect_kind {
                    for attr in &item.0.attrs {
                        if let ItemAttr::SkillLearning(skill_kind) = attr {
                            skills.push(*skill_kind);
                        }
                    }
                }
            }
            do_effect(game, &effect, Some(cid), target, 1.0, 1.0);
        }
        UseEffect::Deed => {
            if !use_deed(game) {
                return;
            }
        }
        UseEffect::Seed { .. } => {}
    }

    game.gd.remove_item(il, 1);
}

fn use_deed(game: &mut Game<'_>) -> bool {
    let gd = &mut game.gd;
    let mapid = gd.get_current_mapid();
    if !mapid.is_region_map() {
        game_log_i!("use_item-deed-invalid-map");
        return false;
    }

    let pos = gd.player_pos();
    let map = gd.get_current_map();
    if !map.tile[pos].special.is_none() {
        game_log_i!("use_item-deed-occupied");
    }

    let mut site = Site::new(1, None);
    site.content = SiteContent::Player {
        kind: PlayerBaseKind::Normal,
    };
    let rid = mapid.rid();
    let sid = gd.add_site(site, SiteKind::Player, rid, Some(pos)).unwrap();

    let map_random_id = crate::game::saveload::gen_box_id(gd);
    let map = if let Some(map) = crate::game::map::wilderness::generate_wilderness(gd, pos) {
        map
    } else {
        return false;
    };
    gd.add_map(map, sid, map_random_id);

    let map = gd.get_current_map_mut();
    map.tile[pos].special = SpecialTileKind::SiteSymbol {
        kind: SiteSymbolKind::from("!rm-h0"),
    };
    game_log_i!("use_item-deed-succeed");
    true
}

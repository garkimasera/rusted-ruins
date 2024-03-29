use crate::config::changeable::game_log_cfg;
use crate::damage_popup::PopupKind;
use crate::game::extrait::*;
use crate::game::{ClosureTrigger, Game, InfoGetter};
use common::gamedata::*;
use common::gobj;
use rules::RULES;

use super::item::gen::{choose_item_by_item_selector, gen_item_from_idx};

#[derive(Clone, Copy)]
pub enum CharaDamageKind {
    MeleeAttack,
    RangedAttack,
    Explosion,
    Direct,
    Poison,
    Starve,
    Encumbrance,
}

/// Give damage to a character.
pub fn do_damage(
    game: &mut Game,
    cid: CharaId,
    damage: i32,
    damage_kind: CharaDamageKind,
    origin: Option<CharaId>,
) -> i32 {
    let origin_faction = origin.map(|origin| game.gd.chara.get(origin).faction);
    let pos = game.gd.chara_pos(cid);
    let chara = game.gd.chara.get_mut(cid);
    let faction = chara.faction;

    chara.hp -= damage;

    // Damage log
    if game_log_cfg().combat_log.damage() {
        game_log_i!("damaged-chara"; chara=chara, damage=damage);
    }

    if let Some(pos) = pos {
        crate::damage_popup::push(cid, pos, PopupKind::Damage(damage));
    } else {
        error!("damage to character that is not on map");
    }

    let chara_hp = chara.hp;

    if chara_hp > 0 {
        // Faction process
        if let (Some(origin), Some(origin_faction)) = (origin, origin_faction) {
            if !chara.ai.state.is_combat() {
                chara.ai.state = AiState::Combat { target: origin };

                if origin_faction.is_player() && !faction.is_player() {
                    game_log!("npc-get-hostile"; chara=chara);
                    let target_faction = chara.faction;
                    game.gd
                        .faction
                        .change(target_faction, RULES.faction.relvar_attacked);
                }
            }
        }
    } else {
        // Logging
        match damage_kind {
            CharaDamageKind::MeleeAttack => {
                game_log_i!("killed-by-melee-attack"; chara=chara);
            }
            CharaDamageKind::RangedAttack => {
                game_log_i!("killed-by-ranged-attack"; chara=chara);
            }
            CharaDamageKind::Explosion => {
                game_log_i!("killed-by-explosion"; chara=chara);
            }
            CharaDamageKind::Direct => {
                game_log_i!("killed"; chara=chara);
            }
            CharaDamageKind::Poison => {
                game_log_i!("killed-by-poison-damage"; chara=chara);
            }
            CharaDamageKind::Starve => {
                game_log_i!("killed-by-starve-damage"; chara=chara);
            }
            CharaDamageKind::Encumbrance => {
                game_log_i!("killed-by-encumbrance-damage"; chara=chara);
            }
        }

        process_dying(game, cid, origin);
    }
    chara_hp
}

fn process_dying(game: &mut Game, cid: CharaId, origin: Option<CharaId>) {
    let pos = game.gd.chara_pos(cid);

    // Faction relation
    if origin == Some(CharaId::Player) && Some(cid) != origin {
        let chara = game.gd.chara.get(cid);
        let target_faction = chara.faction;

        game.gd
            .faction
            .change(target_faction, RULES.faction.relvar_killed);
    }

    // Item dropping
    let hunting_skill = if let Some(origin) = origin {
        game.gd.chara.get(origin).skill_level(SkillKind::Hunting)
    } else {
        0
    };
    let level = game.gd.chara.get(cid).lv;

    let ct = gobj::get_obj(game.gd.chara.get(cid).idx);

    for drop_item in &ct.drop_items {
        let probability = if drop_item.hunting {
            (level as f32 + 10.0) / (hunting_skill as f32 + 10.0)
        } else {
            drop_item.probability / 100.0
        };

        if !rng::gen_bool(probability) {
            continue;
        }

        let item_idx =
            if let Some(item_idx) = choose_item_by_item_selector(&drop_item.item_selector) {
                item_idx
            } else {
                continue;
            };

        let mut item = gen_item_from_idx(item_idx, level);
        item.quality.base += drop_item.quality_bonus;

        if let Some(pos) = pos {
            game.push_closure(
                ClosureTrigger::CharaRemove(cid),
                Box::new(move |game: &mut Game| {
                    game.gd.get_current_map_mut().locate_item(item, pos, 1);
                }),
            );
        }
    }
}

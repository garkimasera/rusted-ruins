//! Process characters action

pub mod get_item;
pub mod harvest;
pub mod use_item;

use super::combat;
use super::extrait::*;
use super::{Game, InfoGetter};
use common::gamedata::*;
use geom::*;
use rng::dice;
use rules::RULES;

pub fn try_move(game: &mut Game, chara_id: CharaId, dir: Direction) -> bool {
    if dir.as_vec() == (0, 0) {
        return true;
    } // Move to current tile always success
    let dest_tile = game.gd.get_current_map().chara_pos(chara_id).unwrap() + dir.as_vec();

    if !game
        .gd
        .get_current_map()
        .is_passable(game.gd.chara.get(chara_id), dest_tile)
    {
        return false;
    }

    let other_chara = game.gd.get_current_map().get_chara(dest_tile);
    if other_chara.is_none() {
        game.gd.get_current_map_mut().move_chara(chara_id, dir);
        if chara_id == CharaId::Player {
            game.anim_queue.push_player_move(dir);
        }
    } else {
        let relation = game.gd.chara_relation(chara_id, other_chara.unwrap());

        match relation {
            Relationship::ALLY | Relationship::FRIENDLY | Relationship::NEUTRAL => {
                {
                    let current_map = game.gd.get_current_map_mut();
                    current_map.move_chara(chara_id, dir);
                }
                if chara_id == CharaId::Player {
                    game.anim_queue.push_player_move(dir);
                }
            }
            Relationship::HOSTILE => {
                combat::attack_neighbor(game, chara_id, other_chara.unwrap());
            }
        }
    }
    true
}

/// Shot target
pub fn shot_target(game: &mut Game, cid: CharaId, target: CharaId) -> bool {
    combat::shot_target(game, cid, target)
}

/// Drink one item
pub fn drink_item(game: &mut Game, il: ItemLocation, cid: CharaId) {
    let gd = &mut game.gd;
    let item = gd.remove_item_and_get(il, 1); // Decrease the number of item by 1
    let item_obj = item.obj();

    let chara = gd.chara.get_mut(cid);
    game_log!("drink-item"; chara=chara, item=item);

    let eff: i32 = item_obj.eff.into();
    apply_medical_effect(game, cid, &item_obj.medical_effect, eff);
}

/// Eat one item
pub fn eat_item(game: &mut Game, il: ItemLocation, cid: CharaId) {
    let gd = &mut game.gd;
    let item = gd.remove_item_and_get(il, 1); // Decrease the number of item by 1
    let item_obj = item.obj();

    let chara = gd.chara.get_mut(cid);
    game_log!("eat-item"; chara=chara, item=item);
    let nutrition: f32 = item_obj.nutrition.into();
    chara.add_sp(nutrition * RULES.chara.sp_nutrition_factor, cid);

    let eff: i32 = item_obj.eff.into();
    apply_medical_effect(game, cid, &item_obj.medical_effect, eff);
}

pub fn release_item(game: &mut Game, il: ItemLocation, cid: CharaId) {
    let mut item = game.gd.remove_item_and_get(il, 1);
    let item_obj = item.obj();
    let item_dice: f64 = dice(item_obj.dice_n, item_obj.dice_x).into();

    match item.charge() {
        Some(n) if n >= 1 => {
            let skill_level: f64 = game
                .gd
                .chara
                .get(cid)
                .skills
                .get(SkillKind::MagicDevice)
                .into();
            let power =
                (skill_level / 10.0 + 1.0) * item_dice * RULES.magic.magic_device_base_power;
            if let Some(effect) = item_obj.magical_effect.as_ref() {
                super::effect::process_effect(game, effect, Some(cid), None, power);
            } else {
                return;
            }
            //            super::magic::do_magic(game, cid, item_obj.magical_effect, power);
            *item.charge_mut().unwrap() = n - 1;
        }
        _ => (),
    }

    game.gd.get_item_list_mut(il.0).append(item, 1);
}

fn apply_medical_effect(game: &mut Game, cid: CharaId, effect: &Option<Effect>, eff: i32) {
    if effect.is_none() {
        return;
    }
    let effect = effect.as_ref().unwrap();
    super::effect::process_effect(game, effect, Some(cid), None, eff.into());
}

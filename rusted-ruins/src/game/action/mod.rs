//! Process characters action

pub mod ability;
pub mod get_item;
pub mod harvest;
pub mod use_item;

use super::damage::*;
use super::effect::{do_effect, melee_attack_effect, ranged_attack_effect};
use super::extrait::*;
use super::power::{calc_hit, calc_power};
use super::target::Target;
use super::{Game, InfoGetter};
use common::gamedata::*;
use geom::*;
use rules::RULES;

pub fn try_move(game: &mut Game, cid: CharaId, dir: Direction) -> bool {
    // Move to current tile always success
    if dir.as_coords() == (0, 0) {
        return true;
    }

    if cid == CharaId::Player {
        super::time::set_player_moved();
    }

    let dest_tile = game.gd.get_current_map().chara_pos(cid).unwrap() + dir.as_coords();

    if !game
        .gd
        .get_current_map()
        .is_passable(game.gd.chara.get(cid), dest_tile)
    {
        return false;
    }

    if let Some(other_chara) = game.gd.get_current_map().get_chara(dest_tile) {
        let relation = game.gd.chara_relation(cid, other_chara);

        match relation {
            Relationship::Ally | Relationship::Friendly | Relationship::Neutral => {
                let current_map = game.gd.get_current_map_mut();
                if current_map.tile[dest_tile].chara != Some(CharaId::Player) {
                    current_map.move_chara(cid, dir);
                }
                if cid == CharaId::Player {
                    game.anim_queue.push_player_move(dir);
                }
            }
            Relationship::Hostile => {
                melee_attack(game, cid, other_chara);
            }
        }
    } else {
        game.gd.get_current_map_mut().move_chara(cid, dir);
        if cid == CharaId::Player {
            game.anim_queue.push_player_move(dir);
        }
    }

    true
}

/// Melee attack
pub fn melee_attack(game: &mut Game, cid: CharaId, target: CharaId) {
    let (effect, power_calc) = melee_attack_effect(&game.gd, cid);
    let power = calc_power(&game.gd, cid, &power_calc);
    let hit = calc_hit(&game.gd, cid, &power_calc);

    do_effect(game, &effect, Some(cid), target, power, hit);

    // Exp processing
    let target_level = game.gd.chara.get(target).lv;
    let attacker = game.gd.chara.get_mut(cid);
    let skill_kind = match power_calc {
        PowerCalcMethod::BareHands => SkillKind::BareHands,
        PowerCalcMethod::Melee(weapon_kind) => weapon_kind.into(),
        _ => unreachable!(),
    };
    attacker.add_attack_exp(skill_kind, target_level);
}

/// Shoot target
pub fn shoot_target(game: &mut Game, cid: CharaId, target: CharaId) -> bool {
    if !game.gd.target_visible(cid, target) || cid == target {
        return false;
    }

    let (effect, power_calc) = if let Some(result) = ranged_attack_effect(&game.gd, cid) {
        result
    } else {
        return false;
    };
    let power = calc_power(&game.gd, cid, &power_calc);
    let hit = calc_hit(&game.gd, cid, &power_calc);
    do_effect(game, &effect, Some(cid), target, power, hit);

    // Exp processing
    let target_level = game.gd.chara.get(target).lv;
    let attacker = game.gd.chara.get_mut(cid);
    let skill_kind = match power_calc {
        PowerCalcMethod::Ranged(weapon_kind) => weapon_kind.into(),
        _ => unreachable!(),
    };
    attacker.add_attack_exp(skill_kind, target_level);

    true
}

/// Throw one item
pub fn throw_item(game: &mut Game, il: ItemLocation, cid: CharaId, target: Target) {
    let gd = &mut game.gd;
    let effect = crate::game::item::throw::item_to_throw_effect(gd, il, cid);
    let item = gd.remove_item_and_get(il, 1);

    let power_calc = PowerCalcMethod::Throw(item.w());
    let power = calc_power(gd, cid, &power_calc);
    let hit = calc_hit(gd, cid, &power_calc);

    let chara = gd.chara.get(cid);
    game_log_i!("throw-item"; chara=chara, item=item);
    super::effect::do_effect(game, &effect, Some(cid), target, power, hit);

    // Exp processing
    let target_level = match target {
        Target::Chara(cid) => game.gd.chara.get(cid).lv,
        _ => 1,
    };
    let attacker = game.gd.chara.get_mut(cid);
    attacker.add_attack_exp(SkillKind::Throwing, target_level);
}

/// Drink one item
pub fn drink_item(game: &mut Game, il: ItemLocation, cid: CharaId) {
    let gd = &mut game.gd;
    let item = gd.remove_item_and_get(il, 1); // Decrease the number of item by 1

    let chara = gd.chara.get_mut(cid);
    game_log_i!("drink-item"; chara=chara, item=item);

    if let Some(ItemObjAttr::Medical { effect }) = find_attr!(item.obj(), ItemObjAttr::Medical) {
        let power = calc_power(&game.gd, cid, &PowerCalcMethod::Medical) * item.power_factor();
        let hit = calc_hit(&game.gd, cid, &PowerCalcMethod::Medical);
        super::effect::do_effect(game, effect, None, cid, power, hit);
    }
}

/// Eat one item
pub fn eat_item(game: &mut Game, il: ItemLocation, cid: CharaId) {
    let gd = &mut game.gd;
    let item = gd.remove_item_and_get(il, 1); // Decrease the number of item by 1
    let item_obj = item.obj();

    let chara = gd.chara.get_mut(cid);
    game_log_i!("eat-item"; chara=chara, item=item);

    let nutrition: f32 = if let Some(&ItemObjAttr::Nutrition(nutrition)) =
        find_attr!(item_obj, ItemObjAttr::Nutrition)
    {
        nutrition as f32
    } else {
        0.0
    };

    if let Some(damage) = chara.add_sp(nutrition * RULES.chara.sp_nutrition_factor, cid) {
        do_damage(game, cid, damage, CharaDamageKind::Starve, None);
    }

    if let Some(ItemObjAttr::Medical { effect, .. }) = find_attr!(item.obj(), ItemObjAttr::Medical)
    {
        let power = calc_power(&game.gd, cid, &PowerCalcMethod::Medical) * item.power_factor();
        let hit = calc_hit(&game.gd, cid, &PowerCalcMethod::Medical);
        super::effect::do_effect(game, effect, None, cid, power, hit);
    }
}

pub fn release_item(game: &mut Game, il: ItemLocation, cid: CharaId, target: Target) {
    let mut item = game.gd.remove_item_and_get(il, 1);
    let item_obj = item.obj();

    match item.charge() {
        Some(n) if n >= 1 => {
            if let Some(ItemObjAttr::Release { effect, .. }) =
                find_attr!(item_obj, ItemObjAttr::Release)
            {
                let power =
                    calc_power(&game.gd, cid, &PowerCalcMethod::Release) * item.power_factor();
                let hit = calc_hit(&game.gd, cid, &PowerCalcMethod::Release);
                super::effect::do_effect(game, effect, Some(cid), target, power, hit);
            } else {
                return;
            }
            *item.charge_mut().unwrap() = n - 1;
        }
        _ => (),
    }

    game.gd.get_item_list_mut(il.0).append(item, 1);
}

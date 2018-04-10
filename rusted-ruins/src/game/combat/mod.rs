
use rng;
use super::Game;
use super::chara::CharaEx;
use super::skill::SkillListEx;
use common::gobj;
use common::gamedata::chara::{CharaId, Chara};
use common::gamedata::skill::SkillKind;
use common::gamedata::item::*;

pub enum DamageKind {
    ShortRangeAttack,
    LongRangeAttack,
    Poison,
}

pub fn attack_neighbor(game: &mut Game, attacker: CharaId, target: CharaId) {
    // Damage calculation
    let (damage, weapon_kind) = {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        let weapon_data = get_weapon_data(attacker);
        let weapon_dice_result = rng::dice(weapon_data.dice_n, weapon_data.dice_x);

        let damage_coef = 256 + attacker.params.str as i32 * 16;

        let damage = weapon_dice_result.saturating_mul(damage_coef) / 256;
        let defence_power = calc_defence(target);
        (if damage < defence_power { 0 }else{ damage - defence_power }, weapon_data.kind)
    };
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("attack"; attacker=attacker.get_name(), target=target.get_name(), damage=damage);
    }
    // Damage processing
    super::chara::damage(game, target, damage, DamageKind::ShortRangeAttack);
    // Exp processing
    {
        let attacker = game.gd.chara.get_mut(attacker);
        if let Some(weapon_kind) = weapon_kind {
            attacker.skills.add_exp(SkillKind::Weapon(weapon_kind), 1);
        }
    }
    // Animation pushing
    game.anim_queue.push_attack(game.gd.get_current_map().chara_pos(target).unwrap());
}

/// Shot target by long range weapons.
/// If attacker actually do actions, returns true.
pub fn shot_target(game: &mut Game, attacker: CharaId, target: CharaId) -> bool {
    // Damage calculation
    let (damage, weapon_kind) = {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        let weapon = if let Some(weapon) = attacker.equip.item(EquipSlotKind::LongRangeWeapon, 0) {
            weapon
        } else { // If this chara doesn't equip long range weapon
            game_log_i!("no-long-range-weapon-equipped");
            return false;
        };
        let weapon_obj = gobj::get_obj(weapon.idx);
        let weapon_kind = get_weapon_kind(weapon_obj);
        let weapon_dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);

        let damage_coef = 256 + attacker.params.dex as i32 * 16;

        let damage = weapon_dice_result.saturating_mul(damage_coef) / 256;
        let defence_power = calc_defence(target);
        (if damage < defence_power { 0 }else{ damage - defence_power }, weapon_kind)
    };
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("shot-target"; attacker=attacker.get_name(), target=target.get_name(), damage=damage);
    }
    // Damage processing
    super::chara::damage(game, target, damage, DamageKind::LongRangeAttack);
    // Exp processing
    {
        let attacker = game.gd.chara.get_mut(attacker);
        attacker.skills.add_exp(SkillKind::Weapon(weapon_kind), 1);
    }
    // Animation pushing
    true
}

fn get_weapon_kind(item: &ItemObject) -> WeaponKind {
    match item.kind {
        ItemKind::Weapon(kind) => kind,
        _ => unreachable!(),
    }
}

struct WeaponData {
    dice_n: i32,
    dice_x: i32,
    /// For hand attack, this is None
    kind: Option<WeaponKind>,
}

fn get_weapon_data(chara: &Chara) -> WeaponData {
    if let Some(weapon) = chara.equip.item(EquipSlotKind::ShortRangeWeapon, 0) {
        let item_obj = gobj::get_obj(weapon.idx);
        let kind = match item_obj.kind {
            ItemKind::Weapon(kind) => kind,
            _ => unreachable!(),
        };
        WeaponData {
            dice_n: item_obj.dice_n.into(),
            dice_x: item_obj.dice_x.into(),
            kind: Some(kind),
        }
    } else {
        WeaponData {
            dice_n: 4, dice_x: 4,
            kind: None,
        }
    }
}

fn calc_defence(chara: &Chara) -> i32 {
    let armor_power = if let Some(armor) = chara.equip.item(EquipSlotKind::BodyArmor, 0) {
        let item_obj = gobj::get_obj(armor.idx);
        item_obj.def
    } else {
        0
    };
    armor_power.into()
}

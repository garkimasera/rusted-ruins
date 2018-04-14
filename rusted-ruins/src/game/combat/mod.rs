
use rng;
use super::Game;
use super::chara::CharaEx;
use super::skill::SkillListEx;
use common::gobj;
use common::gamedata::*;

pub enum DamageKind {
    ShortRangeAttack,
    LongRangeAttack,
    Poison,
}

/// Attack neighbor enemy by short range weapon or bare hands
pub fn attack_neighbor(game: &mut Game, attacker: CharaId, target: CharaId) {
    let skill_kind;
    
    // Damage calculation
    let damage = {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        
        if let Some(weapon) = attacker.equip.item(EquipSlotKind::ShortRangeWeapon, 0) {
            let weapon_obj = gobj::get_obj(weapon.idx);
            let weapon_kind = get_weapon_kind(weapon_obj);
            skill_kind = SkillKind::Weapon(weapon_kind);
            
            let dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);
            let damage_coef = 256 + attacker.params.str as i32 * 16;
            let damage = dice_result.saturating_mul(damage_coef) / 256;
            let defence_power = calc_defence(target);

            if damage < defence_power { 0 } else { damage - defence_power }
        } else { // Attack by bare hands
            skill_kind = SkillKind::BareHands;
            let dice_result = rng::dice(1, 1);
            let damage_coef = 256 + attacker.params.str as i32 * 16;
            let damage = dice_result.saturating_mul(damage_coef) / 256;
            let defence_power = calc_defence(target);

            if damage < defence_power { 0 } else { damage - defence_power }
        }
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
        attacker.skills.add_exp(skill_kind, 1);
    }
    // Animation pushing
    game.anim_queue.push_attack(game.gd.get_current_map().chara_pos(target).unwrap());
}

/// Shot target by long range weapons.
/// If attacker actually do actions, returns true.
pub fn shot_target(game: &mut Game, attacker: CharaId, target: CharaId) -> bool {
    // Damage calculation
    let (damage, weapon_kind, attacker_pos, target_pos) = {
        let attacker_pos = game.gd.get_current_map().chara_pos(attacker).unwrap();
        let target_pos = game.gd.get_current_map().chara_pos(target).unwrap();
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
        (
            if damage < defence_power { 0 }else{ damage - defence_power }, weapon_kind,
            attacker_pos, target_pos         
        )
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
    game.anim_queue.push_shot(attacker_pos, target_pos);
    true
}

fn get_weapon_kind(item: &ItemObject) -> WeaponKind {
    match item.kind {
        ItemKind::Weapon(kind) => kind,
        _ => unreachable!(),
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

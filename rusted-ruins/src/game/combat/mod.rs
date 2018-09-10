
use rng;
use super::Game;
use super::chara::CharaEx;
use super::skill::SkillListEx;
use common::gobj;
use common::gamedata::*;
use rules::RULES;

pub enum DamageKind {
    MeleeAttack,
    RangedAttack,
    Poison,
}

/// Attack neighbor enemy by short range weapon or bare hands
pub fn attack_neighbor(game: &mut Game, attacker: CharaId, target: CharaId) {
    let skill_kind;
    
    // Damage calculation
    let damage = {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        
        if let Some(weapon) = attacker.equip.item(EquipSlotKind::MeleeWeapon, 0) {
            let weapon_obj = gobj::get_obj(weapon.idx);
            let weapon_kind = get_weapon_kind(weapon_obj);
            skill_kind = SkillKind::Weapon(weapon_kind);
            
            let dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);
            let weapon_skill_level = attacker.skills.get(skill_kind);
            let attack_power = calc_attack_power(dice_result, attacker.params.str, weapon_skill_level);
            let equip_defence = calc_equip_defence(target, Element::Physical);
            let defence_skill_level = target.skills.get(SkillKind::Defence);
            let defence_power = calc_defence_power(equip_defence, target.params.vit, defence_skill_level);
            (attack_power / defence_power) as i32
        } else { // Attack by bare hands
            skill_kind = SkillKind::BareHands;
            let dice_result = rng::dice(1, 1);
            let weapon_skill_level = attacker.skills.get(skill_kind);
            let attack_power = calc_attack_power(dice_result, attacker.params.str, weapon_skill_level);
            let equip_defence = calc_equip_defence(target, Element::Physical);
            let defence_skill_level = target.skills.get(SkillKind::Defence);
            let defence_power = calc_defence_power(equip_defence, target.params.vit, defence_skill_level);
            (attack_power / defence_power) as i32
        }
    };
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("attack"; attacker=attacker.get_name(), target=target.get_name(), damage=damage);
    }
    // Damage processing
    super::chara::damage(game, target, damage, DamageKind::MeleeAttack);
    // Exp processing
    {
        let target_level = game.gd.chara.get(target).base_params.level;
        let attacker = game.gd.chara.get_mut(attacker);
        attacker.add_skill_exp(
            skill_kind,
            RULES.exp.attack,
            target_level);
    }
    // Animation pushing
    game.anim_queue.push_attack(game.gd.get_current_map().chara_pos(target).unwrap());
    // Sound effect
    ::audio::play_sound("punch");
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
        let weapon = if let Some(weapon) = attacker.equip.item(EquipSlotKind::RangedWeapon, 0) {
            weapon
        } else { // If this chara doesn't equip long range weapon
            game_log_i!("no-ranged-weapon-equipped");
            return false;
        };
        let weapon_obj = gobj::get_obj(weapon.idx);
        let weapon_kind = get_weapon_kind(weapon_obj);
        let dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);
        
        let weapon_skill_level = attacker.skills.get(SkillKind::Weapon(weapon_kind));
        let attack_power = calc_attack_power(dice_result, attacker.params.dex, weapon_skill_level);
        let equip_defence = calc_equip_defence(target, Element::Physical);
        let defence_skill_level = target.skills.get(SkillKind::Defence);
        let defence_power = calc_defence_power(equip_defence, target.params.vit, defence_skill_level);
        let damage = (attack_power / defence_power) as i32;
        
        (damage, weapon_kind, attacker_pos, target_pos)
    };
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("shot-target"; attacker=attacker.get_name(), target=target.get_name(), damage=damage);
    }
    // Damage processing
    super::chara::damage(game, target, damage, DamageKind::RangedAttack);
    // Exp processing
    {
        let target_level = game.gd.chara.get(target).base_params.level;
        let attacker = game.gd.chara.get_mut(attacker);
        attacker.add_skill_exp(
            SkillKind::Weapon(weapon_kind),
            RULES.exp.attack,
            target_level);
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

fn calc_equip_defence(chara: &Chara, _e: Element) -> f64 {
    10.0
}

/// Calculate attack power
fn calc_attack_power(dice: i32, chara_param: u16, skill_level: u16) -> f64 {
    assert!(dice > 0);
    assert!(chara_param > 0);
    let dice = dice as f64;
    let chara_param = chara_param as f64;
    let skill_level = skill_level as f64;
    
    dice * chara_param * chara_param * (skill_level + 8.0).powf(1.5)
}

/// Calculate defence power
fn calc_defence_power(equip_defence: f64, chara_param: u16, skill_level: u16) -> f64 {
    assert!(equip_defence > 0.0);
    assert!(chara_param > 0);
    let chara_param = chara_param as f64;
    let skill_level = skill_level as f64;

    (equip_defence + 16.0) * chara_param * (skill_level + 8.0)
}

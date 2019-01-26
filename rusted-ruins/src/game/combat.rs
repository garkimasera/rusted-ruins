
use crate::rng;
use super::Game;
use super::chara::CharaEx;
use common::gobj;
use common::gamedata::*;
use rules::RULES;
use rng::Rng;

pub enum DamageKind {
    MeleeAttack,
    RangedAttack,
    Poison,
}

pub struct AttackParams {
    attacker: Option<CharaId>,
    kind: DamageKind,
    element: Element,
    attack_power: f64,
}

/// Attack neighbor enemy by short range weapon or martial arts
pub fn attack_neighbor(game: &mut Game, attacker: CharaId, target: CharaId) {
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("attack"; attacker=attacker, target=target);
    }
    // Animation pushing
    game.anim_queue.push_attack(game.gd.get_current_map().chara_pos(target).unwrap());
    // Judges hit or miss
    {
        let attacker = game.gd.chara.get(attacker);
        let accuracy_power = calc_accuracy_power(
            1,
            1,
            attacker.attr.dex);
        if !hit_judge(&game.gd, accuracy_power, target, DamageKind::MeleeAttack) {
            return;
        }
    }

    let skill_kind;

    // Damage calculation
    let attack_power = {
        let attacker = game.gd.chara.get(attacker);
        
        if let Some(weapon) = attacker.equip.item(EquipSlotKind::MeleeWeapon, 0) {
            let weapon_obj = gobj::get_obj(weapon.idx);
            let weapon_kind = get_weapon_kind(weapon_obj);
            skill_kind = SkillKind::Weapon(weapon_kind);
            
            let dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);
            let weapon_skill_level = attacker.skills.get(skill_kind);
            calc_attack_power(dice_result, attacker.attr.str, weapon_skill_level)
        } else { // Attack by martial arts
            skill_kind = SkillKind::MartialArts;
            let weapon_skill_level = attacker.skills.get(skill_kind);
            let dice_result = rng::dice(1, weapon_skill_level as i32 / 3 + 1);
            calc_attack_power(dice_result, attacker.attr.str, weapon_skill_level)
        }
    };
    let attack_params = AttackParams {
        attacker: Some(attacker),
        kind: DamageKind::MeleeAttack,
        element: Element::Physical,
        attack_power,
    };
    // Damage target
    let _damage = attack_target(game, attack_params, target);
    // Exp processing
    {
        let target_level = game.gd.chara.get(target).base_attr.level;
        let attacker = game.gd.chara.get_mut(attacker);
        attacker.add_attack_exp(skill_kind, target_level);
    }
    // Sound effect
    crate::audio::play_sound("punch");
}

/// Shot target by long range weapons.
/// If attacker actually do actions, returns true.
pub fn shot_target(game: &mut Game, attacker: CharaId, target: CharaId) -> bool {
    // Damage calculation
    
    let (attack_params, weapon_kind, attacker_pos, target_pos) = {
        let attacker_pos = game.gd.get_current_map().chara_pos(attacker).unwrap();
        let target_pos = game.gd.get_current_map().chara_pos(target).unwrap();
        let cattacker = game.gd.chara.get(attacker);
        let weapon = if let Some(weapon) = cattacker.equip.item(EquipSlotKind::RangedWeapon, 0) {
            weapon
        } else { // If this chara doesn't equip long range weapon
            game_log_i!("no-ranged-weapon-equipped");
            return false;
        };
        let weapon_obj = gobj::get_obj(weapon.idx);
        let weapon_kind = get_weapon_kind(weapon_obj);
        let dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);
        
        let weapon_skill_level = cattacker.skills.get(SkillKind::Weapon(weapon_kind));
        let attack_power = calc_attack_power(dice_result, cattacker.attr.dex, weapon_skill_level);

        let attack_params = AttackParams {
            attacker: Some(attacker),
            kind: DamageKind::RangedAttack,
            element: Element::Physical,
            attack_power,
        };

        (attack_params, weapon_kind, attacker_pos, target_pos)
    };
    // Logging
    {
        let attacker = game.gd.chara.get(attacker);
        let target = game.gd.chara.get(target);
        game_log!("shot-target"; attacker=attacker, target=target);
    }
    // Damage target
    let _damage = attack_target(game, attack_params, target);
    // Exp processing
    {
        let target_level = game.gd.chara.get(target).base_attr.level;
        let attacker = game.gd.chara.get_mut(attacker);
        attacker.add_attack_exp(SkillKind::Weapon(weapon_kind), target_level);
    }
    // Animation pushing
    game.anim_queue.push_shot(attacker_pos, target_pos);
    // Sound effect
    crate::audio::play_sound("arrow");
    true
}

/// Routines for targetted character
fn attack_target(game: &mut Game, attack_params: AttackParams, target: CharaId) -> i32 {
    let equip_def = calc_equip_defence(&game.gd, target);
    let ctarget = game.gd.chara.get(target);
    let idx = ctarget.template;
    let defence_skill_level = ctarget.skills.get(SkillKind::Defence);
    let defence_power = calc_defence_power(
        equip_def[attack_params.element], ctarget.attr.vit, defence_skill_level);
    let damage = (attack_params.attack_power / defence_power).floor() as i32;

    // Dagame log
    game_log!("damaged-chara"; chara=ctarget, damage=damage);
    
    // Give damage
    let hp = super::chara::damage(game, target, damage, attack_params.kind);

    if hp > 0 {
        // Exp for targetted character
        if let Some(attacker) = attack_params.attacker {
            let attacker_level = game.gd.chara.get(attacker).base_attr.level;
            let target = game.gd.chara.get_mut(target);
            target.add_damage_exp(damage, attacker_level);
        }
    } else {
        super::quest::count_slayed_monster(&mut game.gd, idx);
    }

    damage
}

fn get_weapon_kind(item: &ItemObject) -> WeaponKind {
    match item.kind {
        ItemKind::Weapon(kind) => kind,
        _ => unreachable!(),
    }
}

/// Calculate character's defence for each elements
fn calc_equip_defence(gd: &GameData, cid: CharaId) -> ElementArray<u16> {
    let mut def: ElementArray<u16> = ElementArray::default();
    
    for (_, _, item) in gd.get_equip_list(cid).item_iter() {
        let item_obj: &ItemObject = gobj::get_obj(item.idx);
        for e in &ELEMENTS {
            def[*e] = def[*e].saturating_add(item_obj.def[*e]);
        }
    }

    def
}

/// Calculate attack power
fn calc_attack_power(dice: i32, chara_param: u16, skill_level: u32) -> f64 {
    assert!(dice > 0);
    assert!(chara_param > 0);
    let dice = dice as f64;
    let chara_param = chara_param as f64;
    let skill_level = skill_level as f64;
    
    dice * chara_param * chara_param * (skill_level + 8.0).powf(1.5)
}

/// Calculate defence power
fn calc_defence_power(equip_def: u16, chara_param: u16, skill_level: u32) -> f64 {
    assert!(chara_param > 0);
    let equip_def = equip_def as f64;
    let chara_param = chara_param as f64;
    let skill_level = skill_level as f64;

    (equip_def + 16.0) * chara_param * (skill_level + 8.0)
}

/// Calculate accuracy power
fn calc_accuracy_power(weapon: u32, skill_level: u32, chara_param: u16) -> f64 {
    let weapon = weapon as f64;
    let skill_level = skill_level as f64;
    let chara_param = chara_param as f64 * 0.25;

    weapon + skill_level + chara_param * 0.5
}

/// Calculate evasion power
fn calc_evasion_power(equip: u32, skill_level: u32, chara_param: u16) -> f64 {
    let equip = equip as f64;
    let skill_level = skill_level as f64;
    let chara_param = chara_param as f64 * 0.25;

    equip + skill_level + chara_param * 0.5
}

fn hit_judge(gd: &GameData, accuracy_power: f64, target: CharaId, kind: DamageKind) -> bool {
    let evasion_power = {
        let equip = match kind {
            DamageKind::MeleeAttack => 1, // TODO: Use equipment's parameter
            DamageKind::RangedAttack => 1,
            _ => { return true; } // Some kind damage always hits
        };

        let target = gd.chara.get(target);
        calc_evasion_power(
            equip,
            target.skills.get(SkillKind::Evasion),
            target.attr.dex)
    };

    let d = accuracy_power -  evasion_power;
    let p = 1.0 / (1.0 + (-d * 0.125).exp());
    let is_hit = rng::get_rng().gen_bool(p);

    if !is_hit {
        game_log!("attack-evade"; chara=gd.chara.get(target));
    }

    is_hit
}


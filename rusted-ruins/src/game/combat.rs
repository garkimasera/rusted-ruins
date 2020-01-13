use super::chara::CharaEx;
use super::{Game, InfoGetter};
use crate::rng;
use common::gamedata::*;
use common::gobj;
use rng::Rng;
// use rules::RULES;

pub enum DamageKind {
    MeleeAttack,
    RangedAttack,
    Poison,
    Starve,
}

pub struct AttackParams {
    pub attacker_id: Option<CharaId>,
    pub kind: DamageKind,
    pub element: Element,
    pub attack_power: f64,
}

/// Attack neighbor enemy by short range weapon or martial arts
pub fn attack_neighbor(game: &mut Game, attacker_id: CharaId, target_id: CharaId) {
    // Logging
    {
        let attacker = game.gd.chara.get(attacker_id);
        let target = game.gd.chara.get(target_id);
        game_log!("attack"; attacker=attacker, target=target);
    }
    // Animation pushing
    game.anim_queue
        .push_attack(game.gd.get_current_map().chara_pos(target_id).unwrap());
    // Judges hit or miss
    {
        let attacker = game.gd.chara.get(attacker_id);
        let attacker_level = attacker.level;
        let accuracy_power = calc_accuracy_power(
            1, // TODO: Use appropriate parameters
            1,
            attacker.attr.dex,
        );
        // When miss
        if !hit_judge(&game.gd, accuracy_power, target_id, DamageKind::MeleeAttack) {
            // Exp to target chara
            game.gd
                .chara
                .get_mut(target_id)
                .add_evasion_exp(attacker_level);
            crate::audio::play_sound("attack-miss");
            return;
        }
    }

    let skill_kind;

    // Damage calculation
    let attack_power = {
        let attacker = game.gd.chara.get(attacker_id);

        if let Some(weapon) = attacker.equip.item(EquipSlotKind::MeleeWeapon, 0) {
            let weapon_obj = gobj::get_obj(weapon.idx);
            let weapon_kind = get_weapon_kind(weapon_obj);
            skill_kind = SkillKind::Weapon(weapon_kind);

            let dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);
            let weapon_skill_level = attacker.skills.get(skill_kind);
            calc_attack_power(dice_result, attacker.attr.str, weapon_skill_level)
        } else {
            // Attack by martial arts
            skill_kind = SkillKind::MartialArts;
            let weapon_skill_level = attacker.skills.get(skill_kind);
            let dice_result = rng::dice(1, weapon_skill_level as i32 / 3 + 1);
            calc_attack_power(dice_result, attacker.attr.str, weapon_skill_level)
        }
    };
    let attack_params = AttackParams {
        attacker_id: Some(attacker_id),
        kind: DamageKind::MeleeAttack,
        element: Element::Physical,
        attack_power,
    };
    // Damage target
    let _damage = attack_target(game, attack_params, target_id);
    // Exp processing
    {
        let target_level = game.gd.chara.get(target_id).level;
        let attacker = game.gd.chara.get_mut(attacker_id);
        attacker.add_attack_exp(skill_kind, target_level);
    }
    // Sound effect
    crate::audio::play_sound("punch");
}

/// Shot target by long range weapons.
/// If attacker actually do actions, returns true.
pub fn shot_target(game: &mut Game, attacker_id: CharaId, target_id: CharaId) -> bool {
    let attacker = game.gd.chara.get(attacker_id);
    let weapon = if let Some(weapon) = attacker.equip.item(EquipSlotKind::RangedWeapon, 0) {
        weapon
    } else {
        // If this chara doesn't equip long range weapon
        game_log_i!("no-ranged-weapon-equipped");
        return false;
    };
    let attacker_pos = game.gd.get_current_map().chara_pos(attacker_id).unwrap();
    let target_pos = game.gd.get_current_map().chara_pos(target_id).unwrap();

    // Judges hit or miss
    {
        let attacker = game.gd.chara.get(attacker_id);
        let attacker_level = attacker.level;
        let accuracy_power = calc_accuracy_power(
            1, // TODO: Use appropriate parameters
            1,
            attacker.attr.dex,
        );
        // When miss
        if !hit_judge(&game.gd, accuracy_power, target_id, DamageKind::MeleeAttack) {
            // Exp to target chara
            game.gd
                .chara
                .get_mut(target_id)
                .add_evasion_exp(attacker_level);
            game.anim_queue.push_shot(attacker_pos, target_pos);
            crate::audio::play_sound("attack-miss");
            return true;
        }
    }

    // Animation pushing
    game.anim_queue.push_shot(attacker_pos, target_pos);

    // Damage calculation
    let (attack_params, weapon_kind) = {
        let weapon_obj = gobj::get_obj(weapon.idx);
        let weapon_kind = get_weapon_kind(weapon_obj);
        let dice_result = rng::dice(weapon_obj.dice_n as i32, weapon_obj.dice_x as i32);

        let weapon_skill_level = attacker.skills.get(SkillKind::Weapon(weapon_kind));
        let attack_power = calc_attack_power(dice_result, attacker.attr.dex, weapon_skill_level);

        let attack_params = AttackParams {
            attacker_id: Some(attacker_id),
            kind: DamageKind::RangedAttack,
            element: Element::Physical,
            attack_power,
        };

        (attack_params, weapon_kind)
    };
    // Logging
    {
        let target = game.gd.chara.get(target_id);
        game_log!("shot-target"; attacker=attacker, target=target);
    }
    // Damage target
    let _damage = attack_target(game, attack_params, target_id);
    // Exp processing
    {
        let target_level = game.gd.chara.get(target_id).level;
        let attacker = game.gd.chara.get_mut(attacker_id);
        attacker.add_attack_exp(SkillKind::Weapon(weapon_kind), target_level);
    }
    // Sound effect
    crate::audio::play_sound("arrow");
    true
}

/// Routines for targetted character
pub fn attack_target(game: &mut Game, attack_params: AttackParams, target_id: CharaId) -> i32 {
    let equip_def = calc_equip_defence(&game.gd, target_id);
    let target = game.gd.chara.get_mut(target_id);
    let idx = target.template;
    let defence_skill_level = target.skills.get(SkillKind::Defence);
    let defence_power = calc_defence_power(
        equip_def[attack_params.element],
        target.attr.vit,
        defence_skill_level,
    );
    let damage = (attack_params.attack_power / defence_power).floor() as i32;

    // Dagame log
    game_log!("damaged-chara"; chara=target, damage=damage);

    // Give damage
    let hp = target.damage(damage, attack_params.kind);

    if hp > 0 {
        // Exp for targetted character
        if let Some(attacker_id) = attack_params.attacker_id {
            let attacker_level = game.gd.chara.get(attacker_id).level;
            let target = game.gd.chara.get_mut(target_id);
            target.add_damage_exp(damage, attacker_level);
        }
    } else {
        super::quest::count_slayed_monster(&mut game.gd, idx);
        game.anim_queue
            .push_destroy(game.gd.chara_pos(target_id).unwrap());
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

fn hit_judge(gd: &GameData, accuracy_power: f64, target_id: CharaId, kind: DamageKind) -> bool {
    let evasion_power = {
        let equip = match kind {
            DamageKind::MeleeAttack => 1,
            DamageKind::RangedAttack => 1,
            _ => {
                return true;
            } // Some kind damage always hits
        };

        let target = gd.chara.get(target_id);
        calc_evasion_power(
            equip,
            target.skills.get(SkillKind::Evasion),
            target.attr.dex,
        )
    };

    let d = accuracy_power - evasion_power;
    let p = 1.0 / (1.0 + (-d * 0.125).exp());
    let is_hit = rng::get_rng().gen_bool(p);

    if !is_hit {
        game_log!("attack-evade"; chara=gd.chara.get(target_id));
    }

    is_hit
}

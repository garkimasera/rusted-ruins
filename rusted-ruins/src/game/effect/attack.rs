use crate::config::changeable::game_log_cfg;
use crate::game::damage::*;
use crate::game::extrait::CharaExt;
use crate::game::Game;
use crate::rng;
use common::gamedata::*;
use common::gobj;
use geom::ShapeKind;
use rng::Rng;
// use rules::RULES;

#[derive(Clone, Copy)]
struct AttackParams {
    pub attacker_id: Option<CharaId>,
    pub kind: CharaDamageKind,
    pub element: Element,
    pub attack_power: f32,
    pub hit_power: f32,
    pub always_hit: bool,
}

// Melee attack to a chara.
pub fn melee_attack(
    game: &mut Game,
    cid: CharaId,
    target_id: CharaId,
    attack_power: f32,
    hit_power: f32,
    element: Element,
) {
    let attacker = game.gd.chara.get(cid);
    let target = game.gd.chara.get(target_id);

    if game_log_cfg().combat_log.attack() {
        game_log!("attack"; attacker=attacker, target=target);
    }

    let attack_params = AttackParams {
        attacker_id: Some(cid),
        kind: CharaDamageKind::MeleeAttack,
        element,
        attack_power,
        hit_power,
        always_hit: false,
    };

    attack_target(game, attack_params, target_id);
}

// Ranged attack to a chara.
pub fn ranged_attack(
    game: &mut Game,
    cid: CharaId,
    target_id: CharaId,
    attack_power: f32,
    hit_power: f32,
    element: Element,
) {
    let target = game.gd.chara.get(target_id);

    if game_log_cfg().combat_log.attack() {
        game_log!("arrow-hit"; chara=target);
    }

    let attack_params = AttackParams {
        attacker_id: Some(cid),
        kind: CharaDamageKind::RangedAttack,
        element,
        attack_power,
        hit_power,
        always_hit: false,
    };

    attack_target(game, attack_params, target_id);
}

// Explosion attack to a chara.
pub fn explosion_attack(
    game: &mut Game,
    cid: CharaId,
    target_id: CharaId,
    attack_power: f32,
    hit_power: f32,
    element: Element,
) {
    let attack_params = AttackParams {
        attacker_id: Some(cid),
        kind: CharaDamageKind::Explosion,
        element,
        attack_power,
        hit_power,
        always_hit: false,
    };

    attack_target(game, attack_params, target_id);
}

/// Routines for targetted character
fn attack_target(game: &mut Game, attack_params: AttackParams, target_id: CharaId) -> i32 {
    if !attack_params.always_hit
        && !hit_judge(
            &game.gd,
            attack_params.hit_power,
            target_id,
            attack_params.kind,
        )
    {
        return 0;
    }

    let equip_def = calc_equip_defence(&game.gd, target_id);
    let target = game.gd.chara.get_mut(target_id);
    let idx = target.template;
    let defence_skill_level = target.skill_level(SkillKind::Defence);
    let defence_power = calc_defence_power(
        equip_def[attack_params.element],
        target.attr.vit,
        defence_skill_level,
    );
    let damage = (attack_params.attack_power / defence_power).floor() as i32;

    // Give damage
    let hp = do_damage(game, target_id, damage, attack_params.kind);

    if hp > 0 {
        // Exp for targetted character
        if let Some(attacker_id) = attack_params.attacker_id {
            let attacker_level = game.gd.chara.get(attacker_id).level;
            let target = game.gd.chara.get_mut(target_id);
            target.add_damage_exp(damage, attacker_level);
        }
    } else {
        crate::game::quest::count_slayed_monster(&mut game.gd, idx);
    }

    damage
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

/// Calculate defence power
fn calc_defence_power(equip_def: u16, chara_param: u16, skill_level: u32) -> f32 {
    assert!(chara_param > 0);
    let equip_def = equip_def as f32;
    let chara_param = chara_param as f32;
    let skill_level = skill_level as f32;

    (equip_def + 16.0) * chara_param * (skill_level + 8.0)
}

/// Calculate evasion power
fn calc_evasion_power(equip: u32, skill_level: u32, chara_param: u16) -> f32 {
    let equip = equip as f32;
    let skill_level = skill_level as f32;
    let chara_param = chara_param as f32 * 0.25;

    equip + skill_level + chara_param * 0.5
}

fn hit_judge(
    gd: &GameData,
    accuracy_power: f32,
    target_id: CharaId,
    kind: CharaDamageKind,
) -> bool {
    let evasion_power = {
        let equip = match kind {
            CharaDamageKind::MeleeAttack => 1,
            CharaDamageKind::RangedAttack => 1,
            _ => {
                return true;
            } // Some kind damage always hits
        };

        let target = gd.chara.get(target_id);
        calc_evasion_power(
            equip,
            target.skill_level(SkillKind::Evasion),
            target.attr.dex,
        )
    };

    let d = accuracy_power - evasion_power;
    let p = 1.0 / (1.0 + (-d * 0.125).exp());
    let is_hit = rng::get_rng().gen_bool(p.into());

    if !is_hit && game_log_cfg().combat_log.attack() {
        game_log!("attack-evade"; chara=gd.chara.get(target_id));
    }

    is_hit
}

pub fn weapon_to_effect(item: &Item) -> Effect {
    let item_obj = gobj::get_obj(item.idx);

    let weapon_kind = match item_obj.kind {
        ItemKind::Weapon(weapon_kind) => weapon_kind,
        _ => unreachable!(),
    };

    let effect_kind = vec![if weapon_kind.is_melee() {
        EffectKind::Melee {
            element: Element::Physical,
        }
    } else {
        EffectKind::Ranged {
            element: Element::Physical,
        }
    }];

    let anim_img = if weapon_kind.is_melee() {
        "!damage-blunt".to_owned()
    } else {
        String::new()
    };

    let anim_img_shot = if weapon_kind.is_melee() {
        String::new()
    } else {
        "arrow".to_owned()
    };

    let sound = if weapon_kind.is_melee() {
        "punch".to_owned()
    } else {
        "arrow".to_owned()
    };

    let anim_kind = if weapon_kind.is_melee() {
        EffectAnimKind::Chara
    } else {
        EffectAnimKind::Shot
    };

    Effect {
        kind: effect_kind,
        target_mode: TargetMode::Enemy,
        power_adjust: vec![],
        range: 1,
        shape: ShapeKind::OneTile,
        size: 0,
        anim_kind,
        anim_img,
        anim_img_shot,
        sound,
    }
}

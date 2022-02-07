use crate::config::changeable::game_log_cfg;
use crate::damage_popup::PopupKind;
use crate::game::damage::*;
use crate::game::extrait::*;
use crate::game::power::calc_evasion_power;
use crate::game::power::AttackKind;
use crate::game::Game;
use crate::rng;
use common::gamedata::*;
use common::gobj;
use geom::ShapeKind;
use ordered_float::NotNan;
use rng::Rng;
use rules::RULES;

#[derive(Clone, Copy)]
struct AttackParams {
    pub attacker_id: Option<CharaId>,
    pub kind: AttackKind,
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
        game_log_i!("attack"; attacker=attacker, target=target);
    }

    let attack_params = AttackParams {
        attacker_id: Some(cid),
        kind: AttackKind::Melee,
        element,
        attack_power,
        hit_power,
        always_hit: false,
    };

    attack_target(game, attack_params, cid, target_id);
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
        game_log_i!("arrow-hit"; chara=target);
    }

    let attack_params = AttackParams {
        attacker_id: Some(cid),
        kind: AttackKind::Ranged,
        element,
        attack_power,
        hit_power,
        always_hit: false,
    };

    attack_target(game, attack_params, cid, target_id);
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
        kind: AttackKind::Explosion,
        element,
        attack_power,
        hit_power,
        always_hit: true,
    };

    attack_target(game, attack_params, cid, target_id);
}

/// Routines for targetted character
fn attack_target(
    game: &mut Game,
    attack_params: AttackParams,
    cid: CharaId,
    target_id: CharaId,
) -> i32 {
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
    let defence_skill_level = target.skill_level(SkillKind::Defence);
    let defence_power = calc_defence_power(
        equip_def[attack_params.element],
        target.attr.vit,
        defence_skill_level,
    );
    let damage = (attack_params.attack_power / defence_power).floor() as i32;

    // Give damage
    let hp = do_damage(
        game,
        target_id,
        damage,
        attack_params.kind.into(),
        Some(cid),
    );

    if hp > 0 {
        // Exp for targetted character
        if let Some(attacker_id) = attack_params.attacker_id {
            let attacker_level = game.gd.chara.get(attacker_id).lv;
            let target = game.gd.chara.get_mut(target_id);
            target.add_damage_exp(damage, attacker_level);
        }
    }

    damage
}

/// Calculate character's defence for each elements
fn calc_equip_defence(gd: &GameData, cid: CharaId) -> ElementArray<u32> {
    let mut def: ElementArray<u32> = ElementArray::default();

    for (_, _, item) in gd.get_equip_list(cid).item_iter() {
        for e in &ELEMENTS {
            def[*e] = def[*e].saturating_add(item.defence(*e));
        }
    }

    def
}

/// Calculate defence power
fn calc_defence_power(equip_def: u32, chara_param: u16, skill_level: u32) -> f32 {
    assert!(chara_param > 0);
    let equip_def = equip_def as f32;
    let chara_param = chara_param as f32;
    let skill_level = skill_level as f32;

    (equip_def + 16.0) * chara_param * (skill_level + 8.0)
}

fn hit_judge(gd: &GameData, hit_power: f32, target_id: CharaId, kind: AttackKind) -> bool {
    let evasion_power = calc_evasion_power(gd, target_id, kind);

    let d = hit_power - evasion_power + RULES.power.hit_calc_factor0;
    let factor = if d < 0.0 {
        RULES.power.hit_calc_factor1
    } else {
        RULES.power.hit_calc_factor2
    };
    let d = factor * d;
    let p = (0.5 * d) / (1.0 + d.abs()) + 0.5;
    trace!(
        "hit_power = {}, evasion_power = {}, p = {}",
        hit_power,
        evasion_power,
        p
    );
    let is_hit = rng::get_rng().gen_bool(p.into());

    if !is_hit {
        if let Some(pos) = gd.chara_pos(target_id) {
            crate::damage_popup::push(target_id, pos, PopupKind::Miss);
        }

        if game_log_cfg().combat_log.attack() {
            game_log_i!("attack-evade"; chara=gd.chara.get(target_id));
        }
    }

    is_hit
}

pub fn weapon_to_effect(item: &Item) -> Effect {
    let item_obj = gobj::get_obj(item.idx);

    let weapon_kind = match item_obj.kind {
        ItemKind::Weapon(weapon_kind) => weapon_kind,
        _ => unreachable!(),
    };

    let (base_power, hit) = if let Some(ItemObjAttr::Weapon { base_power, hit }) =
        find_attr!(item.obj(), ItemObjAttr::Weapon)
    {
        (*base_power, *hit)
    } else {
        (BasePower::default(), NotNan::default())
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
    } else if let Some(anim_img_shot) =
        find_attr!(item_obj, ItemObjAttr::AnimImgShot(anim_img_shot))
    {
        anim_img_shot.clone()
    } else {
        "arrow".to_owned()
    };

    let sound = if let Some(sound) = find_attr!(item_obj, ItemObjAttr::Sound(sound)) {
        sound.clone()
    } else if weapon_kind.is_melee() {
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
        base_power,
        hit,
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

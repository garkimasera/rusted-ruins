use crate::game::extrait::*;
use common::gamedata::*;
use rules::RULES;

use super::damage::CharaDamageKind;

pub fn calc_power(gd: &GameData, cid: CharaId, method: &PowerCalcMethod) -> f32 {
    let skill_base = RULES.power.skill_base;

    match *method {
        PowerCalcMethod::Fixed => 1.0,
        PowerCalcMethod::BareHands => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::BareHands) as f32;
            let str = chara.attr.str as f32;
            str * str * (skill_level + skill_base).powf(1.5)
        }
        PowerCalcMethod::Melee(weapon_kind) => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(weapon_kind.into()) as f32;
            let str = chara.attr.str as f32;
            str * str * (skill_level + skill_base).powf(1.5)
        }
        PowerCalcMethod::Ranged(weapon_kind) => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(weapon_kind.into()) as f32;
            let dex = chara.attr.dex as f32;
            dex * dex * (skill_level + skill_base).powf(1.5)
        }
        PowerCalcMethod::Release => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::MagicDevice) as f32;
            let int = chara.attr.int as f32;
            int * int * (skill_level + skill_base).powf(1.4)
        }
        PowerCalcMethod::Magic(magic_kind) => {
            let chara = gd.chara.get(cid);
            let skill_level = (chara.skill_level(magic_kind.into()) as f32
                + chara.skill_level(SkillKind::Magic) as f32)
                / 2.0;
            let wil = chara.attr.wil as f32;
            let int = chara.attr.int as f32;
            wil * int * (skill_level + skill_base).powf(1.5)
        }
        PowerCalcMethod::Throw(weight) => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::Throwing) as f32;
            let str = chara.attr.str as f32;
            str * str
                * (skill_level + skill_base).powf(1.5)
                * (weight as f32 * RULES.power.throw_weight_factor / 1000.0)
        }
        PowerCalcMethod::Medical => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::Pharmacy) as f32;
            let int = chara.attr.int as f32;
            int * (skill_level + skill_base).powf(0.5) * RULES.power.medical_power_base
        }
        PowerCalcMethod::Custom(..) => todo!(),
    }
}

pub fn calc_hit(gd: &GameData, cid: CharaId, method: &PowerCalcMethod) -> f32 {
    match *method {
        PowerCalcMethod::Fixed => 1.0,
        PowerCalcMethod::BareHands => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::BareHands) as f32;
            let dex = chara.attr.dex as f32;
            dex + skill_level
        }
        PowerCalcMethod::Melee(weapon_kind) => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(weapon_kind.into()) as f32;
            let dex = chara.attr.dex as f32;
            dex + skill_level
        }
        PowerCalcMethod::Ranged(weapon_kind) => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(weapon_kind.into()) as f32;
            let dex = chara.attr.dex as f32;
            dex + skill_level
        }
        PowerCalcMethod::Release => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::MagicDevice) as f32;
            let dex = chara.attr.dex as f32;
            dex + skill_level
        }
        PowerCalcMethod::Magic(magic_kind) => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(magic_kind.into()) as f32;
            let wil = chara.attr.wil as f32;
            wil + skill_level
        }
        PowerCalcMethod::Throw(weight) => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::Throwing) as f32;
            let str = chara.attr.str as f32;
            let dex = chara.attr.dex as f32;

            let weight_correction = (weight / 1000) as f32 - str * RULES.power.throw_hit_str_factor;
            let weight_correction = if weight_correction > 0.0 {
                weight_correction
            } else {
                0.0
            };

            dex + skill_level - weight_correction
        }
        PowerCalcMethod::Medical => {
            let chara = gd.chara.get(cid);
            let skill_level = chara.skill_level(SkillKind::Pharmacy) as f32;
            let dex = chara.attr.dex as f32;
            dex + skill_level
        }
        PowerCalcMethod::Custom(..) => todo!(),
    }
}

impl From<AttackKind> for CharaDamageKind {
    fn from(kind: AttackKind) -> CharaDamageKind {
        match kind {
            AttackKind::Melee => CharaDamageKind::MeleeAttack,
            AttackKind::Ranged => CharaDamageKind::RangedAttack,
            AttackKind::Explosion => CharaDamageKind::Explosion,
            AttackKind::Direct => CharaDamageKind::Direct,
        }
    }
}

/// Calculate evasion power
pub fn calc_evasion_power(gd: &GameData, cid: CharaId, kind: AttackKind) -> f32 {
    let chara = gd.chara.get(cid);
    let skill_level = chara.skill_level(SkillKind::Evasion) as f32;
    let chara_param = chara.attr.dex as f32;

    let correction = match kind {
        AttackKind::Melee => 1.0,
        AttackKind::Ranged => 1.0,
        _ => 0.0,
    };

    skill_level + chara_param + correction + RULES.power.base_evasion_power
}

pub fn calc_defence(gd: &GameData, cid: CharaId, element: Element, _kind: AttackKind) -> f32 {
    let equip_def = calc_equip_defence(gd, cid, element);
    let chara = gd.chara.get(cid);

    let defence_skill_level = chara.skill_level(SkillKind::Defence);
    let (attr, skill_level) = match element {
        Element::None => {
            return 1.0;
        }
        Element::Physical => (chara.attr.dex, defence_skill_level),
        Element::Heat => (
            chara.attr.dex,
            defence_skill_level / 2 + chara.skill_level(SkillKind::Heat),
        ),
        Element::Cold => (
            chara.attr.dex,
            defence_skill_level / 2 + chara.skill_level(SkillKind::Cold),
        ),
        Element::Shock => (
            chara.attr.dex,
            defence_skill_level / 2 + chara.skill_level(SkillKind::Shock),
        ),
        Element::Poison => (
            chara.attr.dex,
            defence_skill_level / 2 + chara.skill_level(SkillKind::Pharmacy),
        ),
        Element::Spirit => (
            chara.attr.wil,
            defence_skill_level / 2 + chara.skill_level(SkillKind::Spirit),
        ),
    };

    (equip_def as f32 + chara.tm.defence[element] + RULES.power.base_defence)
        * attr as f32
        * (RULES.power.skill_base + skill_level as f32)
}

/// Calculate character's defence for each elements
fn calc_equip_defence(gd: &GameData, cid: CharaId, element: Element) -> u32 {
    let mut def = 0u32;

    for (_, _, item) in gd.get_equip_list(cid).item_iter() {
        def = def.saturating_add(item.defence(element));
    }

    def
}

use crate::game::chara::CharaExt;
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

/// Calculate evasion power
pub fn calc_evasion_power(gd: &GameData, cid: CharaId, kind: CharaDamageKind) -> f32 {
    let chara = gd.chara.get(cid);
    let skill_level = chara.skill_level(SkillKind::Evasion) as f32;
    let chara_param = chara.attr.dex as f32;

    let correction = match kind {
        CharaDamageKind::MeleeAttack => 1.0,
        CharaDamageKind::RangedAttack => 1.0,
        _ => 0.0,
    };

    skill_level + chara_param + correction + RULES.power.base_evasion_power
}

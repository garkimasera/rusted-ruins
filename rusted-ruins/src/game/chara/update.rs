use super::total_effect;
use crate::game::extrait::*;
use common::gamedata::*;
use common::gobj;
use common::obj::CharaTemplateObject;
use rules::RULES;

/// Update character attributes by its status
pub fn update_attributes(chara: &mut Chara) {
    let ct = gobj::get_obj(chara.idx);

    // Update total effect
    total_effect::add_class(&mut chara.te, RULES.classes.get(chara.class));

    chara.attr.max_hp = calc_max_hp(chara, ct);

    for status in &chara.status {
        total_effect::add_status(&mut chara.te, status);
    }

    for (_, chara_trait) in &chara.traits {
        total_effect::add_chara_trait(&mut chara.te, chara_trait);
    }
    // chara.attr.spd = std::cmp::max((base_attr.spd as f32 * factor) as u16, RULES.chara.min_spd);

    // Update attributes
    chara.attr.max_hp = calc_max_hp(chara, ct);
    chara.attr.str = (ct.base_attr.str + chara.te.str).max(1) as u16;
    chara.attr.vit = (ct.base_attr.vit + chara.te.vit).max(1) as u16;
    chara.attr.dex = (ct.base_attr.dex + chara.te.dex).max(1) as u16;
    chara.attr.int = (ct.base_attr.int + chara.te.int).max(1) as u16;
    chara.attr.wil = (ct.base_attr.wil + chara.te.wil).max(1) as u16;
    chara.attr.cha = (ct.base_attr.cha + chara.te.cha).max(1) as u16;
    chara.attr.spd = (ct.base_attr.spd as f32 * chara.te.spd_factor).max(1.0) as u16;

    // View range
    chara.attr.view_range = RULES.chara.default_view_range;
}

fn calc_max_hp(chara: &mut Chara, ct: &CharaTemplateObject) -> i32 {
    let base_hp = (ct.base_attr.base_hp + chara.te.base_hp).max(1);
    let factor = chara.skill_level(SkillKind::Endurance) as i32 + 8;
    ((factor * base_hp / 8) + chara.te.max_hp).max(1)
}

pub fn update_encumbrance_status(chara: &mut Chara) {
    let cap = calc_carrying_capacity(chara);
    let total_weight = chara.item_list.sum_weight() as f32;
    let ratio = total_weight / cap;

    if ratio > RULES.chara.carrying_capacity_threshold_overloaded {
        chara.add_status(CharaStatus::Overloaded);
        return;
    } else if ratio > RULES.chara.carrying_capacity_threshold_strained {
        chara.add_status(CharaStatus::Strained);
        return;
    } else if ratio > RULES.chara.carrying_capacity_threshold_stressed {
        chara.add_status(CharaStatus::Stressed);
        return;
    } else if ratio > RULES.chara.carrying_capacity_threshold_burdened {
        chara.add_status(CharaStatus::Burdened);
        return;
    }
    chara.remove_encumbrance_status();
}

pub fn calc_carrying_capacity(chara: &Chara) -> f32 {
    let skill_level = chara.skill_level(SkillKind::Carrying) as f32;

    (chara.attr.str as f32 / 2.0 + chara.attr.vit as f32)
        * (skill_level + 10.0)
        * RULES.chara.carrying_capacity_factor
}

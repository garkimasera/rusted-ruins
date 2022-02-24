use crate::game::extrait::*;
use common::gamedata::*;
use common::gobj;
use common::obj::CharaTemplateObject;
use rules::RULES;

/// Update character attributes by its status
pub fn update_attributes(chara: &mut Chara) {
    let ct = gobj::get_obj(chara.idx);

    super::total_modifier::update(chara);

    // Update attributes
    chara.attr.max_hp = calc_max_hp(chara, ct);
    chara.attr.max_mp = calc_max_mp(chara, ct);
    chara.attr.str = (ct.base_attr.str + chara.tm.str).max(1) as u16;
    chara.attr.vit = (ct.base_attr.vit + chara.tm.vit).max(1) as u16;
    chara.attr.dex = (ct.base_attr.dex + chara.tm.dex).max(1) as u16;
    chara.attr.int = (ct.base_attr.int + chara.tm.int).max(1) as u16;
    chara.attr.wil = (ct.base_attr.wil + chara.tm.wil).max(1) as u16;
    chara.attr.cha = (ct.base_attr.cha + chara.tm.cha).max(1) as u16;

    // View range
    chara.attr.view_range = RULES.chara.default_view_range;
}

fn calc_max_hp(chara: &mut Chara, ct: &CharaTemplateObject) -> i32 {
    let base_hp = (ct.base_attr.base_hp + chara.tm.base_hp).max(1);
    let factor = chara.skill_level(SkillKind::Endurance) as i32 + RULES.chara.max_hp_skill_factor;
    ((factor * base_hp / RULES.chara.max_hp_skill_factor) + chara.tm.max_hp).max(1)
}

fn calc_max_mp(chara: &mut Chara, ct: &CharaTemplateObject) -> i32 {
    let base_mp = (ct.base_attr.base_mp + chara.tm.base_mp).max(0);
    let factor = chara.skill_level(SkillKind::Magic) as i32 + RULES.chara.max_mp_skill_factor;
    ((factor * base_mp / RULES.chara.max_mp_skill_factor) + chara.tm.max_mp).max(0)
}

pub fn update_encumbrance_status(chara: &mut Chara) {
    let cap = calc_carrying_capacity(chara);
    let total_weight = chara.item_list.sum_weight() as f32;
    let ratio = total_weight / cap;

    let spd_factor = if ratio > RULES.chara.carrying_capacity_threshold_overloaded {
        chara.add_status(CharaStatus::Overloaded);
        RULES.chara.speed_coeff_overloaded
    } else if ratio > RULES.chara.carrying_capacity_threshold_strained {
        chara.add_status(CharaStatus::Strained);
        RULES.chara.speed_coeff_strained
    } else if ratio > RULES.chara.carrying_capacity_threshold_stressed {
        chara.add_status(CharaStatus::Stressed);
        RULES.chara.speed_coeff_stressed
    } else if ratio > RULES.chara.carrying_capacity_threshold_burdened {
        chara.add_status(CharaStatus::Burdened);
        RULES.chara.speed_coeff_burdened
    } else {
        chara.remove_encumbrance_status();
        1.0
    };
    chara.tm.spd_factor *= spd_factor;
    chara.attr.spd = ((chara.obj().base_attr.spd as f32 + chara.tm.spd as f32)
        * chara.tm.spd_factor)
        .max(1.0) as u16;
}

pub fn calc_carrying_capacity(chara: &Chara) -> f32 {
    let skill_level = chara.skill_level(SkillKind::Carrying) as f32;

    (chara.attr.str as f32 / 2.0 + chara.attr.vit as f32)
        * (skill_level + 10.0)
        * RULES.chara.carrying_capacity_factor
}

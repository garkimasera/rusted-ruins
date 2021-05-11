use crate::game::extrait::*;
use common::gamedata::*;
use common::gobj;
use common::obj::CharaTemplateObject;
use rules::RULES;

/// Update character attributes by its status
pub fn update_attributes(chara: &mut Chara) {
    let ct = gobj::get_obj(chara.template);

    let base_attr = ct.base_attr.revise(RULES.class.get(chara.class).revision);

    chara.attr.max_hp = calc_max_hp(chara, ct);
    chara.attr.str = base_attr.str as u16;
    chara.attr.vit = base_attr.vit as u16;
    chara.attr.dex = base_attr.dex as u16;
    chara.attr.int = base_attr.int as u16;
    chara.attr.wil = base_attr.wil as u16;
    chara.attr.cha = base_attr.cha as u16;

    // Speed
    let mut factor = 1.0;
    for status in &chara.status {
        match status {
            CharaStatus::Burdened => {
                factor *= RULES.chara.carrying_capacity_threshold_burdened;
            }
            CharaStatus::Strained => {
                factor *= RULES.chara.carrying_capacity_threshold_strained;
            }
            CharaStatus::Stressed => {
                factor *= RULES.chara.carrying_capacity_threshold_stressed;
            }
            CharaStatus::Overloaded => {
                factor *= RULES.chara.carrying_capacity_threshold_overloaded;
            }
            _ => (),
        }
    }
    chara.attr.spd = std::cmp::max((base_attr.spd as f32 * factor) as u16, RULES.chara.min_spd);

    // View range
    chara.attr.view_range = RULES.chara.default_view_range;
}

fn calc_max_hp(chara: &mut Chara, ct: &CharaTemplateObject) -> i32 {
    (chara.skill_level(SkillKind::Endurance) as i32 + 8) * ct.base_attr.base_hp / 8
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

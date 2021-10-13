use common::basic::BonusLevel;
use common::gamedata::{
    CharaAttrDiff, CharaStatus, CharaTotalEffect, CharaTrait, Property, SkillKind,
};
use rules::class::Class;
use rules::RULES;
use std::collections::HashMap;

pub fn add_chara_trait(te: &mut CharaTotalEffect, t: &CharaTrait) {
    if let CharaTrait::Id(id) = t {
        let t = RULES.chara_traits.get(id);

        for property in &t.properties {
            add_property(te, property);
        }
    }
}

pub fn add_property(te: &mut CharaTotalEffect, p: &Property) {
    match p {
        Property::CharaStr(value) => te.str += value,
        Property::CharaVit(value) => te.vit += value,
        Property::CharaDex(value) => te.dex += value,
        Property::CharaInt(value) => te.int += value,
        Property::CharaWil(value) => te.wil += value,
        Property::CharaCha(value) => te.cha += value,
        Property::CharaSpd(value) => te.spd += value,
    }
}

pub fn add_status(_te: &mut CharaTotalEffect, _status: &CharaStatus) {
    // match status {
    //     _ => (),
    // }
}

pub fn add_class(te: &mut CharaTotalEffect, class: &Class) {
    add_attr_diff(te, &class.attr);
    add_skill_bonus_map(te, &class.skill_bonus)
}

fn add_attr_diff(te: &mut CharaTotalEffect, d: &CharaAttrDiff) {
    te.base_hp += d.base_hp;
    te.str += d.str;
    te.vit += d.vit;
    te.dex += d.dex;
    te.int += d.int;
    te.wil += d.wil;
    te.cha += d.cha;
    te.spd += d.spd;
}

fn add_skill_bonus_map(te: &mut CharaTotalEffect, skill_bonus: &HashMap<SkillKind, BonusLevel>) {
    for (skill_kind, bonus_level) in skill_bonus {
        let bonus = RULES.params.skill_bonus[bonus_level];

        te.skill_level
            .entry(*skill_kind)
            .and_modify(|(factor, lv)| {
                *factor += bonus.0;
                *lv += bonus.1;
            })
            .or_insert((bonus.0, bonus.1));
    }
}

use common::basic::BonusLevel;
use common::gamedata::{
    CharaAttrDiff, CharaModifier, CharaStatus, CharaTotalModifier, CharaTrait, SkillKind,
};
use rules::class::Class;
use rules::RULES;
use std::collections::HashMap;

pub fn add_chara_trait(tm: &mut CharaTotalModifier, t: &CharaTrait) {
    if let CharaTrait::Id(id) = t {
        let t = RULES.chara_traits.get(id);

        for modifier in &t.modifiers {
            add_modifier(tm, modifier);
        }
    }
}

pub fn add_modifier(tm: &mut CharaTotalModifier, p: &CharaModifier) {
    match p {
        CharaModifier::Str(value) => tm.str += value,
        CharaModifier::Vit(value) => tm.vit += value,
        CharaModifier::Dex(value) => tm.dex += value,
        CharaModifier::Int(value) => tm.int += value,
        CharaModifier::Wil(value) => tm.wil += value,
        CharaModifier::Cha(value) => tm.cha += value,
        CharaModifier::Spd(value) => tm.spd += value,
    }
}

pub fn add_status(_tm: &mut CharaTotalModifier, _status: &CharaStatus) {
    // match status {
    //     _ => (),
    // }
}

pub fn add_class(tm: &mut CharaTotalModifier, class: &Class) {
    add_attr_diff(tm, &class.attr);
    add_skill_bonus_map(tm, &class.skill_bonus)
}

fn add_attr_diff(tm: &mut CharaTotalModifier, d: &CharaAttrDiff) {
    tm.base_hp += d.base_hp;
    tm.str += d.str;
    tm.vit += d.vit;
    tm.dex += d.dex;
    tm.int += d.int;
    tm.wil += d.wil;
    tm.cha += d.cha;
    tm.spd += d.spd;
}

fn add_skill_bonus_map(te: &mut CharaTotalModifier, skill_bonus: &HashMap<SkillKind, BonusLevel>) {
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

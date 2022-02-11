use common::basic::BonusLevel;
use common::gamedata::*;
use common::gamedata::{
    CharaAttrDiff, CharaModifier, CharaStatus, CharaTotalModifier, CharaTrait, SkillKind,
};
use common::gobj;
use rules::class::Class;
use rules::RULES;
use std::collections::HashMap;

pub fn update(chara: &mut Chara) {
    chara.tm = Box::default();
    add_class(&mut chara.tm, RULES.classes.get(chara.class));

    for status in &chara.status {
        add_status(&mut chara.tm, status);
    }

    for (_, chara_trait) in &chara.traits {
        add_chara_trait(&mut chara.tm, chara_trait);
    }

    add_equipments(&mut chara.tm, &chara.equip);
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
        &CharaModifier::Defence { element, value } => {
            tm.defence[element].0 += f32::from(value);
        }
        &CharaModifier::DefenceMultiplier { element, value } => {
            tm.defence[element].1 += f32::from(value);
        }
    }
}

pub fn add_chara_trait(tm: &mut CharaTotalModifier, t: &CharaTrait) {
    if let CharaTrait::Id(id) = t {
        let t = RULES.chara_traits.get(id);

        for modifier in &t.modifiers {
            add_modifier(tm, modifier);
        }
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

fn add_equipments(tm: &mut CharaTotalModifier, equip: &EquipItemList) {
    for (_, _, item) in equip.slot_iter() {
        let item = if let Some(item) = item {
            item
        } else {
            continue;
        };
        let item_obj = gobj::get_obj(item.idx);

        for item_obj_attr in &item_obj.attrs {
            if let ItemObjAttr::CharaModifier(m) = item_obj_attr {
                add_modifier(tm, m);
            }
        }
    }
}

use common::gamedata::{CharaAttrDiff, CharaStatus, CharaTotalEffect, CharaTrait, Property};
use rules::class::Class;
use rules::RULES;

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

pub fn add_status(te: &mut CharaTotalEffect, status: &CharaStatus) {
    match status {
        CharaStatus::Burdened => {
            te.spd_factor *= RULES.chara.speed_coeff_burdened;
        }
        CharaStatus::Strained => {
            te.spd_factor *= RULES.chara.speed_coeff_strained;
        }
        CharaStatus::Stressed => {
            te.spd_factor *= RULES.chara.speed_coeff_stressed;
        }
        CharaStatus::Overloaded => {
            te.spd_factor *= RULES.chara.speed_coeff_overloaded;
        }
        _ => (),
    }
}

pub fn add_class(te: &mut CharaTotalEffect, class: &Class) {
    add_attr_diff(te, &class.attr);
}

pub fn add_attr_diff(te: &mut CharaTotalEffect, d: &CharaAttrDiff) {
    te.base_hp += d.base_hp;
    te.str += d.str;
    te.vit += d.vit;
    te.dex += d.dex;
    te.int += d.int;
    te.wil += d.wil;
    te.cha += d.cha;
    te.spd += d.spd;
}

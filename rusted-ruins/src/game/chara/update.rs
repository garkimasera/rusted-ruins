use common::gamedata::*;
use common::gobj;
use common::obj::CharaTemplateObject;
use rules::RULES;

/// Update character attributes by its status
pub fn update_attributes(chara: &mut Chara) {
    let ct = gobj::get_obj(chara.template);

    let base_attr = if let Some(&r) = RULES.chara.class_revision.get(&chara.class) {
        ct.base_attr.revise(r)
    } else {
        ct.base_attr
    };

    chara.attr.max_hp = calc_max_hp(chara, ct);
    chara.attr.str = base_attr.str as u16;
    chara.attr.vit = base_attr.vit as u16;
    chara.attr.dex = base_attr.dex as u16;
    chara.attr.int = base_attr.int as u16;
    chara.attr.wil = base_attr.wil as u16;
    chara.attr.cha = base_attr.cha as u16;
    chara.attr.spd = base_attr.spd as u16;
    chara.attr.view_range = RULES.chara.default_view_range;
}

fn calc_max_hp(chara: &mut Chara, ct: &CharaTemplateObject) -> i32 {
    (chara.skills.get(SkillKind::Endurance) as i32 + 8) * ct.base_attr.base_hp / 8
}

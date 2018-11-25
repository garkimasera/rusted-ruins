
use common::gamedata::*;
use common::obj::CharaTemplateObject;
use common::gobj;
use rules::RULES;

/// Update character attributes by its status
pub fn update_attributes(chara: &mut Chara) {
    let ct = gobj::get_obj(chara.template);
    
    chara.attr.max_hp = calc_max_hp(chara, ct);
    
    chara.attr.str = chara.base_attr.str;
    chara.attr.vit = chara.base_attr.vit;
    chara.attr.dex = chara.base_attr.dex;
    chara.attr.int = chara.base_attr.int;
    chara.attr.wil = chara.base_attr.wil;
    chara.attr.cha = chara.base_attr.cha;
    chara.attr.spd = chara.base_attr.spd;
    chara.attr.view_range = RULES.chara.default_view_range;
}

fn calc_max_hp(chara: &mut Chara, ct: &CharaTemplateObject) -> i32 {
    (chara.skills.get(SkillKind::Endurance) as i32 + 8) * ct.base_hp / 8
}


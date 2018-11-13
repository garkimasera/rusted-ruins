
use common::gamedata::*;
use common::obj::CharaTemplateObject;
use common::gobj;
use rules::RULES;

/// Update character parameters by its status
pub fn update_params(chara: &mut Chara) {
    let ct = gobj::get_obj(chara.template);
    
    chara.params.max_hp = calc_max_hp(chara, ct);
    
    chara.params.str = chara.base_params.str;
    chara.params.vit = chara.base_params.vit;
    chara.params.dex = chara.base_params.dex;
    chara.params.int = chara.base_params.int;
    chara.params.wil = chara.base_params.wil;
    chara.params.cha = chara.base_params.cha;
    chara.params.spd = chara.base_params.spd;
    chara.params.view_range = RULES.chara.default_view_range;
}

fn calc_max_hp(chara: &mut Chara, ct: &CharaTemplateObject) -> i32 {
    (chara.skills.get(SkillKind::Endurance) as i32 + 8) * ct.base_hp / 8
}


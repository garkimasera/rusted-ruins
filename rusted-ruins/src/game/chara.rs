
use common::objholder::CharaTemplateIdx;
use common::gamedata::chara::*;
use common::gamedata::item::Inventory;
use common::gobj;
use text;

/// Create character from chara_template
pub fn create_chara(chara_template_idx: CharaTemplateIdx) -> Chara {
    let ct = gobj::get_obj(chara_template_idx);

    let max_hp = ct.max_hp;

    let params = CharaParams {
        level: 1,
        max_hp: max_hp,
        str: ct.str,
        vit: ct.vit,
        dex: ct.dex,
        int: ct.int,
        wil: ct.wil,
        cha: ct.cha,
        spd: ct.spd,
    };

    let chara = Chara {
        name: text::obj_txt(&ct.id).to_owned(),
        params: params,
        template: chara_template_idx,
        inventory: Inventory::for_chara(),
        wait_time: 100.0,
        hp: max_hp,
        rel: Relationship::NEUTRAL,
    };
    chara
}

/// Create npc character from the race
pub fn create_npc_chara(race: Race) -> Chara {
    let ct_iterater = gobj::get_objholder().chara_template.iter().enumerate();
    
    for (idx, _) in ct_iterater.filter(|&(_, ct)| ct.race == race) {
        let mut chara = create_chara(CharaTemplateIdx(idx as u32));
        chara.rel = Relationship::HOSTILE;
        return chara;
    }
    panic!();
}


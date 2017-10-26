
use common::objholder::CharaTemplateIdx;
use common::gamedata::chara::*;
use common::gamedata::item::Inventory;
use common::gobj;
use rand::{Rng, thread_rng};
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
pub fn create_npc_chara(race: Race, floor_level: u32) -> Chara {    
    let mut chara = create_chara(choose_npc_chara_template(race, floor_level));
    chara.rel = Relationship::HOSTILE;
    return chara;
}

/// Choose one chara_template by race, gen_level and gen_weight
fn choose_npc_chara_template(race: Race, floor_level: u32) -> CharaTemplateIdx {
    let v = &gobj::get_objholder().chara_template;
    // Search the first idx that has specified race
    let start_idx = v.iter().enumerate().find(|&(_, ct)| ct.race == race).expect("No character found").0;

    // Search the last idx, and sum up gen_weight * weight_dist
    let weight_dist = CalcLevelWeightDist::new(floor_level);
    let mut sum = 0.0;
    let mut end_idx = v.len() - 1;
    
    for i in start_idx..(v.len()) {
        let ct = &v[i];
        if ct.race != race {
            end_idx = i;
            break;
        }
        sum += weight_dist.calc(ct.gen_level) * ct.gen_weight as f64;
    }

    // Choose one chara
    let r = thread_rng().gen_range(0.0, sum);
    let mut sum = 0.0;
    for i in start_idx..(end_idx + 1) {
        let ct = &v[i];
        sum += weight_dist.calc(ct.gen_level) * ct.gen_weight as f64;
        if r < sum {
            return CharaTemplateIdx(i as u32);
        }
    }

    CharaTemplateIdx(start_idx as u32)
}

struct CalcLevelWeightDist {
    floor_level: f64,
    upper_margin: f64,
}

impl CalcLevelWeightDist {
    fn new(floor_level: u32) -> CalcLevelWeightDist {
        CalcLevelWeightDist {
            floor_level: floor_level as f64,
            upper_margin: 5.0,
        }
    }
    
    fn calc(&self, l: u32) -> f64 {
        let l = l as f64;
        if l <= self.floor_level {
            l / self.floor_level
        } else {
            let a = -(l / self.upper_margin) + 1.0 + (self.floor_level / self.upper_margin);
            if a < 0.0 { 0.0 } else { a }
        }
    }
}


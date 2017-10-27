
use common::objholder::CharaTemplateIdx;
use common::gamedata::chara::*;
use common::gamedata::item::Inventory;
use common::gamedata::site::DungeonKind;
use common::gobj;
use rules::RULES;
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
pub fn create_npc_chara(dungeon: DungeonKind, floor_level: u32) -> Chara {    
    let mut chara = create_chara(choose_npc_chara_template(dungeon, floor_level));
    chara.rel = Relationship::HOSTILE;
    return chara;
}

/// Choose one chara_template by race, gen_level and gen_weight
fn choose_npc_chara_template(dungeon: DungeonKind, floor_level: u32) -> CharaTemplateIdx {
    let dungeon_adjustments = RULES.map_gen.npc_gen.get(&dungeon).expect("No rule for npc generation");
    let chara_templates = &gobj::get_objholder().chara_template;

    // Sum up gen_weight * weight_dist * dungeon_adjustment
    let weight_dist = CalcLevelWeightDist::new(floor_level);
    let mut sum = 0.0;
    let mut first_available_ct_idx = None;
    
    for (i, ct) in chara_templates.iter().enumerate() {
        if let Some(da) = dungeon_adjustments.get(&ct.race) {
            sum += weight_dist.calc(ct.gen_level) * ct.gen_weight as f64 * *da as f64;
            if first_available_ct_idx.is_none() {
                first_available_ct_idx = Some(i);
            }
        }
    }

    // Choose one chara
    let r = thread_rng().gen_range(0.0, sum);
    let mut sum = 0.0;
    for (i, ct) in chara_templates.iter().enumerate() {
        if let Some(da) = dungeon_adjustments.get(&ct.race) {
            sum += weight_dist.calc(ct.gen_level) * ct.gen_weight as f64 * *da as f64;
            if r < sum {
                return CharaTemplateIdx(i as u32);
            }
        }
    }

    CharaTemplateIdx(first_available_ct_idx.unwrap() as u32)
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


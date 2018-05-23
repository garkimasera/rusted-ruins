
use common::basic::WAIT_TIME_START;
use common::objholder::CharaTemplateIdx;
use common::gamedata::*;
use common::gobj;
use rules::RULES;

/// Create character from chara_template
pub fn create_chara(chara_template_idx: CharaTemplateIdx) -> Chara {
    let ct = gobj::get_obj(chara_template_idx);

    let max_hp = ct.max_hp;

    let base_params = CharaBaseParams {
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
    
    let mut chara = Chara {
        name: None,
        params: CharaParams::default(),
        base_params: base_params,
        template: chara_template_idx,
        class: CharaClass::Civilian,
        item_list: ItemList::for_chara(),
        equip: EquipItemList::new(&[]),
        wait_time: WAIT_TIME_START,
        ai: CharaAI::default(),
        hp: max_hp,
        status: Vec::new(),
        nutrition: RULES.params.default_nutrition,
        skills: SkillList::default(),
        rel: Relationship::NEUTRAL,
        trigger: None,
        talk: None,
    };
    super::update_params(&mut chara);
    chara.hp = chara.base_params.max_hp;
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
    let dungeon_gen_params = RULES.dungeon_gen.get(&dungeon).expect("No rule for npc generation");
    let nrp = &dungeon_gen_params.npc_race_probability;
    let chara_templates = &gobj::get_objholder().chara_template;

    // Sum up gen_weight * weight_dist * dungeon_adjustment
    let weight_dist = CalcLevelWeightDist::new(floor_level);
    let mut sum = 0.0;
    let mut first_available_ct_idx = None;
    
    for (i, ct) in chara_templates.iter().enumerate() {
        if let Some(da) = nrp.get(&ct.race) {
            sum += weight_dist.calc(ct.gen_level) * ct.gen_weight as f64 * *da as f64;
            if first_available_ct_idx.is_none() {
                first_available_ct_idx = Some(i);
            }
        }
    }

    // Choose one chara
    if sum == 0.0 {
        return CharaTemplateIdx(first_available_ct_idx
                                .expect("Any appropriate chara_template not found") as u32);
    }
    let r = ::rng::gen_range(0.0, sum);
    let mut sum = 0.0;
    for (i, ct) in chara_templates.iter().enumerate() {
        if let Some(da) = nrp.get(&ct.race) {
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


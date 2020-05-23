use crate::game::extrait::*;
use common::basic::WAIT_TIME_NUMERATOR;
use common::gamedata::*;
use common::gobj;
use common::obj::CharaTemplateObject;
use common::objholder::CharaTemplateIdx;
use rng::gen_range;
use rules::RULES;
use std::collections::HashMap;

/// Create character from chara_template
pub fn create_chara(chara_template_idx: CharaTemplateIdx, lv: u32) -> Chara {
    let ct = gobj::get_obj(chara_template_idx);

    let mut chara = Chara {
        name: None,
        attr: CharaAttributes::default(),
        template: chara_template_idx,
        class: CharaClass::default(),
        level: lv,
        item_list: ItemList::new(),
        equip: EquipItemList::new(&[]),
        wait_time: WAIT_TIME_NUMERATOR,
        ai: create_ai(ct.default_ai_kind),
        hp: 1,
        status: Vec::new(),
        sp: RULES.chara.sp_default,
        skills: gen_skill_list(ct, lv),
        rel: Relationship::NEUTRAL,
        trigger_talk: None,
    };

    chara.update();
    chara.hp = chara.attr.max_hp;
    chara.reset_wait_time();
    chara
}

/// Create npc character from the race
pub fn create_npc_chara(dungeon: DungeonKind, floor_level: u32) -> Chara {
    let idx = choose_npc_chara_template(
        &RULES
            .dungeon_gen
            .get(&dungeon)
            .expect("No rule for npc generation")
            .npc_race_probability,
        floor_level,
    );
    let ct = gobj::get_obj(idx);
    let mut chara = create_chara(idx, ct.gen_level);
    set_skill(&mut chara);
    chara.rel = Relationship::HOSTILE;
    chara
}

/// Choose one chara_template by race, gen_level and gen_weight
pub fn choose_npc_chara_template(nrp: &HashMap<String, f32>, floor_level: u32) -> CharaTemplateIdx {
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
    if !(sum > 0.0) {
        return CharaTemplateIdx::from_usize(
            first_available_ct_idx.expect("Any appropriate chara_template not found"),
        );
    }
    let r = gen_range(0.0, sum);
    let mut sum = 0.0;
    for (i, ct) in chara_templates.iter().enumerate() {
        if let Some(da) = nrp.get(&ct.race) {
            sum += weight_dist.calc(ct.gen_level) * ct.gen_weight as f64 * *da as f64;
            if r < sum {
                return CharaTemplateIdx::from_usize(i);
            }
        }
    }

    CharaTemplateIdx::from_usize(first_available_ct_idx.unwrap())
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
            if a < 0.0 {
                0.0
            } else {
                a
            }
        }
    }
}

/// Create AI parameters
pub fn create_ai(ai_kind: NpcAIKind) -> CharaAI {
    CharaAI { kind: ai_kind }
}

/// Get equip slot list
pub fn equip_slots(race: &str) -> Vec<EquipSlotKind> {
    let mut slots = RULES.chara_gen.equip_slots.clone();
    slots.extend_from_slice(&RULES.race.get(race).equip_slots);
    slots
}

/// Set skills to npc
fn set_skill(chara: &mut Chara) {
    let _ct = gobj::get_obj(chara.template);
}

/// Generate skill list based on floor level and CharaTemplateObject
fn gen_skill_list(_ct: &CharaTemplateObject, lv: u32) -> SkillList {
    let mut skill_list = SkillList::default();
    let common_skills = &RULES.chara_gen.common_skills;

    for skill_kind in common_skills {
        skill_list.set_skill_level(*skill_kind, lv)
    }

    skill_list
}

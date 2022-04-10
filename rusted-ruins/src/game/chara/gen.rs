use crate::game::extrait::*;
use common::basic::{BonusLevel, WAIT_TIME_NUMERATOR};
use common::gamedata::*;
use common::gobj;
use common::obj::CharaTemplateObject;
use common::objholder::CharaTemplateIdx;
use rules::RULES;
use std::collections::HashMap;

/// Create character from chara_template
pub fn create_chara<T: Into<Option<FactionId>>>(
    chara_template_idx: CharaTemplateIdx,
    lv: u32,
    faction: T,
    class: Option<CharaClass>,
) -> Chara {
    let ct = gobj::get_obj(chara_template_idx);
    let class = class.unwrap_or(ct.class);

    let mut chara = Chara {
        name: None,
        attr: CharaAttributes::default(),
        idx: chara_template_idx,
        class,
        faction: faction.into().unwrap_or(ct.faction),
        lv,
        item_list: ItemList::default(),
        equip: EquipItemList::new(&[]),
        wait_time: WAIT_TIME_NUMERATOR,
        ai: CharaAi {
            kind: ct.default_ai_kind,
            ..CharaAi::default()
        },
        hp: 1,
        sp: RULES.chara.sp_default,
        skills: gen_skill_list(ct, lv, class),
        ..Chara::default()
    };

    for race_trait in RULES
        .races
        .iter(&ct.race)
        .flat_map(|race| race.traits.iter())
    {
        chara
            .traits
            .push((CharaTraitOrigin::Race, race_trait.clone()));
    }

    gen_equips(&mut chara, ct);

    chara.update_all();
    chara.hp = chara.attr.max_hp;
    chara.mp = chara.attr.max_mp;
    chara.reset_wait_time();
    chara
}

/// Create npc character from the race
pub fn create_npc_chara(dungeon: DungeonKind, floor_level: u32) -> Option<Chara> {
    let dungeon_gen_rule = &RULES
        .dungeon_gen
        .get(&dungeon)
        .expect("No rule for npc generation");
    let idx = choose_npc_chara_template(&dungeon_gen_rule.npc_race_probability, floor_level)?;
    let ct = gobj::get_obj(idx);
    let mut chara = create_chara(idx, ct.gen_level, ct.faction, None);
    set_skill(&mut chara);
    Some(chara)
}

/// Choose one chara_template by race, gen_level and gen_weight
pub fn choose_npc_chara_template(
    nrp: &HashMap<String, f32>,
    floor_level: u32,
) -> Option<CharaTemplateIdx> {
    let chara_templates = &gobj::get_objholder().chara_template;
    let weight_dist = CalcLevelWeightDist::new(floor_level);

    let calc_weight = |ct: &CharaTemplateObject| {
        RULES
            .races
            .iter_ids(&ct.race)
            .filter_map(|race_id| nrp.get(race_id).copied())
            .reduce(f32::max)
            .map(|p| weight_dist.calc(ct.gen_level) * ct.gen_weight as f32 * p)
            .unwrap_or(0.0)
    };

    rng::choose(chara_templates, calc_weight).map(|(i, _)| CharaTemplateIdx::from_usize(i))
}

struct CalcLevelWeightDist {
    floor_level: f32,
    upper_margin: f32,
}

impl CalcLevelWeightDist {
    fn new(floor_level: u32) -> CalcLevelWeightDist {
        CalcLevelWeightDist {
            floor_level: floor_level as f32,
            upper_margin: 5.0,
        }
    }

    fn calc(&self, l: u32) -> f32 {
        let l = l as f32;
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

/// Get equip slot list
pub fn equip_slots(race: &str) -> Vec<EquipSlotKind> {
    let mut slots = RULES.chara_gen.equip_slots.clone();
    slots.extend_from_slice(&RULES.races.get(race).equip_slots);
    slots
}

/// Set skills to npc
fn set_skill(chara: &mut Chara) {
    let _ct = gobj::get_obj(chara.idx);
}

/// Generate skill list based on floor level and CharaTemplateObject
fn gen_skill_list(_ct: &CharaTemplateObject, lv: u32, class: CharaClass) -> SkillList {
    let mut skill_list = SkillList::default();
    let common_skills = &RULES.chara_gen.common_skills;

    for skill_kind in common_skills {
        skill_list.set_skill_level(*skill_kind, lv)
    }

    for (skill_kind, bonus) in &RULES.classes.get(class).skill_bonus {
        if *bonus > BonusLevel::None {
            skill_list.set_skill_level(*skill_kind, 1);
        }
    }

    skill_list
}

/// Generate equipments from rules.
fn gen_equips(chara: &mut Chara, ct: &CharaTemplateObject) {
    use crate::game::item::gen::*;

    let slots = equip_slots(&ct.race);
    let equip = EquipItemList::new(&slots);
    chara.equip = equip;

    let equips_rule = &RULES.classes.get(chara.class).equips;

    for EquipGen {
        esk,
        item_selector,
        gen_level_bonus,
        quality_bonus,
    } in equips_rule
    {
        let level = chara.lv + *gen_level_bonus;
        let item_selector = item_selector.clone().equip_slot_kind(*esk).level(level);

        let item_idx = if let Some(item_idx) = choose_item_by_item_selector(&item_selector) {
            item_idx
        } else {
            continue;
        };

        let mut item = gen_item_from_idx(item_idx, level);
        item.quality.base += *quality_bonus;
        chara.equip.equip(*esk, 0, item);
    }

    // Overwrite class equipment by chara template equipment settings
    for EquipGen {
        esk,
        item_selector,
        gen_level_bonus,
        quality_bonus,
    } in &ct.equips
    {
        let level = chara.lv + *gen_level_bonus;
        let item_selector = item_selector.clone().equip_slot_kind(*esk).level(level);

        let item_idx = if let Some(item_idx) = choose_item_by_item_selector(&item_selector) {
            item_idx
        } else {
            continue;
        };

        let mut item = gen_item_from_idx(item_idx, level);
        item.quality.base += *quality_bonus;
        chara.equip.equip(*esk, 0, item);
    }
}

use crate::game::extrait::*;
use common::basic::{BonusLevel, WAIT_TIME_NUMERATOR};
use common::gamedata::*;
use common::gobj;
use common::obj::CharaTemplateObject;
use common::objholder::CharaTemplateIdx;
use rules::RULES;
use std::collections::HashMap;

/// Create character from chara_template
pub fn create_chara(
    chara_template_idx: CharaTemplateIdx,
    lv: u32,
    faction: FactionId,
    class: Option<CharaClass>,
) -> Chara {
    let ct = gobj::get_obj(chara_template_idx);
    let class = class.unwrap_or(ct.class);

    let mut chara = Chara {
        name: None,
        attr: CharaAttributes::default(),
        template: chara_template_idx,
        class,
        faction,
        level: lv,
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

    if let Some(race) = RULES.race.get(&ct.race) {
        for race_trait in &race.traits {
            chara
                .traits
                .push((CharaTraitOrigin::Race, race_trait.clone()));
        }
    }

    gen_equips(&mut chara, ct);

    chara.update();
    chara.hp = chara.attr.max_hp;
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
    let faction_id = ct.faction.unwrap_or(dungeon_gen_rule.default_faction_id);
    let mut chara = create_chara(idx, ct.gen_level, faction_id, None);
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
        if let Some(da) = nrp.get(&ct.race) {
            weight_dist.calc(ct.gen_level) * ct.gen_weight as f32 * *da as f32
        } else {
            0.0
        }
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
    slots.extend_from_slice(
        &RULES
            .race
            .get(race)
            .unwrap_or_else(|| {
                error!("unknown race \"{}\"", race);
                panic!()
            })
            .equip_slots,
    );
    slots
}

/// Set skills to npc
fn set_skill(chara: &mut Chara) {
    let _ct = gobj::get_obj(chara.template);
}

/// Generate skill list based on floor level and CharaTemplateObject
fn gen_skill_list(_ct: &CharaTemplateObject, lv: u32, class: CharaClass) -> SkillList {
    let mut skill_list = SkillList::default();
    let common_skills = &RULES.chara_gen.common_skills;

    for skill_kind in common_skills {
        skill_list.set_skill_level(*skill_kind, lv)
    }

    for (skill_kind, bonus) in &RULES.class.get(class).skill_bonus {
        if *bonus > BonusLevel::None {
            skill_list.set_skill_level(*skill_kind, 1);
        }
    }

    skill_list
}

/// Generate equipments from rules.
fn gen_equips(chara: &mut Chara, ct: &CharaTemplateObject) {
    let slots = equip_slots(&ct.race);
    let equip = EquipItemList::new(&slots);
    chara.equip = equip;

    let equips_rule = &RULES.class.get(chara.class).equips;

    for (esk, item_selector, bonus) in equips_rule {
        use crate::game::item::gen::*;

        let level = chara.level + bonus;
        let item_selector = item_selector.clone().equip_slot_kind(*esk).level(level);

        let item_idx = if let Some(item_idx) = choose_item_by_item_selector(&item_selector) {
            item_idx
        } else {
            continue;
        };

        let item = gen_item_from_idx(item_idx, level);

        chara.equip.equip(*esk, 0, item);
    }
}

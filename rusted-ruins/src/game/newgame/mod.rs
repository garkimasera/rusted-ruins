
use common::gamedata;
use common::gamedata::*;
use common::gobj;
use rules::RULES;
use super::skill::SkillListEx;

pub struct NewGameBuilder {
    gd: GameData,
    player_name: Option<String>,
    chara_class: Option<CharaClass>,
}

impl NewGameBuilder {
    pub fn new() -> NewGameBuilder {
        NewGameBuilder {
            gd: GameData::empty(),
            player_name: None,
            chara_class: None,
        }
    }

    pub fn set_player_name(&mut self, name: &str) {
        self.player_name = Some(name.to_owned());
    }

    pub fn set_chara_class(&mut self, chara_class: CharaClass) {
        self.chara_class = Some(chara_class);
    }

    pub fn build(mut self) -> GameData {
        {
            let mut gd = &mut self.gd;

            super::region::add_region(&mut gd, &RULES.newgame.start_region);

            let mid = MapId::RegionMap { rid: RegionId::default() };
            gd.set_current_mapid(mid);
            let start_pos = RULES.newgame.start_pos;

            super::region::gen_dungeon(&mut gd, mid.rid());

            let chara_template_id = &RULES.newgame.chara_template_table[&self.chara_class.unwrap()];
            let mut chara = super::chara::creation::create_chara(gobj::id_to_idx(chara_template_id));
            chara.rel = gamedata::chara::Relationship::ALLY;
            chara.name = Some(self.player_name.unwrap());
            set_initial_skills(&mut chara);
            super::chara::update_params(&mut chara);

            gd.player.set_money(RULES.newgame.start_money as i64);
            
            /* Test code for equipment */
            use common::gamedata::chara::Race;
            let slots = &RULES.chara_gen.default_equip_slots.get(&Race::Human).unwrap();
            let equip = gamedata::item::EquipItemList::new(slots);
            chara.equip = equip;
            let cid = gd.add_chara(chara, CharaKind::Player);
            gd.region.get_map_mut(mid).locate_chara(cid, start_pos);

            // Initial date setting
            gd.time = Time::new(
                RULES.params.initial_date_year,
                RULES.params.initial_date_month,
                RULES.params.initial_date_day,
                RULES.params.initial_date_hour);
        }
        self.gd
    }
}

/// Set initial skills from rule
fn set_initial_skills(chara: &mut Chara) {
    for skill in &RULES.newgame.common_initial_skills {
        chara.skills.learn_new_skill(*skill);
    }
}


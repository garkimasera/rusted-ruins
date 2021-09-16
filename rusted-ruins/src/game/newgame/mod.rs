use super::extrait::*;
use super::skill::SkillListExt;
use common::gamedata::*;
use common::gobj;
use rules::RULES;

pub struct NewGameBuilder {
    gd: GameData,
    player_name: Option<String>,
    chara_class: Option<CharaClass>,
}

impl NewGameBuilder {
    pub fn new() -> NewGameBuilder {
        let mut gd = GameData::empty();
        gd.play_time.start();

        NewGameBuilder {
            gd,
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
        rng::reseed(crate::config::CONFIG.fix_rand);
        {
            let mut gd = &mut self.gd;

            gd.meta.set_save_name(self.player_name.as_ref().unwrap());

            super::region::add_region(&mut gd, &RULES.newgame.start_region);

            let mid = MapId::RegionMap {
                rid: RegionId::default(),
            };
            gd.set_initial_mapid(mid);
            let start_pos = RULES.newgame.start_pos;

            super::region::gen_dungeon(&mut gd, mid.rid());

            let class = self.chara_class.unwrap();
            let chara_template_id = &RULES.newgame.chara_template_table[&class];
            let mut chara = super::chara::gen::create_chara(
                gobj::id_to_idx(chara_template_id),
                1,
                FactionId::player(),
                Some(class),
            );
            chara.name = Some(self.player_name.as_ref().unwrap().clone());
            chara
                .traits
                .push((CharaTraitOrigin::Inherent, CharaTrait::MainCharacter));
            set_initial_skills(&mut chara);
            chara.update();

            gd.player.set_money(RULES.newgame.start_money as i64);

            gd.add_chara(CharaId::Player, chara);
            gd.region
                .get_map_mut(mid)
                .locate_chara(CharaId::Player, start_pos);
            set_initial_items(&mut gd);

            // Initial date setting
            gd.time = GameTime::new(
                RULES.newgame.initial_date_year,
                RULES.newgame.initial_date_month,
                RULES.newgame.initial_date_day,
                RULES.newgame.initial_date_hour,
            );

            // Faction relation setting
            for (faction_id, faction) in &RULES.faction.factions {
                gd.faction.set(*faction_id, faction.default_relation);
            }

            // Creation setting
            crate::game::creation::add_initial_recipes(gd);
        }
        self.gd
    }
}

/// Set initial items from rule
fn set_initial_items(gd: &mut GameData) {
    for &(item_idx, n) in &RULES.newgame.common_initial_items {
        let item = crate::game::item::gen::gen_item_from_idx(item_idx, 1);

        gd.append_item_to(ItemListLocation::PLAYER, item, n);
    }
}

/// Set initial skills from rule
fn set_initial_skills(chara: &mut Chara) {
    for skill in &RULES.newgame.common_initial_skills {
        chara.skills.learn_new_skill(*skill);
    }

    for active_skill in &RULES.newgame.common_initial_active_skills {
        chara
            .active_skills
            .push((ActiveSkillOrigin::Learned, active_skill.clone()));
    }
}

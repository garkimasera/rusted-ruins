
use common::gamedata;
use common::gamedata::GameData;
use common::gamedata::map::MapId;
use common::gamedata::region::RegionId;

pub struct NewGameBuilder {
    gd: GameData,
    player_name: Option<String>,
}

impl NewGameBuilder {
    pub fn new() -> NewGameBuilder {
        NewGameBuilder {
            gd: GameData::empty(),
            player_name: None,
        }
    }

    pub fn set_player_name(&mut self, name: &str) {
        self.player_name = Some(name.to_owned());
    }

    pub fn build(mut self) -> GameData {
        {
            let mut gd = &mut self.gd;

            super::region::add_region(&mut gd, "!east-coast");

            let mut mid = MapId::RegionMap { rid: RegionId::default() };
            gd.set_current_mapid(mid);
            let start_pos = gd.get_current_map().entrance;

            super::region::gen_dungeon(&mut gd, mid.rid());

            let mut chara = gamedata::chara::Chara::default();
            chara.base_params.spd = 100;
            chara.base_params.str = 25;
            chara.rel = gamedata::chara::Relationship::ALLY;
            //chara.name = self.player_name.unwrap();
            super::chara::update_params(&mut chara);
            /* Test code for equipment */
            use common::gamedata::chara::Race;
            let slots = &::rules::RULES.chara_gen.default_equip_slots.get(&Race::Human).unwrap();
            let equip = gamedata::item::EquipItemList::new(slots);
            chara.equip = equip;
            gd.add_chara_to_map(chara, gamedata::chara::CharaKind::Player, mid, start_pos);
            /*
            /* Test code for talk */
            let mut chara = super::chara::creation::create_npc_chara(
            ::common::gamedata::site::DungeonKind::Cave, 10);
            chara.rel = ::common::gamedata::chara::Relationship::FRIENDLY;
            chara.talk = Some(::common::gamedata::chara::CharaTalk {
            id: "!hello".to_owned(),
            section: "start".to_owned(),
            event_data: None,
        });
            gd.add_chara_to_map(chara, ::common::gamedata::chara::CharaKind::OnMap, mid, start_pos + (0, 2));
             */

        }
        self.gd
    }
}


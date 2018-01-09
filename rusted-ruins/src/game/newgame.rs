
use common::gamedata;
use common::gamedata::GameData;
use super::site::add_dungeon_site;
use super::map;

pub fn create_newgame() -> GameData {
    let mut gd = GameData::empty();

    let region = gamedata::region::Region::new();
    gd.region.add_region(region);
    
    add_dungeon_site(&mut gd);

    let mid = gd.get_current_mapid();
    let start_pos = gd.get_current_map().entrance;

    let mut chara = gamedata::chara::Chara::default();
    chara.params.spd = 100;
    chara.params.str = 25;
    chara.rel = gamedata::chara::Relationship::ALLY;
    /* Test code for equipment */
    use common::gamedata::chara::Race;
    let slots = &::rules::RULES.chara_gen.default_equip_slots.get(&Race::Human).unwrap();
    let equip = gamedata::item::EquipItemList::new(slots);
    chara.equip = equip;
    gd.add_chara_to_map(chara, gamedata::chara::CharaKind::Player, mid, start_pos);
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

    map::gen_npcs(&mut gd, mid, 10, 10);
    map::gen_items(&mut gd, mid);

    gd
}


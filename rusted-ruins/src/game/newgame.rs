
use common::gamedata;
use common::gamedata::GameData;
use super::site::add_dungeon_site;

pub fn create_newgame() -> GameData {
    let mut gd = GameData::empty();

    
    //add_dungeon_site(&mut gd);
    let site = gamedata::site::Site::new("Ruin 0", 10);
    let mut map = super::map::builder::MapBuilder::new(40, 40).build();
    let sid = gd.add_site(site, gamedata::site::SiteKind::Start);
    let mid = gd.add_map(map, sid);

    let mut chara = gamedata::chara::Chara::default();
    chara.params.spd = 100;
    chara.params.str = 25;
    chara.rel = gamedata::chara::Relationship::ALLY;
    gd.add_chara_to_map(chara, gamedata::chara::CharaKind::Player, mid, ::array2d::Vec2d(15, 15));

    let mut chara = gamedata::chara::Chara::default();
    chara.params.spd = 100;
    chara.params.str = 20;
    chara.rel = gamedata::chara::Relationship::HOSTILE;
    gd.add_chara_to_map(chara, gamedata::chara::CharaKind::OnMap, mid, ::array2d::Vec2d(2, 2));

    gd
}


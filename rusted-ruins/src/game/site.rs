
use common::gamedata::GameData;
use common::gamedata::site::Site;

pub fn add_dungeon_site(gd: &mut GameData) {
    let mut site = Site::new("Ruin 0", 1);

    let mut map = super::map::builder::MapBuilder::new(10, 10);
    
}


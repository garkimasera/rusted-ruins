
use common::gamedata::GameData;
use common::gamedata::region::*;

pub fn add_region(gd: &mut GameData, id: &str) {
    let map = super::map::from_template::from_template_id("!east-coast");
    
    let region = Region::new(id, map);
    gd.region.add_region(region);
}


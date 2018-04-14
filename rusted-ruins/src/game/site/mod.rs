
pub mod gen;

use std::borrow::Cow;
use array2d::*;
use common::gamedata::*;
use common::gobj;
use rules::RULES;
use super::map::builder::MapBuilder;
use text;

pub fn add_dungeon_site(gd: &mut GameData, dungeon_kind: DungeonKind, pos: Vec2d) -> SiteId {
    let mut site = Site::new(10);
    site.content = SiteContent::AutoGenDungeon { dungeon_kind };
    let sid = gd.add_site(site, SiteKind::AutoGenDungeon, RegionId::default(), pos).unwrap();
    extend_site_floor(gd, sid);
    
    sid
}

/// Extend dungion site by one floor
pub fn extend_site_floor(gd: &mut GameData, sid: SiteId) {
    let floor = gd.region.get_site(sid).floor_num();
    let map = match gd.region.get_site(sid).content {
        SiteContent::AutoGenDungeon { dungeon_kind } => {
            let map_size = RULES.dungeon_gen[&dungeon_kind].map_size;
            let tile_idx = gobj::id_to_idx(&RULES.dungeon_gen[&dungeon_kind].terrain[0][0]);
            let wall_idx = gobj::id_to_idx(&RULES.dungeon_gen[&dungeon_kind].terrain[0][1]);
            MapBuilder::new(map_size.0 as u32, map_size.1 as u32)
                .floor(floor)
                .tile(tile_idx)
                .wall(wall_idx)
                .deepest_floor(floor >= gd.region.get_site(sid).max_floor() - 1)
                .build()
        }
        _ => {
            MapBuilder::new(40, 40).floor(floor).build()
        }
    };
    
    let mid = gd.add_map(map, sid);
    super::map::gen_npcs(gd, mid, 10, mid.floor());
    super::map::gen_items(gd, mid);
}

/// Additional Site method
pub trait SiteEx {
    fn get_name(&self) -> Cow<str>;
}

impl SiteEx for Site {
    fn get_name(&self) -> Cow<str> {
        if let Some(ref name) = self.name {
            let name: &str = &*name;
            return name.into();
        }
        
        match self.content {
            SiteContent::AutoGenDungeon { dungeon_kind } => {
                text::to_txt(&dungeon_kind).into()
            }
            SiteContent::Town { ref town } => {
                text::obj_txt(town.id()).into()
            }
            SiteContent::Other => {
                warn!("Unnamed other kind site");
                "".into()
            }
        }
    }
}

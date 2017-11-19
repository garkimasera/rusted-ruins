
pub mod item;
pub mod chara;
pub mod map;
pub mod site;

use array2d::Vec2d;

use self::chara::*;
use self::map::*;
use self::site::*;


/// Includes all data for one game
#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub site: SiteHolder,
    pub chara: CharaHolder,
    current_mapid: MapId,
}

impl GameData {
    pub fn empty() -> GameData {
        GameData {
            site: SiteHolder::new(),
            chara: CharaHolder::new(),
            current_mapid: map::STARTING_MAP_ID,
        }
    }

    pub fn get_current_mapid(&self) -> MapId {
        self.current_mapid
    }

    pub fn get_current_map(&self) -> &Map {
        self.site.get_map(self.current_mapid)
    }

    pub fn get_current_map_mut(&mut self) -> &mut Map {
        self.site.get_map_mut(self.current_mapid)
    }

    pub fn add_chara(&mut self, chara: Chara, kind: CharaKind) -> CharaId {
        match kind {
            CharaKind::Player => {
                self.chara.0.insert(CharaId::Player, chara);
                CharaId::Player
            }
            CharaKind::OnMap => {
                panic!("Adding OnMap chara without mapid is unavailable")
            }
        }
    }

    pub fn add_chara_to_map(&mut self, chara: Chara, kind: CharaKind, mid: MapId, pos: Vec2d) -> CharaId {
        match kind {
            CharaKind::Player => {
                self.chara.0.insert(CharaId::Player, chara);
                let map = self.site.get_map_mut(mid);
                map.add_chara(pos, CharaId::Player);
                CharaId::Player
            }
            CharaKind::OnMap => {
                let cid = CharaId::OnMap { mid, n: self.site.get_map(mid).search_empty_onmap_charaid_n() };
                self.chara.0.insert(cid, chara);
                self.site.get_map_mut(mid).add_chara(pos, cid);
                cid
            }
        }
    }

    pub fn remove_chara(&mut self, cid: CharaId) {
        match cid {
            CharaId::Player => {
                panic!();
            }
            CharaId::OnMap { mid, .. } => {
                let map = self.site.get_map_mut(mid);
                
                self.chara.remove_chara(cid);
                map.remove_chara(cid);
            }
        }
    }

    pub fn add_site(&mut self, site: Site, kind: SiteKind) -> SiteId {
        match kind {
            SiteKind::Start => {
                let sid = SiteId::Start;
                self.site.0.insert(sid, site);
                sid
            }
            SiteKind::AutoGenDungeon => {
                let sid = SiteId::AutoGenDungeon { n: 0 };
                self.site.0.insert(sid, site);
                sid
            }
        }
    }

    pub fn add_map(&mut self, map: Map, sid: SiteId) -> MapId {
        let site = self.site.get_mut(sid);
        let floor = site.add_map(map);
        MapId { sid, floor }
    }

    pub fn set_current_mapid(&mut self, mid: MapId) {
        self.current_mapid = mid;
    }
}

fn unknown_id_err<T: ::std::fmt::Debug>(id: T) -> String {
    format!("Internal error: Unknown id - {:?}", id)
}


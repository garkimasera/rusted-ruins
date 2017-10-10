
pub mod item;
pub mod chara;
pub mod map;
pub mod site;

use std::collections::HashMap;
use array2d::Vec2d;

use self::chara::{Chara, CharaId, CharaKind};
use self::map::{Map, MapId};
use self::site::{Site, SiteKind, SiteId};


/// Includes all data for one game
#[derive(Serialize, Deserialize)]
pub struct GameData {
    site: HashMap<site::SiteId, site::Site>,
    chara: HashMap<chara::CharaId, chara::Chara>,
    current_mapid: MapId,
}

impl GameData {
    pub fn empty() -> GameData {
        GameData {
            site: HashMap::new(),
            chara: HashMap::new(),
            current_mapid: map::STARTING_MAP_ID,
        }
    }
    
    pub fn get_chara(&self, id: chara::CharaId) -> &chara::Chara {
        self.chara.get(&id).expect(&unknown_id_err(id))
    }

    pub fn get_chara_mut(&mut self, id: chara::CharaId) -> &mut chara::Chara {
        self.chara.get_mut(&id).expect(&unknown_id_err(id))
    }

    pub fn get_map(&self, id: MapId) -> &Map {
        self.site.get(&id.sid).expect(&unknown_id_err(id)).get_map(id.floor)
    }

    pub fn get_map_mut(&mut self, id: MapId) -> &mut Map {
        self.site.get_mut(&id.sid).expect(&unknown_id_err(id)).get_map_mut(id.floor)
    }

    pub fn get_current_map(&self) -> &Map {
        self.get_map(self.current_mapid)
    }

    pub fn get_current_map_mut(&mut self) -> &mut Map {
        let current_mapid = self.current_mapid;
        self.get_map_mut(current_mapid)
    }

    pub fn iter_charaid(&self) -> ::std::collections::hash_map::Keys<CharaId, Chara> {
        self.chara.keys()
    }

    pub fn add_chara(&mut self, chara: Chara, kind: CharaKind) -> CharaId {
        match kind {
            CharaKind::Player => {
                self.chara.insert(CharaId::Player, chara);
                CharaId::Player
            },
            CharaKind::OnMap => {
                panic!("Adding OnMap chara without mapid is unavailable")
            },
        }
    }

    pub fn add_chara_to_map(&mut self, chara: Chara, kind: CharaKind, mid: MapId, pos: Vec2d) -> CharaId {
        match kind {
            CharaKind::Player => {
                self.chara.insert(CharaId::Player, chara);
                let mut map = self.get_map_mut(mid);
                map.add_chara(pos, CharaId::Player);
                CharaId::Player
            },
            CharaKind::OnMap => {
                let cid = CharaId::OnMap { mid, n: self.get_map(mid).search_empty_onmap_charaid_n() };
                self.chara.insert(cid, chara);
                self.get_map_mut(mid).add_chara(pos, cid);
                cid
            },
        }
    }

    pub fn add_site(&mut self, site: Site, kind: SiteKind) -> SiteId {
        match kind {
            SiteKind::Start => {
                let sid = SiteId::Start;
                self.site.insert(sid, site);
                sid
            },
            SiteKind::AutoGenDungeon => {
                let sid = SiteId::AutoGenDungeon { n: 0 };
                self.site.insert(sid, site);
                sid
            },
        }
    }

    pub fn add_map(&mut self, map: Map, sid: SiteId) -> MapId {
        let mut site = self.site.get_mut(&sid).expect(&unknown_id_err(sid));
        let floor = site.add_map(map);
        MapId { sid, floor }
    }
}

fn unknown_id_err<T: ::std::fmt::Debug>(id: T) -> String {
    format!("Internal error: Unknown id - {:?}", id)
}



pub mod item;
pub mod chara;
pub mod map;
pub mod site;

use array2d::Vec2d;

use self::chara::*;
use self::map::*;
use self::site::*;
use self::item::*;


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

    // Fuctions for item handling

    /// Get item list by ItemListLocation
    pub fn get_item_list(&self, list_location: ItemListLocation) -> &ItemList {
        match list_location {
            ItemListLocation::Chara { cid } => {
                &self.chara.get(cid).item_list
            }
            ItemListLocation::Equip { cid } => {
                self.chara.get(cid).equip.list()
            }
            ItemListLocation::OnMap { mid, pos } => {
                &self.site.get_map(mid).tile[pos].item_list.as_ref().expect("Get item list to empty tile")
            }
        }
    }

    /// Mutable version for get_item_list
    pub fn get_item_list_mut(&mut self, list_location: ItemListLocation) -> &mut ItemList {
        match list_location {
            ItemListLocation::Chara { cid } => {
                &mut self.chara.get_mut(cid).item_list
            }
            ItemListLocation::Equip { .. } => {
                panic!("Mutable borrow is prohibited for equipment list");
            }
            ItemListLocation::OnMap { mid, pos } => {
                self.site.get_map_mut(mid).tile[pos].item_list.as_mut()
                    .expect("Get item list to empty tile")
            }
        }
    }

    pub fn get_filtered_item_list(&self, list_location: ItemListLocation, filter: ItemFilter)
                                  -> FilteredItemList {
        let item_list = self.get_item_list(list_location);
        FilteredItemList::new(item_list, list_location, filter)
    }

    /// Remove item from list
    pub fn remove_item<T: Into<ItemMoveNum>>(&mut self, item_location: ItemLocation, n: T) {
        {
            let item_list = self.get_item_list_mut(item_location.0);
            item_list.remove(item_location.1, n);
        }
        self.check_item_list_on_tile(item_location.0);
    }

    /// Remove item from list and get its clone or moved value
    pub fn remove_item_and_get<T: Into<ItemMoveNum>>(&mut self, item_location: ItemLocation, n: T)
                                             -> Box<Item> {
        let result = {
            let item_list = self.get_item_list_mut(item_location.0);
            item_list.remove_and_get(item_location.1, n)
        };
        self.check_item_list_on_tile(item_location.0);
        result
    }

    /// Move item to dest
    /// If destination list is full, returns false and does nothing
    pub fn move_item<T: Into<ItemMoveNum>>(&mut self, item_location: ItemLocation,
                                           dest: ItemListLocation, n: T) -> bool {
        let (item, n) = {
            let src_list = self.get_item_list_mut(item_location.0);
            let n = match n.into() {
                ItemMoveNum::Partial(n) => n,
                ItemMoveNum::All => {
                    src_list.get_number(item_location.1)
                }
            };
            (src_list.remove_and_get(item_location.1, n), n)
        };
        {
            let dest_list = self.get_item_list_mut(dest);
            if !dest_list.has_empty() { return false; }
            dest_list.append(item, n);
        }
        self.check_item_list_on_tile(item_location.0);
        true
    }

    /// Checks item list on tile is empty or not. If so, delete
    fn check_item_list_on_tile(&mut self, item_list_location: ItemListLocation) {
        match item_list_location {
            ItemListLocation::OnMap { mid, pos } => {
                if self.get_item_list(item_list_location).is_empty() {
                    self.site.get_map_mut(mid).tile[pos].item_list = None;
                }
            }
            _ => (),
        }
    }

    pub fn get_equip_list(&self, cid: CharaId) -> &EquipItemList {
        let chara = self.chara.get(cid);
        &chara.equip
    }

    pub fn get_equip_list_mut(&mut self, cid: CharaId) -> &mut EquipItemList {
        let chara = self.chara.get_mut(cid);
        &mut chara.equip
    }
}

fn unknown_id_err<T: ::std::fmt::Debug>(id: T) -> String {
    format!("Internal error: Unknown id - {:?}", id)
}


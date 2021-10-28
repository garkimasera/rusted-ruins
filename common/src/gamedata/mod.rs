pub mod chara;
pub mod defs;
pub mod effect;
pub mod faction;
pub mod item;
pub mod item_attr;
pub mod learned_recipes;
pub mod map;
pub mod meta;
pub mod modifier;
pub mod play_time;
pub mod player;
pub mod quest;
pub mod region;
pub mod script_exec;
pub mod settings;
pub mod shop;
pub mod site;
pub mod skill;
pub mod time;
pub mod town;
pub mod traits;
pub mod variables;

use geom::Vec2d;

pub use self::chara::*;
pub use self::defs::*;
pub use self::effect::*;
pub use self::faction::*;
pub use self::item::*;
pub use self::item_attr::*;
pub use self::learned_recipes::*;
pub use self::map::*;
pub use self::meta::*;
pub use self::modifier::*;
pub use self::play_time::*;
pub use self::player::*;
pub use self::quest::*;
pub use self::region::*;
pub use self::script_exec::*;
pub use self::settings::*;
pub use self::shop::*;
pub use self::site::*;
pub use self::skill::*;
pub use self::time::*;
pub use self::town::*;
pub use self::traits::*;
pub use self::variables::*;

/// Includes all data for one game world.
/// This can be a snapshot of the current game, so it must implement Serialize and Deserialize.
#[derive(Serialize, Deserialize)]
pub struct GameData {
    #[serde(skip)]
    pub meta: MetaData,
    pub chara: CharaHolder,
    pub region: RegionHolder,
    pub time: GameTime,
    pub play_time: PlayTime,
    pub player: Player,
    pub quest: QuestHolder,
    pub vars: Variables,
    pub faction: Faction,
    pub learned_recipes: LearnedRecipes,
    pub script_exec: ScriptExec,
    pub settings: Settings,
    current_mapid: MapId,
}

impl Default for GameData {
    fn default() -> Self {
        GameData::empty()
    }
}

impl GameData {
    pub fn empty() -> GameData {
        GameData {
            meta: MetaData::default(),
            chara: CharaHolder::new(),
            region: RegionHolder::default(),
            time: GameTime::default(),
            player: Player::default(),
            play_time: PlayTime::default(),
            quest: QuestHolder::default(),
            vars: Variables::default(),
            faction: Faction::default(),
            script_exec: ScriptExec::default(),
            settings: Settings::new(),
            learned_recipes: LearnedRecipes::default(),
            current_mapid: MapId::default(),
        }
    }

    pub fn get_current_mapid(&self) -> MapId {
        self.current_mapid
    }

    pub fn get_current_map(&self) -> &Map {
        self.region.get_map(self.current_mapid)
    }

    pub fn get_current_map_mut(&mut self) -> &mut Map {
        self.region.get_map_mut(self.current_mapid)
    }

    pub fn get_current_region_id(&self) -> RegionId {
        let mapid = self.get_current_mapid();
        mapid.rid()
    }

    pub fn get_current_region(&self) -> &Region {
        self.region.get(self.current_mapid.rid())
    }

    pub fn get_current_region_mut(&mut self) -> &mut Region {
        self.region.get_mut(self.current_mapid.rid())
    }

    pub fn get_charas_on_map(&self) -> Vec<CharaId> {
        let map = self.get_current_map();
        map.iter_charaid().copied().collect()
    }

    pub fn add_chara(&mut self, cid: CharaId, chara: Chara) {
        assert!(!matches!(cid, CharaId::OnMap { .. }));
        assert!(!self.chara.exist(cid));
        self.chara.add(cid, chara);
    }

    /// Add chara as OnMap
    pub fn add_chara_to_map(&mut self, chara: Chara, mid: MapId) -> CharaId {
        let cid = CharaId::OnMap {
            mid,
            n: self.region.get_map(mid).search_empty_onmap_charaid_n(),
        };

        if mid == self.current_mapid {
            self.chara.add(cid, chara);
        } else {
            let map = self.region.get_map_mut(mid);
            map.charas.as_mut().unwrap().insert(cid, chara);
        }
        cid
    }

    /// Remove specified character from the current map.
    /// If the chara is not OnMap, removes from the current map, but remains the data.
    pub fn remove_chara_from_map(&mut self, cid: CharaId) {
        match cid {
            CharaId::Player => {
                panic!();
            }
            CharaId::OnMap { .. } => {
                self.get_current_map_mut().remove_chara(cid);
                self.chara.remove_chara(cid);
            }
            _ => {
                self.get_current_map_mut().remove_chara(cid);
            }
        }
    }

    /// Remove specified character from game.
    /// If the character is in the current map, remove from map data
    pub fn remove_chara(&mut self, cid: CharaId) {
        match cid {
            CharaId::Player => {
                panic!();
            }
            CharaId::OnMap { mid, .. } => {
                let map = self.region.get_map_mut(mid);
                map.remove_chara(cid);
            }
            _ => {
                self.get_current_map_mut().remove_chara(cid);
            }
        }
        self.chara.remove_chara(cid);
    }

    pub fn add_site(
        &mut self,
        site: Site,
        kind: SiteKind,
        rid: RegionId,
        pos: Option<Vec2d>,
    ) -> Option<SiteId> {
        let region = self.region.get_mut(rid);
        region.add_site(site, kind, pos)
    }

    pub fn remove_site(&mut self, sid_to_remove: SiteId) {
        self.region.remove_site(sid_to_remove);
        self.chara
            .retain(|cid, _| !matches!(cid, &CharaId::OnSite { sid, .. } if sid == sid_to_remove));
    }

    pub fn add_map(&mut self, map: Map, sid: SiteId, map_random_id: u64) -> MapId {
        let site = self.region.get_site_mut(sid);
        let floor = site.add_map(map, map_random_id);
        MapId::SiteMap { sid, floor }
    }

    pub fn set_current_mapid(&mut self, mid: MapId) {
        if mid == self.current_mapid {
            return;
        }
        // OnMap characters on the next map
        let next_charas = self
            .region
            .get_map_mut(mid)
            .charas
            .take()
            .expect("Map.charas is empty");
        let prev_charas = self.chara.replace_on_map_chara(next_charas);
        let map = self.get_current_map_mut();
        assert!(map.charas.is_none());
        map.charas = Some(prev_charas);

        // Update current_mapid
        self.current_mapid = mid;
    }

    pub fn set_initial_mapid(&mut self, mid: MapId) {
        let charas = self.region.get_map_mut(mid).charas.take().unwrap();
        self.chara.replace_on_map_chara(charas);
        self.current_mapid = mid;
    }

    // Fuctions for item handling

    /// Get item list by ItemListLocation
    pub fn get_item_list(&self, list_location: ItemListLocation) -> &ItemList {
        match list_location {
            ItemListLocation::Chara { cid } => &self.chara.get(cid).item_list,
            ItemListLocation::Equip { cid } => self.chara.get(cid).equip.list(),
            ItemListLocation::OnMap { mid, pos } => &self.region.get_map(mid).tile[pos].item_list,
            ItemListLocation::Shop { cid } => &self.get_shop(cid).items,
            ItemListLocation::Container { ill, i } => {
                let item = self.get_item((ill.into(), i));

                item.0
                    .attrs
                    .iter()
                    .filter_map(|attr| match attr {
                        ItemAttr::Container(container) => Some(container.as_ref()),
                        _ => None,
                    })
                    .next()
                    .expect("invalid ItemListLocation for container item")
            }
        }
    }

    /// Mutable version for get_item_list
    pub fn get_item_list_mut(&mut self, list_location: ItemListLocation) -> &mut ItemList {
        match list_location {
            ItemListLocation::Chara { cid } => &mut self.chara.get_mut(cid).item_list,
            ItemListLocation::Equip { .. } => {
                panic!("Mutable borrow is prohibited for equipment list");
            }
            ItemListLocation::OnMap { mid, pos } => {
                &mut self.region.get_map_mut(mid).tile[pos].item_list
            }
            ItemListLocation::Shop { cid } => &mut self.get_shop_mut(cid).items,
            ItemListLocation::Container { ill, i } => {
                let item = self.get_item_mut((ill.into(), i));

                item.0
                    .attrs
                    .iter_mut()
                    .filter_map(|attr| match attr {
                        ItemAttr::Container(container) => Some(container.as_mut()),
                        _ => None,
                    })
                    .next()
                    .expect("invalid ItemListLocation for container item")
            }
        }
    }

    /// Get item list on the current map
    pub fn get_item_list_on_current_map(&self, pos: Vec2d) -> &ItemList {
        let mid = self.get_current_mapid();
        &self.region.get_map(mid).tile[pos].item_list
    }

    pub fn get_item(&self, item_location: ItemLocation) -> (&Item, u32) {
        let a = &self.get_item_list(item_location.0).items[item_location.1 as usize];
        (&a.0, a.1)
    }

    pub fn get_item_mut(&mut self, il: ItemLocation) -> (&mut Item, u32) {
        match il.0 {
            ItemListLocation::Equip { cid } => {
                let list = self.get_equip_list_mut(cid);
                let item = &mut list.list_mut().items[il.1 as usize].0;
                (item, 1)
            }
            _ => {
                let a = &mut self.get_item_list_mut(il.0).items[il.1 as usize];
                (&mut a.0, a.1)
            }
        }
    }

    /// Remove item from list
    pub fn remove_item<T: Into<ItemMoveNum>>(&mut self, item_location: ItemLocation, n: T) {
        let item_list = self.get_item_list_mut(item_location.0);
        item_list.remove(item_location.1, n);
    }

    /// Remove item from list and get its clone or moved value
    pub fn remove_item_and_get<T: Into<ItemMoveNum>>(
        &mut self,
        item_location: ItemLocation,
        n: T,
    ) -> Item {
        let item_list = self.get_item_list_mut(item_location.0);
        item_list.remove_and_get(item_location.1, n)
    }

    pub fn get_equip_list(&self, cid: CharaId) -> &EquipItemList {
        let chara = self.chara.get(cid);
        &chara.equip
    }

    pub fn get_equip_list_mut(&mut self, cid: CharaId) -> &mut EquipItemList {
        let chara = self.chara.get_mut(cid);
        &mut chara.equip
    }

    pub fn equipment_item_location(
        &self,
        cid: CharaId,
        esk: EquipSlotKind,
        n: usize,
    ) -> Option<ItemLocation> {
        let list = self.get_equip_list(cid);
        list.list_idx(esk, n)
            .map(|i| (ItemListLocation::Equip { cid }, i as u32))
    }

    pub fn get_shop(&self, cid: CharaId) -> &Shop {
        let (sid, id) = match cid {
            CharaId::OnSite { sid, id } => (sid, id),
            _ => panic!("Tried to get shop according to no OnSite character"),
        };
        let site = self.region.get_site(sid);
        match &site.content {
            SiteContent::Town { ref town } => town
                .get_shop(id)
                .unwrap_or_else(|| panic!("Shop #{} doesnot exit in {:?}", id, sid)),
            _ => unreachable!("Tried to get shop in a site that is not town"),
        }
    }

    pub fn get_shop_mut(&mut self, cid: CharaId) -> &mut Shop {
        let (sid, id) = match cid {
            CharaId::OnSite { sid, id } => (sid, id),
            _ => panic!("Tried to get shop according to no OnSite character"),
        };
        let site = self.region.get_site_mut(sid);
        match &mut site.content {
            SiteContent::Town { ref mut town } => town
                .get_shop_mut(id)
                .unwrap_or_else(|| panic!("Shop #{} doesnot exit in {:?}", id, sid)),
            _ => unreachable!("Tried to get shop in a site that is not town"),
        }
    }
}

fn unknown_id_err<T: std::fmt::Debug>(id: T) -> ! {
    panic!("Internal error: Unknown id - {:?}", id)
}

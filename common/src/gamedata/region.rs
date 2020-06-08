use super::map::*;
use super::site::*;
use super::unknown_id_err;
use filebox::FileBox;
use geom::*;
use std::collections::HashMap;
use std::path::Path;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct RegionId(pub(crate) u32);

/// Region represents "Region Map", and sites on it
#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    id: RegionId,
    pub(crate) sites: HashMap<SiteId, SiteInfo>,
    /// An map to represents this region
    pub(crate) map: BoxedMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteInfo {
    site: Site,
    /// Position on the region map
    pos: Vec2d,
}

#[derive(Serialize, Deserialize)]
pub struct RegionHolder(pub(crate) HashMap<RegionId, Region>);

impl Default for RegionId {
    fn default() -> RegionId {
        RegionId(0)
    }
}

impl RegionHolder {
    pub fn new() -> RegionHolder {
        RegionHolder(HashMap::new())
    }

    pub fn get(&self, rid: RegionId) -> &Region {
        self.0.get(&rid).unwrap_or_else(|| unknown_id_err(rid))
    }

    pub fn get_mut(&mut self, rid: RegionId) -> &mut Region {
        self.0.get_mut(&rid).unwrap_or_else(|| unknown_id_err(rid))
    }

    pub fn get_checked(&self, rid: RegionId) -> Option<&Region> {
        self.0.get(&rid)
    }

    pub fn get_mut_checked(&mut self, rid: RegionId) -> Option<&mut Region> {
        self.0.get_mut(&rid)
    }

    pub fn get_site(&self, sid: SiteId) -> &Site {
        let region = self
            .0
            .get(&sid.rid)
            .unwrap_or_else(|| unknown_id_err(sid.rid));
        &region
            .sites
            .get(&sid)
            .unwrap_or_else(|| unknown_id_err(sid))
            .site
    }

    pub fn get_site_mut(&mut self, sid: SiteId) -> &mut Site {
        let region = self
            .0
            .get_mut(&sid.rid)
            .unwrap_or_else(|| unknown_id_err(sid.rid));
        &mut region
            .sites
            .get_mut(&sid)
            .unwrap_or_else(|| unknown_id_err(sid))
            .site
    }

    pub fn get_site_pos(&self, sid: SiteId) -> Vec2d {
        let region = self
            .0
            .get(&sid.rid)
            .unwrap_or_else(|| unknown_id_err(sid.rid));
        region
            .sites
            .get(&sid)
            .unwrap_or_else(|| unknown_id_err(sid))
            .pos
    }

    pub fn get_map(&self, mid: MapId) -> &Map {
        match mid {
            MapId::SiteMap { sid, floor } => self.get_site(sid).get_map(floor),
            MapId::RegionMap { rid } => &self.get(rid).map,
        }
    }

    pub fn get_map_mut(&mut self, mid: MapId) -> &mut Map {
        match mid {
            MapId::SiteMap { sid, floor } => self.get_site_mut(sid).get_map_mut(floor),
            MapId::RegionMap { rid } => &mut self.get_mut(rid).map,
        }
    }

    pub fn get_boxed_map_mut(&mut self, mid: MapId) -> &mut BoxedMap {
        match mid {
            MapId::SiteMap { sid, floor } => self.get_site_mut(sid).get_boxed_map_mut(floor),
            MapId::RegionMap { rid } => &mut self.get_mut(rid).map,
        }
    }

    pub fn get_site_checked(&self, sid: SiteId) -> Option<&Site> {
        let region = self.0.get(&sid.rid)?;
        Some(&region.sites.get(&sid)?.site)
    }

    pub fn get_site_mut_checked(&mut self, sid: SiteId) -> Option<&mut Site> {
        let region = self.0.get_mut(&sid.rid)?;
        Some(&mut region.sites.get_mut(&sid)?.site)
    }

    pub fn map_exist(&self, mid: MapId) -> bool {
        match mid {
            MapId::SiteMap { sid, floor } => {
                if let Some(site) = self.get_site_checked(sid) {
                    site.map_exist(floor)
                } else {
                    false
                }
            }
            MapId::RegionMap { rid } => self.0.contains_key(&rid),
        }
    }

    pub fn get_map_checked(&self, mid: MapId) -> Option<&Map> {
        match mid {
            MapId::SiteMap { sid, floor } => {
                let site = self.get_site_checked(sid)?;
                site.get_map_checked(floor)
            }
            MapId::RegionMap { rid } => Some(&self.get_checked(rid)?.map),
        }
    }

    pub fn get_map_checked_mut(&self, mid: MapId) -> Option<&Map> {
        match mid {
            MapId::SiteMap { sid, floor } => {
                let site = self.get_site_checked(sid)?;
                site.get_map_checked(floor)
            }
            MapId::RegionMap { rid } => Some(&self.get_checked(rid)?.map),
        }
    }

    /// Preload map from file
    pub fn preload_map<P: AsRef<Path>>(&mut self, mid: MapId, map_dir_path: P) {
        info!("preload map {:?}", mid);
        let boxed_map = self.get_boxed_map_mut(mid);
        match boxed_map.read(map_dir_path) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
                unimplemented!()
            }
        }
    }

    pub fn get_map_mut_checked(&mut self, mid: MapId) -> Option<&mut Map> {
        match mid {
            MapId::SiteMap { sid, floor } => {
                let site = self.get_site_mut_checked(sid)?;
                site.get_map_mut_checked(floor)
            }
            MapId::RegionMap { rid } => Some(&mut self.get_mut_checked(rid)?.map),
        }
    }

    pub fn add_region(&mut self, mut region: Region) -> RegionId {
        // Search unused id
        for i in 0.. {
            let rid = RegionId(i);
            if self.0.get(&rid).is_none() {
                region.id = rid;
                self.0.insert(rid, region);
                return rid;
            }
        }
        unreachable!()
    }

    pub fn visit_all_maps<F: FnMut(MapId, &BoxedMap)>(&self, mut f: F) {
        for (&rid, region) in &self.0 {
            let mid = MapId::RegionMap { rid };
            f(mid, &region.map);

            for (&sid, siteinfo) in &region.sites {
                siteinfo.site.visit_maps(|floor, map| {
                    let mid = MapId::SiteMap { sid, floor };
                    f(mid, map);
                });
            }
        }
    }

    /// Covnert map path to MapId and pos.
    pub fn path_to_map_id_and_pos(&self, path: &str) -> Option<(MapId, Vec2d)> {
        fn floor_and_pos(a: &str) -> Option<(u32, Vec2d)> {
            let a: Vec<&str> = a.split(':').collect();
            if a.len() != 2 {
                return None;
            }
            let floor: u32 = if let Ok(floor) = a[0].parse() {
                floor
            } else {
                return None;
            };
            let pos: Vec<&str> = a[1].split(',').collect();
            let pos_x: u32 = if let Ok(pos_x) = pos[0].parse() {
                pos_x
            } else {
                return None;
            };
            let pos_y: u32 = if let Ok(pos_y) = pos[1].parse() {
                pos_y
            } else {
                return None;
            };
            Some((floor, Vec2d::from((pos_x, pos_y))))
        }

        let path_elements: Vec<&str> = path.split('/').collect();

        if path_elements.len() == 2 {
            let region_name = path_elements[0];
            let (floor, pos) = if let Some(a) = floor_and_pos(path_elements[1]) {
                a
            } else {
                return None;
            };
            for (&rid, region) in self.0.iter() {
                if region.name == region_name {
                    return Some((MapId::RegionMap { rid }, pos));
                }
            }
        } else if path_elements.len() == 3 {
            let region_name = path_elements[0];
            let site_name = path_elements[1];
            let (floor, pos) = if let Some(a) = floor_and_pos(path_elements[2]) {
                a
            } else {
                return None;
            };

            for (_, region) in self.0.iter() {
                if region.name != region_name {
                    continue;
                }
                for (&sid, site) in region.sites.iter() {
                    if site.site.id.is_some() && site.site.id.as_ref().unwrap() == site_name {
                        return Some((MapId::SiteMap { sid, floor }, pos));
                    }
                }
            }
        }
        None
    }
}

impl Region {
    pub fn new(name: &str, map: Map, map_random_id: u64) -> Region {
        Region {
            name: name.to_owned(),
            id: RegionId(0),
            sites: HashMap::new(),
            map: FileBox::new(map_random_id, map),
        }
    }

    /// Add new site to region
    /// If already site is existed, this function will fail and return None
    pub fn add_site(&mut self, site: Site, kind: SiteKind, pos: Vec2d) -> Option<SiteId> {
        // Calculate new number for the given site
        let n = self.search_empty_n(kind);
        let sid = SiteId {
            rid: self.id,
            kind,
            n,
        };
        let site_info = SiteInfo { site, pos };
        self.sites.insert(sid, site_info);
        Some(sid)
    }

    /// Get the number of sites on the region
    pub fn get_site_n(&self, kind: SiteKind) -> u32 {
        self.sites.keys().filter(|&sid| sid.kind == kind).count() as u32
    }

    /// Get site by position on the region
    pub fn get_id_by_pos(&self, pos: Vec2d) -> Option<SiteId> {
        for (sid, sinfo) in self.sites.iter() {
            if sinfo.pos == pos {
                return Some(*sid);
            }
        }
        None
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn get_map_mut(&mut self) -> &mut Map {
        &mut self.map
    }

    fn search_empty_n(&self, kind: SiteKind) -> u32 {
        for n in 0.. {
            let sid = SiteId {
                rid: self.id,
                kind,
                n,
            };
            if self.sites.get(&sid).is_none() {
                return n;
            }
        }
        unreachable!()
    }
}

use crate::game::saveload::gen_box_id;
use common::gamedata::*;

pub fn gen_temp_site_from_map(
    gd: &mut GameData,
    rid: RegionId,
    map: Map,
    name: &str,
    site_content: SiteContent,
) -> MapId {
    assert!(site_content.kind() == SiteKind::Temp);
    let mut site = Site::new(1, None);
    site.content = site_content;
    site.name = Some(name.into());
    let sid = gd.add_site(site, SiteKind::Temp, rid, None).unwrap();
    let map_random_id = gen_box_id(gd);
    gd.add_map(map, sid, map_random_id)
}

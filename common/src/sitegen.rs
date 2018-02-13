
use gamedata::site::SiteKind;
use gamedata::shop::ShopKind;

/// Hold data for site generation
#[derive(Clone, Serialize, Deserialize)]
pub struct SiteGenObject {
    pub id: String,
    pub kind: SiteKind,
    pub map_template_id: Vec<String>,
    pub shops: Vec<ShopGenData>,
}

/// Data to generate shops on the site
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ShopGenData {
    pub pos: Vec2d,
    pub floor: u32,
    pub kind: ShopKind,
}


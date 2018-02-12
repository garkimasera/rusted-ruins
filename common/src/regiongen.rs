
use array2d::Vec2d;

/// Hold data for region generation
#[derive(Clone, Serialize, Deserialize)]
pub struct RegionGenObject {
    pub id: String,
    pub map_template_id: String,
    /// Id and position of SiteGenObject for towns
    pub towns: Vec<(String, Vec2d)>,
}


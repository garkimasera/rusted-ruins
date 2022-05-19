use geom::Coords;

/// Hold data for region generation
#[derive(Clone, Serialize, Deserialize)]
pub struct RegionGenObject {
    pub id: String,
    pub map_template_id: String,
    /// Id and position of SiteGenObject for towns
    pub towns: Vec<(String, Coords)>,
    /// Id and position of SiteGenObject for other sites
    pub others: Vec<(String, Coords)>,
}

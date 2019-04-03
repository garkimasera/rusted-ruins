use common::hashmap::HashMap;
use geom::Vec2d;

#[derive(Serialize, Deserialize)]
pub struct FloorGen {
    pub floor_gen_params: HashMap<String, FloorGenParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FloorGenParams {
    pub map_size: Vec2d,
    pub map_gen_kind: MapGenKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapGenKind {
    Flat,
    Fractal,
    Lattice,
    Rooms,
}

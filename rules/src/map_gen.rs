use common::hashmap::HashMap;
use map_generator::MapGenParam;

#[derive(Serialize, Deserialize)]
pub struct MapGen {
    pub map_gen_params: HashMap<String, MapGenParam>,
}

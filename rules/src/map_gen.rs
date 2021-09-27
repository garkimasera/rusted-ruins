use crate::Rule;
use common::hashmap::HashMap;
use map_generator::MapGenParam;

#[derive(Serialize, Deserialize)]
pub struct MapGen {
    pub map_gen_params: HashMap<String, MapGenParam>,
}

impl Rule for MapGen {
    const NAME: &'static str = "map_gen";

    fn append(&mut self, other: Self) {
        for (k, v) in other.map_gen_params.into_iter() {
            self.map_gen_params.insert(k, v);
        }
    }
}

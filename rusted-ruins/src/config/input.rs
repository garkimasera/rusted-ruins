
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct InputConfig {
    pub normal: HashMap<String, String>,
    pub dialog: HashMap<String, String>,
}


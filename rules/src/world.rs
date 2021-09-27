use crate::Rule;

/// Rules for game world
#[derive(Serialize, Deserialize)]
pub struct World {
    /// Restart map path
    pub restart_path: String,
    /// Script id to execute on restart
    pub restart_script: String,
}

impl Rule for World {
    const NAME: &'static str = "world";

    fn append(&mut self, other: Self) {
        *self = other;
    }
}

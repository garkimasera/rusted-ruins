use std::collections::HashMap;
use common::gamedata::{CreationKind, CreationRequiredTime, Recipe};

#[derive(Serialize, Deserialize)]
pub struct Creation {
    pub required_time: HashMap<CreationRequiredTime, u16>,
    cooking_recipes: Vec<Recipe>,
}

impl Creation {
    pub(crate) fn sort(&mut self) {
        self.cooking_recipes.sort();
    }

    pub fn recipes(&self, kind: CreationKind) -> &[Recipe] {
        match kind {
            CreationKind::Cooking => self.cooking_recipes.as_ref(),
        }
    }
}

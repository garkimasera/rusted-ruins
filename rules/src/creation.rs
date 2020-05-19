use common::gamedata::{CreationKind, CreationRequiredTime, Recipe};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Creation {
    pub required_time: HashMap<CreationRequiredTime, u16>,
    #[serde(default)]
    cooking_recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize)]
pub struct Recipes {
    cooking_recipes: Vec<Recipe>,
}

impl Creation {
    pub(crate) fn sort(&mut self) {
        self.cooking_recipes.sort();
    }

    pub fn join(&mut self, mut recipes: Recipes) {
        self.cooking_recipes.append(&mut recipes.cooking_recipes);
    }

    pub fn recipes(&self, kind: CreationKind) -> &[Recipe] {
        match kind {
            CreationKind::Cooking => self.cooking_recipes.as_ref(),
        }
    }

    pub fn join_from_dir(&mut self, dir: &Path) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if !file_type.is_file() {
                continue;
            }
            let path = entry.path();
            let extension = path.extension();
            if extension.is_none() {
                continue;
            }
            let extension = extension.unwrap();
            if extension != "json" {
                continue;
            }

            let recipes: Recipes = super::read_from_json(&path);
            self.join(recipes);
        }
        Ok(())
    }
}

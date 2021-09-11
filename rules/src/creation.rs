use common::gamedata::{CreationKind, CreationRequiredTime, Recipe};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Creation {
    pub required_time: HashMap<CreationRequiredTime, u16>,
    pub recipe_learning_level_margin: u32,
    #[serde(default)]
    art_recipes: Vec<Recipe>,
    #[serde(default)]
    construction_recipes: Vec<Recipe>,
    #[serde(default)]
    cooking_recipes: Vec<Recipe>,
    #[serde(default)]
    craft_recipes: Vec<Recipe>,
    #[serde(default)]
    pharmacy_recipes: Vec<Recipe>,
    #[serde(default)]
    smith_recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize)]
pub struct Recipes {
    #[serde(default)]
    art_recipes: Vec<Recipe>,
    #[serde(default)]
    construction_recipes: Vec<Recipe>,
    #[serde(default)]
    cooking_recipes: Vec<Recipe>,
    #[serde(default)]
    craft_recipes: Vec<Recipe>,
    #[serde(default)]
    pharmacy_recipes: Vec<Recipe>,
    #[serde(default)]
    smith_recipes: Vec<Recipe>,
}

impl Creation {
    pub(crate) fn sort(&mut self) {
        self.cooking_recipes.sort();
    }

    pub fn join(&mut self, mut recipes: Recipes) {
        self.art_recipes.append(&mut recipes.art_recipes);
        self.construction_recipes
            .append(&mut recipes.construction_recipes);
        self.cooking_recipes.append(&mut recipes.cooking_recipes);
        self.craft_recipes.append(&mut recipes.craft_recipes);
        self.pharmacy_recipes.append(&mut recipes.pharmacy_recipes);
        self.smith_recipes.append(&mut recipes.smith_recipes);
    }

    pub fn recipes(&self, kind: CreationKind) -> &[Recipe] {
        match kind {
            CreationKind::Art => self.art_recipes.as_ref(),
            CreationKind::Construction => self.construction_recipes.as_ref(),
            CreationKind::Cooking => self.cooking_recipes.as_ref(),
            CreationKind::Craft => self.craft_recipes.as_ref(),
            CreationKind::Pharmacy => self.pharmacy_recipes.as_ref(),
            CreationKind::Smith => self.smith_recipes.as_ref(),
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
            if extension != "ron" {
                continue;
            }

            let recipes: Recipes = super::read_from_file(&path);
            self.join(recipes);
        }
        Ok(())
    }
}

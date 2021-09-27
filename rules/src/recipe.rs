use super::Rule;
use common::gamedata::{CreationKind, Recipe};

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

impl Rule for Recipes {
    const NAME: &'static str = "recipes";

    fn append(&mut self, mut other: Self) {
        self.art_recipes.append(&mut other.art_recipes);
        self.construction_recipes
            .append(&mut other.construction_recipes);
        self.cooking_recipes.append(&mut other.cooking_recipes);
        self.craft_recipes.append(&mut other.craft_recipes);
        self.pharmacy_recipes.append(&mut other.pharmacy_recipes);
        self.smith_recipes.append(&mut other.smith_recipes);

        self.art_recipes.sort();
        self.construction_recipes.sort();
        self.cooking_recipes.sort();
        self.craft_recipes.sort();
        self.pharmacy_recipes.sort();
        self.smith_recipes.sort();
    }
}

impl Recipes {
    pub fn get(&self, kind: CreationKind) -> &[Recipe] {
        match kind {
            CreationKind::Art => self.art_recipes.as_ref(),
            CreationKind::Construction => self.construction_recipes.as_ref(),
            CreationKind::Cooking => self.cooking_recipes.as_ref(),
            CreationKind::Craft => self.craft_recipes.as_ref(),
            CreationKind::Pharmacy => self.pharmacy_recipes.as_ref(),
            CreationKind::Smith => self.smith_recipes.as_ref(),
        }
    }
}

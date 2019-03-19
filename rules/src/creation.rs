use common::gamedata::CreationKind;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub product: String,
    pub ingredients: Vec<String>,
    pub difficulty: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Creation {
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

use crate::gamedata::CreationKind;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LearnedRecipes(Vec<Vec<String>>);

impl LearnedRecipes {
    pub fn new() -> LearnedRecipes {
        LearnedRecipes(vec![vec![]; CreationKind::Smith as usize + 1])
    }

    pub fn learned(&self, kind: CreationKind, recipe_name: &str) -> bool {
        let recipes = &self.0[kind as usize];
        recipes.iter().any(|r| r == recipe_name)
    }

    pub fn add(&mut self, kind: CreationKind, recipe_name: &str) {
        let recipes = &mut self.0[kind as usize];
        if recipes.iter().any(|s| s == recipe_name) {
            return;
        }
        recipes.push(recipe_name.to_owned());
    }
}

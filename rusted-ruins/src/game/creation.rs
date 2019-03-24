use super::extrait::*;
use super::InfoGetter;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;

pub fn item_auto_pick(gd: &GameData, recipe: &Recipe) -> Vec<Option<ItemLocation>> {
    let mut result = Vec::new();

    for ingredient in &recipe.ingredients {
        let idx: ItemIdx = gobj::id_to_idx(ingredient);
        let item_locations = gd.search_item(idx);
        if item_locations.is_empty() {
            result.push(None);
        } else {
            result.push(Some(item_locations[0]));
        }
    }
    result
}

pub fn start_creation(gd: &mut GameData, recipe: &Recipe, il: Vec<ItemLocation>) {
    let mut ingredients = Vec::new();

    for item_location in &il {
        ingredients.push(gd.remove_item_and_get(*item_location, 1));
    }

    let player = gd.chara.get_mut(CharaId::Player);
    player.add_status(CharaStatus::Creation {
        turn_left: 16,
        recipe: recipe.clone(),
        ingredients,
    });
}

pub fn finish_creation(gd: &mut GameData, recipe: &Recipe, ingredients: Vec<Item>) {

}

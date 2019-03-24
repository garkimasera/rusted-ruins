use super::InfoGetter;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use rules::creation::Recipe;

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

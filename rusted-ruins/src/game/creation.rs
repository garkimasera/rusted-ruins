use super::extrait::*;
use super::InfoGetter;
use crate::game::Game;
use crate::text::obj_txt;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use rules::RULES;

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

pub fn start_creation(game: &mut Game, recipe: &Recipe, il: Vec<ItemLocation>) {
    let gd = &mut game.gd;
    let mut ingredients = Vec::new();

    for item_location in &il {
        ingredients.push(gd.remove_item_and_get(*item_location, 1));
    }

    let player = gd.chara.get_mut(CharaId::Player);
    let work = Work::Creation {
        recipe: recipe.clone(),
        ingredients,
    };
    let needed_turn = RULES.creation.required_time[&recipe.required_time];
    player.add_status(CharaStatus::Work {
        turn_left: needed_turn,
        needed_turn,
        work,
    });

    let player = gd.chara.get(CharaId::Player);
    let product = obj_txt(&recipe.product);
    game.anim_queue.push_work(1.0);
    game_log_i!("creation-start"; chara=player, product=product);
}

pub fn finish_creation(gd: &mut GameData, recipe: &Recipe, _ingredients: Vec<Item>) {
    let idx: ItemIdx = gobj::id_to_idx(&recipe.product);
    let item_obj = gobj::get_obj(idx);
    let item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        quality: ItemQuality::default(),
        attributes: vec![],
    };

    let il = gd.get_item_list_mut(ItemListLocation::Chara {
        cid: CharaId::Player,
    });
    il.append(item, 1);

    let player = gd.chara.get(CharaId::Player);
    let product = obj_txt(&recipe.product);
    game_log_i!("creation-finish"; chara=player, product=product);
}

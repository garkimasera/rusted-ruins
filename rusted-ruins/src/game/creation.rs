use super::extrait::*;
use crate::game::{Game, InfoGetter};
use crate::text::obj_txt;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use rules::RULES;

pub fn start_creation(
    game: &mut Game,
    recipe: &Recipe,
    ill: ItemListLocation,
    prior_high_quality: bool,
) {
    let gd = &mut game.gd;
    let mut ingredients = Vec::new();

    let il = gd.get_item_list_mut(ill);

    for (ingredient, n) in &recipe.ingredients {
        let idx: ItemIdx = if let Some(idx) = gobj::id_to_idx_checked(ingredient) {
            idx
        } else {
            warn!("creation failed: unknown ingredient {}", ingredient);
            return;
        };
        il.consume(
            idx,
            *n,
            |item, n| ingredients.push((item.clone(), n)),
            prior_high_quality,
        );
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

pub fn finish_creation(gd: &mut GameData, recipe: &Recipe, _ingredients: Vec<(Item, u32)>) {
    let idx: ItemIdx = gobj::id_to_idx(&recipe.product);
    let item_obj = gobj::get_obj(idx);
    let item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        quality: ItemQuality::default(),
        attributes: vec![],
    };

    let ill = if recipe.put_on_ground {
        ItemListLocation::OnMap {
            mid: gd.get_current_mapid(),
            pos: gd.player_pos(),
        }
    } else {
        ItemListLocation::Chara {
            cid: CharaId::Player,
        }
    };
    let il = gd.get_item_list_mut(ill);
    il.append(item, 1);

    let player = gd.chara.get(CharaId::Player);
    let product = obj_txt(&recipe.product);
    game_log_i!("creation-finish"; chara=player, product=product);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LearnRecipeResult {
    Success,
    NotRecipeBook,
    NoAvailableRecipe,
}

/// Read item and learn new recipe. Return false if failed.
pub fn learn_recipe(gd: &mut GameData, il: ItemLocation) -> LearnRecipeResult {
    use rand::prelude::SliceRandom;

    let item = gd.get_item(il).0;
    let title = item.title().unwrap().to_owned();

    const RECIPE_KIND_PREFIX: &str = "recipe_kind:";

    if title.starts_with(RECIPE_KIND_PREFIX) {
        let creation_kind = match title.trim_start_matches(RECIPE_KIND_PREFIX) {
            "art" => CreationKind::Art,
            "construction" => CreationKind::Construction,
            "cooking" => CreationKind::Cooking,
            "craft" => CreationKind::Craft,
            "pharmacy" => CreationKind::Pharmacy,
            "smith" => CreationKind::Smith,
            _ => {
                return LearnRecipeResult::NotRecipeBook;
            }
        };
        let quality = item.quality.as_int() * RULES.creation.recipe_learning_item_factor
            + RULES.creation.recipe_learning_item_initial;
        let available_recipes: Vec<&str> = RULES
            .creation
            .recipes(creation_kind)
            .iter()
            .filter_map(|recipe| {
                if (recipe.difficulty as i32) < quality
                    && !gd.learned_recipes.learned(creation_kind, &recipe.product)
                {
                    Some(recipe.product.as_ref())
                } else {
                    None
                }
            })
            .collect();
        if let Some(new_recipe) = available_recipes.choose(&mut rng::GameRng) {
            gd.learned_recipes.add(creation_kind, new_recipe);
            gd.remove_item(il, 1);
            let item_name = obj_txt(new_recipe);
            let chara = gd.chara.get(CharaId::Player);
            game_log_i!("recipe-learned"; chara=chara, item=item_name);
            LearnRecipeResult::Success
        } else {
            let chara = gd.chara.get(CharaId::Player);
            game_log_i!("recipe-learning-failed"; chara=chara);
            LearnRecipeResult::NoAvailableRecipe
        }
    } else {
        LearnRecipeResult::NotRecipeBook
    }
}

pub fn available_recipes(gd: &GameData, kind: CreationKind) -> Vec<&'static Recipe> {
    RULES
        .creation
        .recipes(kind)
        .iter()
        .filter(|recipe| gd.learned_recipes.learned(kind, &recipe.product))
        .collect()
}

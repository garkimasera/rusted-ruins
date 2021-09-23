use super::extrait::*;
use crate::game::Game;
use crate::text::obj_txt;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use rules::RULES;

pub fn start_creation(
    game: &mut Game<'_>,
    kind: CreationKind,
    recipe: &Recipe,
    ill: ItemListLocation,
    prior_high_quality: bool,
    material_to_use: Option<ItemIdx>,
) {
    let gd = &mut game.gd;
    let mut ingredients = Vec::new();
    let mut material = None;

    let il = gd.get_item_list_mut(ill);

    for (ingredient, n) in &recipe.ingredients {
        let idx = if material_group(ingredient).is_some() {
            let idx = material_to_use.expect("empty material_to_use");
            material = Some(gobj::get_obj(idx).material);
            idx
        } else if let Some(idx) = gobj::id_to_idx_checked(ingredient) {
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
        kind,
        recipe: recipe.clone(),
        ingredients,
        material,
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
    game_log!("creation-start"; chara=player, product=product);
}

pub fn finish_creation(
    gd: &mut GameData,
    kind: CreationKind,
    recipe: &Recipe,
    _ingredients: Vec<(Item, u32)>,
    material: Option<MaterialName>,
) {
    let idx: ItemIdx = gobj::id_to_idx(&recipe.product);
    let item_obj = gobj::get_obj(idx);
    let mut item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        quality: ItemQuality::default(),
        attrs: vec![],
        time: None,
    };
    if let Some(material) = material {
        item.attrs.push(ItemAttr::Material(material));
    }

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

    let product = obj_txt(&recipe.product);
    let player = gd.chara.get_mut(CharaId::Player);

    // Exp
    let skill_kind = kind.into();
    let skill_level = player.skill_level(skill_kind);
    if skill_level > 0 {
        let exp = RULES.exp.creation_base_exp;
        player.add_skill_exp(skill_kind, exp, recipe.difficulty);
    }

    game_log!("creation-finish"; chara=player, product=product);
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
        let skill_level = gd
            .chara
            .get(CharaId::Player)
            .skill_level_with_adj(creation_kind.into())
            .0;
        let max_recipe_level = skill_level + RULES.creation.recipe_learning_level_margin;
        let available_recipes: Vec<&str> = RULES
            .creation
            .recipes(creation_kind)
            .iter()
            .filter_map(|recipe| {
                if recipe.difficulty < max_recipe_level
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
            game_log!("recipe-learned"; chara=chara, item=item_name);
            LearnRecipeResult::Success
        } else {
            let chara = gd.chara.get(CharaId::Player);
            game_log!("recipe-learning-failed"; chara=chara);
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

/// Determine a character has enough skill for given creation or not.
pub fn enough_skill(chara: &Chara, recipe: &Recipe, kind: CreationKind) -> bool {
    let skill_level = chara.skill_level(kind.into());

    skill_level >= recipe.difficulty
}

/// Returns available material items for given recipe
pub fn available_material(
    gd: &GameData,
    recipe: &Recipe,
    ill: ItemListLocation,
) -> Vec<(ItemIdx, u32)> {
    let group = if let Some(group) = recipe
        .ingredients
        .iter()
        .find_map(|ingredient| material_group(&ingredient.0))
    {
        group
    } else {
        return vec![];
    };

    let il = gd.get_item_list(ill);
    let mut items = Vec::new();

    for (item, n) in il.iter() {
        if item.obj().group == group {
            if let Some(a) = items.iter_mut().find(|(idx, _)| *idx == item.idx) {
                a.1 += n;
            } else {
                items.push((item.idx, *n))
            }
        } else {
            continue;
        }
    }

    items
}

/// Recipes that difficulty is zero are available from game start.
pub fn add_initial_recipes(gd: &mut GameData) {
    for creation_kind in CreationKind::ALL {
        for recipe in RULES.creation.recipes(*creation_kind) {
            if recipe.difficulty == 0 {
                gd.learned_recipes.add(*creation_kind, &recipe.product);
            }
        }
    }
}

pub fn material_group(ingredient: &str) -> Option<&str> {
    ingredient.strip_prefix("group/")
}

use crate::game::play_time::UniqueIdGeneratorByTime;
use common::gamedata::*;
use common::gobj;
use common::item_selector::ItemSelector;
use common::obj::ImgVariationRule;
use common::objholder::ItemIdx;
use rng::SliceRandom;
use rules::RULES;

/// Generate new item on dungeon floor
pub fn gen_dungeon_item(floor_level: u32, dungeon_kind: DungeonKind) -> Option<Item> {
    let gen_rule = RULES.dungeon_gen.get(&dungeon_kind)?;
    let (_, (item_selector, _)) = rng::choose(&gen_rule.item_gen_weight, |(_, weight)| *weight)?;
    let item_selector = item_selector.clone().level(floor_level);
    let item_idx = choose_item_by_item_selector(&item_selector)?;
    Some(gen_item_from_idx(item_idx, floor_level))
}

/// Generate new item by level.
/// f is weight adjustment function.
pub fn gen_item_by_level<F: FnMut(&ItemObject) -> f32>(
    level: u32,
    f: F,
    is_shop: bool,
) -> Option<Item> {
    choose_item_by_floor_level(level, f, is_shop).map(|idx| gen_item_from_idx(idx, level))
}

/// Choose item by floor level.
/// f is weight adjustment function.
fn choose_item_by_floor_level<F: FnMut(&ItemObject) -> f32>(
    floor_level: u32,
    mut f: F,
    is_shop: bool,
) -> Option<ItemIdx> {
    let items = &gobj::get_objholder().item;

    // Sum up gen_weight * weight_dist * dungeon_adjustment
    let weight_dist = CalcLevelWeightDist::new(floor_level);
    let mut sum = 0.0;
    let mut first_available_item_idx = None;

    for (i, item) in items.iter().enumerate() {
        let gen_weight = if is_shop {
            item.shop_weight
        } else {
            item.gen_weight
        };
        sum += weight_dist.calc(item.gen_level) * gen_weight as f32 * f(item);
        if first_available_item_idx.is_none() {
            first_available_item_idx = Some(i);
        }
    }

    if sum == 0.0 {
        return None;
    }

    // Choose one item
    let r = rng::gen_range(0.0..sum);
    let mut sum = 0.0;
    for (i, item) in items.iter().enumerate() {
        let gen_weight = if is_shop {
            item.shop_weight
        } else {
            item.gen_weight
        };
        sum += weight_dist.calc(item.gen_level) * gen_weight as f32 * f(item);
        if r < sum {
            return Some(ItemIdx::from_usize(i));
        }
    }

    Some(ItemIdx::from_usize(first_available_item_idx.unwrap()))
}

pub fn choose_item_by_item_selector(item_selector: &ItemSelector) -> Option<ItemIdx> {
    let items = &gobj::get_objholder().item;
    let items = item_selector.select_items_from(items);

    items.choose(&mut rng::get_rng()).map(|(idx, _)| *idx)
}

struct CalcLevelWeightDist {
    floor_level: f32,
}

impl CalcLevelWeightDist {
    fn new(floor_level: u32) -> CalcLevelWeightDist {
        CalcLevelWeightDist {
            floor_level: floor_level as f32,
        }
    }

    fn calc(&self, l: u32) -> f32 {
        let l = l as f32;
        if l > self.floor_level {
            0.0
        } else {
            1.0
        }
    }
}

/// Generate an item from ItemGen.
pub fn from_item_gen(item_gen: &ItemGen) -> Option<Item> {
    gobj::id_to_idx_checked::<ItemIdx>(&item_gen.id).map(|idx| gen_item_from_idx(idx, 1))
}

pub fn gen_item_from_id(id: &str, level: u32) -> Item {
    gen_item_from_idx(gobj::id_to_idx(id), level)
}

pub fn gen_item_from_idx(idx: ItemIdx, level: u32) -> Item {
    let item_obj = gobj::get_obj(idx);

    let mut item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        quality: ItemQuality::default(),
        attrs: vec![],
        time: None,
    };

    // Set image variation.
    if item_obj.img.variation_rule == ImgVariationRule::RandomOnGen {
        item.attrs.push(ItemAttr::ImageVariation(rng::gen_range(
            0..item_obj.img.n_pattern,
        )));
    }

    if let Some(&growing_duration) =
        find_attr!(item_obj, ItemObjAttr::Plant { growing_duration, .. } => growing_duration)
    {
        gen_plant_item(&mut item, growing_duration);
    }

    if has_attr!(item_obj, ItemObjAttr::Container) {
        gen_container_item(&mut item);
    }

    if let Some(&duration) = find_attr!(item_obj, ItemObjAttr::Rot(duration)) {
        item.time = Some(ItemTime {
            last_updated: crate::game::time::current_time(),
            remaining: duration,
        });
    }

    if has_attr!(item_obj, ItemObjAttr::Titles) {
        gen_readable_item(&mut item, item_obj)
    }

    if item_obj.kind == ItemKind::MagicDevice {
        gen_magic_device(&mut item, item_obj)
    };

    if let Some(ItemObjAttr::Use(UseEffect::Effect(effect))) =
        find_attr!(item_obj, ItemObjAttr::Use)
    {
        for kind in &effect.kind {
            if let EffectKind::SkillLearning { .. } = kind {
                gen_skill_lerning_item(&mut item, item_obj)
            }
        }
    }

    if has_attr!(item_obj, ItemObjAttr::Tool) {
        gen_tool_item(&mut item, item_obj);
    }

    if has_attr!(item_obj, ItemObjAttr::Module) {
        gen_module_item(&mut item, item_obj);
    }

    set_quality(&mut item, item_obj, level);
    set_material(&mut item, item_obj, level);

    item
}

fn gen_plant_item(item: &mut Item, growing_duration: Duration) {
    let last_updated = crate::game::time::current_time();
    item.flags |= ItemFlags::PLANT;
    if growing_duration.as_secs() > 0 {
        item.time = Some(ItemTime {
            last_updated,
            remaining: growing_duration,
        });
    }
}

fn gen_container_item(item: &mut Item) {
    item.attrs.push(ItemAttr::Container(ItemListContainer::new(
        UniqueIdGeneratorByTime,
    )));
}

/// Generate a magic device item
fn gen_magic_device(item: &mut Item, item_obj: &ItemObject) {
    let n = if let Some(&ItemObjAttr::Charge { min, max }) =
        find_attr!(item_obj, ItemObjAttr::Charge)
    {
        rng::gen_range(min..=max)
    } else {
        return;
    };
    item.attrs.push(ItemAttr::Charge { n: n.into() });
}

/// Generate a readable item
fn gen_readable_item(item: &mut Item, item_obj: &ItemObject) {
    let titles = item_obj
        .attrs
        .iter()
        .filter_map(|attr| match attr {
            ItemObjAttr::Titles(titles) => Some(titles),
            _ => None,
        })
        .next()
        .unwrap();
    let title = titles.choose(&mut rng::GameRng).cloned().unwrap();
    item.attrs.push(ItemAttr::Title(title));
}

/// Generate a skill learning item
fn gen_skill_lerning_item(item: &mut Item, _item_obj: &ItemObject) {
    let skill_kind: SkillKind = if rng::gen_range(0..3) == 0 {
        CreationKind::ALL
            .choose(&mut rng::GameRng)
            .copied()
            .unwrap()
            .into()
    } else {
        WeaponKind::ALL
            .choose(&mut rng::GameRng)
            .copied()
            .unwrap()
            .into()
    };
    item.attrs.push(ItemAttr::SkillLearning(skill_kind));
}

/// Generate tool item
fn gen_tool_item(item: &mut Item, item_obj: &ItemObject) {
    let tool_effect = find_attr!(item_obj, ItemObjAttr::Tool(tool_effect) => tool_effect).unwrap();

    if tool_effect == &ToolEffect::Build {
        item.attrs
            .push(ItemAttr::BuildObj(RULES.item.build_obj_default.clone()));
    }
}

/// Generate module item
fn gen_module_item(item: &mut Item, item_obj: &ItemObject) {
    let effects = find_attr!(item_obj, ItemObjAttr::Module { effects } => effects).unwrap();
    if let Some((_, effect)) = rng::choose(effects, |(_, weight)| (*weight).into()) {
        item.attrs.push(ItemAttr::Module(effect.0.clone()));
    }
}

fn set_quality(item: &mut Item, item_obj: &ItemObject, level: u32) {
    match item_obj.quality_kind {
        QualityKind::None => {}
        QualityKind::Mutable => {
            let level_diff = if level > item_obj.gen_level {
                level - item_obj.gen_level
            } else {
                0
            };
            item.quality.base =
                rng::gen_range(0..=(level_diff / RULES.item.quality_level_factor)) as i32;
        }
    }
}

fn set_material(item: &mut Item, item_obj: &ItemObject, level: u32) {
    if item_obj.material_group.is_empty() {
        return;
    }
    let rule = &RULES.materials;
    let materials = rule.get_by_group(&item_obj.material_group, Some(level));
    let material_names: Vec<MaterialName> = materials.iter().map(|(name, _)| *name).collect();
    let chosen_materials =
        rng::choose(&material_names, |name| RULES.materials.get(name).gen_weight);
    let material_name = if let Some(chosen) = chosen_materials {
        *chosen.1
    } else {
        return;
    };
    item.attrs.push(ItemAttr::Material(material_name));
}

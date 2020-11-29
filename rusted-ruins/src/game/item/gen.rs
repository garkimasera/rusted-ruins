use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use rng::SliceRandom;
use rules::RULES;

/// Generate new item on dungeon floor
pub fn gen_dungeon_item(floor_level: u32, dungeon_kind: DungeonKind) -> Option<Item> {
    let weight = move |item_obj: &ItemObject| {
        let gen_rule = if let Some(gen_rule) = RULES.dungeon_gen.get(&dungeon_kind) {
            gen_rule
        } else {
            return 0.0;
        };
        gen_rule
            .item_gen_weight_for_kind
            .get(&item_obj.kind.rough())
            .copied()
            .unwrap_or(0.0)
    };
    gen_item_by_level(floor_level, weight, false)
}

/// Generate new item by level.
/// f is weight adjustment function.
pub fn gen_item_by_level<F: FnMut(&ItemObject) -> f32>(
    level: u32,
    f: F,
    is_shop: bool,
) -> Option<Item> {
    choose_item_by_floor_level(level, f, is_shop).map(|idx| gen_item_from_idx(idx))
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
    let r = rng::gen_range(0.0, sum);
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
    if let Some(idx) = gobj::id_to_idx_checked::<ItemIdx>(&item_gen.id) {
        Some(gen_item_from_idx(idx))
    } else {
        None
    }
}

pub fn gen_item_from_id(id: &str) -> Item {
    gen_item_from_idx(gobj::id_to_idx(id))
}

pub fn gen_item_from_idx(idx: ItemIdx) -> Item {
    let item_obj = gobj::get_obj(idx);

    let mut item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        quality: ItemQuality::default(),
        attributes: vec![],
    };

    if !item_obj.titles.is_empty() {
        gen_readable_item(&mut item, item_obj)
    }

    match item_obj.kind {
        ItemKind::MagicDevice => gen_magic_device(&mut item, item_obj),
        _ => (),
    };

    if let Some(use_effect) = item_obj.use_effect.as_ref() {
        for kind in &use_effect.kind {
            match kind {
                EffectKind::SkillLearning { .. } => gen_skill_lerning_item(&mut item, item_obj),
                _ => (),
            }
        }
    }

    item
}

/// Generate a magic device item
fn gen_magic_device(item: &mut Item, item_obj: &ItemObject) {
    let charge_n: u32 = rng::gen_range_inclusive(item_obj.charge[0], item_obj.charge[1]).into();
    item.attributes.push(ItemAttribute::Charge { n: charge_n });
}

/// Generate a readable item
fn gen_readable_item(item: &mut Item, item_obj: &ItemObject) {
    let title = item_obj.titles.choose(&mut rng::GameRng).cloned().unwrap();
    item.attributes.push(ItemAttribute::Title(title));
}

/// Generate a skill learning item
fn gen_skill_lerning_item(item: &mut Item, _item_obj: &ItemObject) {
    let skill_kind = if rng::gen_range(0, 3) == 0 {
        SkillKind::Creation(
            CreationKind::ALL
                .choose(&mut rng::GameRng)
                .copied()
                .unwrap(),
        )
    } else {
        SkillKind::Weapon(WeaponKind::ALL.choose(&mut rng::GameRng).copied().unwrap())
    };
    item.attributes
        .push(ItemAttribute::SkillLearning(skill_kind));
}

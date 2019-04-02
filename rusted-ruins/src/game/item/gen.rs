use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use rng::gen_range;

/// Generate new item on dungeon floor
pub fn gen_dungeon_item(floor_level: u32) -> Item {
    gen_item_by_level(floor_level, |_| 1.0, false)
}

/// Generate new item by level.
/// f is weight adjustment function.
pub fn gen_item_by_level<F: FnMut(&ItemObject) -> f64>(level: u32, f: F, is_shop: bool) -> Item {
    let idx = choose_item_by_floor_level(level, f, is_shop);

    gen_item_from_idx(idx)
}

/// Choose item by floor level.
/// f is weight adjustment function.
fn choose_item_by_floor_level<F: FnMut(&ItemObject) -> f64>(
    floor_level: u32,
    mut f: F,
    is_shop: bool,
) -> ItemIdx {
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
        sum += weight_dist.calc(item.gen_level) * gen_weight as f64 * f(item);
        if first_available_item_idx.is_none() {
            first_available_item_idx = Some(i);
        }
    }

    assert!(sum > 0.0);

    // Choose one item
    let r = rng::gen_range(0.0, sum);
    let mut sum = 0.0;
    for (i, item) in items.iter().enumerate() {
        let gen_weight = if is_shop {
            item.shop_weight
        } else {
            item.gen_weight
        };
        sum += weight_dist.calc(item.gen_level) * gen_weight as f64 * f(item);
        if r < sum {
            return ItemIdx::from_usize(i);
        }
    }

    ItemIdx::from_usize(first_available_item_idx.unwrap())
}

struct CalcLevelWeightDist {
    floor_level: f64,
}

impl CalcLevelWeightDist {
    fn new(floor_level: u32) -> CalcLevelWeightDist {
        CalcLevelWeightDist {
            floor_level: floor_level as f64,
        }
    }

    fn calc(&self, l: u32) -> f64 {
        let l = l as f64;
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

fn gen_item_from_idx(idx: ItemIdx) -> Item {
    let item_obj = gobj::get_obj(idx);

    let item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        rank: ItemRank::default(),
        attributes: vec![],
    };

    match item_obj.kind {
        ItemKind::MagicDevice => gen_magic_device(item, item_obj),
        _ => item,
    }
}

/// Generate a magic device item
fn gen_magic_device(mut item: Item, item_obj: &ItemObject) -> Item {
    let charge_n: u32 = gen_range(item_obj.charge[0], item_obj.charge[1]).into();
    item.attributes.push(ItemAttribute::Charge { n: charge_n });
    item
}

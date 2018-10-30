
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;

/// Generate new item on dungeon floor
pub fn gen_dungeon_item(floor_level: u32) -> Item {
    gen_item_by_level(floor_level, |_| 1.0)
}

/// Generate new item by level.
/// f is weight adjustment function.
pub fn gen_item_by_level<F: FnMut(&ItemObject) -> f64>(level: u32, f: F) -> Item {
    let idx = choose_item_by_floor_level(level, f);

    let item_obj = gobj::get_obj(idx);
    Item {
        idx: idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        rank: ItemRank::default(),
        attributes: vec![],
    }
}

/// Choose item by floor level.
/// f is weight adjustment function.
fn choose_item_by_floor_level<F: FnMut(&ItemObject) -> f64>(floor_level: u32, mut f: F) -> ItemIdx {
    let items = &gobj::get_objholder().item;

    // Sum up gen_weight * weight_dist * dungeon_adjustment
    let weight_dist = CalcLevelWeightDist::new(floor_level);
    let mut sum = 0.0;
    let mut first_available_item_idx = None;
    
    for (i, item) in items.iter().enumerate() {
        sum += weight_dist.calc(item.gen_level) * item.gen_weight as f64 * f(item);
        if first_available_item_idx.is_none() {
            first_available_item_idx = Some(i);
        }
    }

    assert!(sum > 0.0);

    // Choose one chara
    let r = ::rng::gen_range(0.0, sum);
    let mut sum = 0.0;
    for (i, item) in items.iter().enumerate() {
        sum += weight_dist.calc(item.gen_level) * item.gen_weight as f64 * f(item);
        if r < sum {
            return ItemIdx(i as u32);
        }
    }

    ItemIdx(first_available_item_idx.unwrap() as u32)
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

/// Generate item from ItemGen.
pub fn from_item_gen(item_gen: &ItemGen) -> Option<Item> {
    if let Some(idx) = gobj::id_to_idx_checked::<ItemIdx>(&item_gen.id) {
        let item_obj = gobj::get_obj(idx);

        Some(Item {
            idx: idx,
            flags: item_obj.default_flags,
            kind: item_obj.kind,
            rank: ItemRank::default(),
            attributes: vec![],
        })
    } else {
        None
    }
}


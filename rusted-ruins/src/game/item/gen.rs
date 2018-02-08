
use common::gamedata::item::*;
use common::gobj;
use common::objholder::ItemIdx;

/// Generate new item on dungeon floor
pub fn gen_dungeon_item(floor_level: u32) -> Box<Item> {
    let idx = choose_item_by_floor_level(floor_level);

    let item_obj = gobj::get_obj(idx);
    let item = Item {
        idx: idx,
        kind: item_obj.content.kind_detail(),
        quality: ItemQuality::default(),
    };
    Box::new(item)
}

fn choose_item_by_floor_level(floor_level: u32) -> ItemIdx {
    let items = &gobj::get_objholder().item;

    // Sum up gen_weight * weight_dist * dungeon_adjustment
    let weight_dist = CalcLevelWeightDist::new(floor_level);
    let mut sum = 0.0;
    let mut first_available_item_idx = None;
    
    for (i, item) in items.iter().enumerate() {
        sum += weight_dist.calc(item.gen_level) * item.gen_weight as f64;
        if first_available_item_idx.is_none() {
            first_available_item_idx = Some(i);
        }
    }

    assert!(sum > 0.0);

    // Choose one chara
    let r = ::rng::gen_range(0.0, sum);
    let mut sum = 0.0;
    for (i, item) in items.iter().enumerate() {
        sum += weight_dist.calc(item.gen_level) * item.gen_weight as f64;
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
        if l < self.floor_level {
            0.0
        } else {
            1.0
        }
    }
}


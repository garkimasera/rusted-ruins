use super::{
    filter::{FilteredListHolder, ItemFilter},
    GameDataItemExt, ItemExt,
};
use common::gamedata::*;
use rules::RULES;

/// Returns the number of given item's (filled, empty, installable) slots
pub fn n_slot(item: &Item, slot_kind: ModuleSlotKind) -> (u32, u32, u32) {
    let mut filled = 0;
    let mut empty = 0;

    for attr in &item.attrs {
        if let ItemAttr::ModuleSlot { kind, content } = attr {
            if slot_kind == *kind {
                if content.is_some() {
                    filled += 1;
                } else {
                    empty += 1;
                }
            }
        }
    }

    let max_slot = if let Some(esk) = item.kind.equip_slot_kind() {
        match slot_kind {
            ModuleSlotKind::Ability => {
                if let Some(a) = RULES.item.ability_slot_required_quality.get(&esk) {
                    if *a >= item.quality.base {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            ModuleSlotKind::Core => {
                if let Some(a) = RULES.item.core_slot_required_quality.get(&esk) {
                    if *a >= item.quality.base {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            ModuleSlotKind::Extend => {
                if let Some(v) = RULES.item.extend_slot_required_quality.get(&esk) {
                    v.iter().filter(|a| **a <= item.quality.base).count() as u32
                } else {
                    0
                }
            }
        }
    } else {
        0
    };

    let installable = if max_slot > filled + empty {
        max_slot - (filled + empty)
    } else {
        0
    };

    (filled, empty, installable)
}

fn slot_install_cost(item: &Item, slot_kind: ModuleSlotKind) -> Option<i64> {
    let (filled, empty, installable) = n_slot(item, slot_kind);

    if installable > 0 {
        let additional_cost = item.price() as f32
            * RULES.item.slot_install_cost_factor[&slot_kind]
            * (filled + empty + 1) as f32;
        let cost = RULES.item.slot_install_cost_base[&slot_kind] + additional_cost as i64;
        Some(cost)
    } else {
        None
    }
}

pub fn slot_installable_item_list(
    gd: &GameData,
    slot_kind: ModuleSlotKind,
) -> Vec<(ItemLocation, i64)> {
    gd.get_filtered_item_list(ItemListLocation::PLAYER, ItemFilter::all())
        .filter_map(|(il, item, _)| slot_install_cost(item, slot_kind).map(|cost| (il, cost)))
        .collect()
}

pub fn install_slot(gd: &mut GameData, il: ItemLocation, slot_kind: ModuleSlotKind, cost: i64) {
    if gd.player.money() < cost {
        return;
    }
    let mut item = gd.remove_item_and_get(il, 1);
    item.attrs.push(ItemAttr::ModuleSlot {
        kind: slot_kind,
        content: None,
    });
    item.attrs.sort();
    gd.append_item_to(ItemListLocation::PLAYER, item, 1);
    gd.player.sub_money(cost);
}

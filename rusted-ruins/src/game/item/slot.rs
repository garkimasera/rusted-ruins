use super::{
    filter::{FilteredListHolder, ItemFilter},
    GameDataItemExt, ItemExt,
};
use common::{gamedata::*, objholder::ItemIdx};
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
        game_log!("install-slot-lack-of-money"; chara=gd.chara.get(CharaId::Player));
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

pub fn insert_module_to(item: &mut Item, module_item: Item, i_slot: u32) -> Option<Item> {
    let module_item_idx = module_item.idx;
    let effect = if let Some(effect) = find_attr!(module_item, ItemAttr::Module(effect) => effect) {
        effect
    } else {
        error!("insert module failed");
        return None;
    };

    if let Some(ItemAttr::ModuleSlot { content, .. }) = item
        .attrs
        .iter_mut()
        .filter(|attr| matches!(attr, ItemAttr::ModuleSlot { .. }))
        .nth(i_slot as usize)
    {
        std::mem::replace(content, Some((module_item_idx, effect.clone())))
            .map(|(module_item_idx, effect)| module_item_from(module_item_idx, effect))
    } else {
        error!("insert module failed");
        None
    }
}

fn module_item_from(item_idx: ItemIdx, effect: ModuleEffect) -> Item {
    let mut item = super::gen::gen_item_from_idx(item_idx, 0);
    if let Some(ItemAttr::Module(e)) = find_attr_mut!(item, ItemAttr::Module) {
        *e = effect;
    }
    item
}

pub fn slot_list(item: &Item) -> Vec<(ModuleSlotKind, String)> {
    item.attrs
        .iter()
        .filter_map(|attr| {
            if let ItemAttr::ModuleSlot { kind, content } = attr {
                Some((*kind, super::info::slot_text(content)))
            } else {
                None
            }
        })
        .collect()
}

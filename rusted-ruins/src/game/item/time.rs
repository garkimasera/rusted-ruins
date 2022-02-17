use super::gen::gen_item_from_id;
use crate::game::extrait::*;
use common::gamedata::*;
use common::gobj;
use rules::RULES;

/// Update item time stamp for current map.
#[allow(clippy::needless_collect)]
pub fn update_item_time(gd: &mut GameData) {
    trace!("update item time");

    let cids: Vec<CharaId> = gd.get_current_map().iter_charaid().copied().collect();

    // For characters
    for &cid in &cids {
        let ill = ItemListLocation::Chara { cid };

        update_item_list_time(gd, ill);
    }

    // For tiles
    let mid = gd.get_current_mapid();
    for pos in gd.get_current_map().tile.iter_idx() {
        let ill = ItemListLocation::OnMap { mid, pos };

        update_item_list_time(gd, ill);
    }

    // For shops
    if !mid.is_region_map() {
        let sid = mid.sid();
        let shop_cids: Vec<CharaId> = gd.region.get_site(sid).iter_shop_ids().collect();

        for cid in shop_cids.into_iter() {
            let ill = ItemListLocation::Shop { cid };
            update_item_list_time(gd, ill);
        }
    }
}

#[derive(Debug, Default)]
pub struct UpdateItemRule {
    prevent_rot: bool,
}

fn update_item_list_time(gd: &mut GameData, ill: ItemListLocation) {
    let mut items_to_add: Vec<(Item, u32)> = Vec::new();
    let mut items_to_remove: Vec<(u32, u32)> = Vec::new();

    let mut rule = UpdateItemRule::default();

    if let ItemListLocation::Shop { .. } = ill {
        rule.prevent_rot = true;
    }

    if let ItemListLocation::Container { ill, i } = ill {
        let container_item = &gd.get_item((ill.into(), i)).0;
        let item_obj = container_item.obj();

        rule.prevent_rot =
            *find_attr!(item_obj, ItemObjAttr::Container { prevent_rot, .. } => prevent_rot)
                .expect("invalid item for container");
    }

    let item_list_len = gd.get_item_list(ill).len() as u32;

    for i in 0..item_list_len {
        match process_item(gd, (ill, i), &rule) {
            UpdateTimeResult::None => (),
            UpdateTimeResult::Transform(item, n_gen, n_remove) => {
                items_to_add.push((item, n_gen));
                items_to_remove.push((i as u32, n_remove));
            }
        }
    }

    for &(i, n) in &items_to_remove {
        gd.remove_item((ill, i), n);
    }

    let item_list = gd.get_item_list_mut(ill);
    for (item, n) in items_to_add.into_iter() {
        item_list.append(item, n);
    }
}

enum UpdateTimeResult {
    None,
    Transform(Item, u32, u32),
}

fn process_item(gd: &mut GameData, il: ItemLocation, rule: &UpdateItemRule) -> UpdateTimeResult {
    if has_attr!(gd.get_item(il).0, ItemAttr::Container) {
        let ill = ItemListLocation::in_container(il);
        update_item_list_time(gd, ill)
    }

    let (item, n) = gd.get_item_mut(il);
    let item_obj = gobj::get_obj(item.idx);

    if item.time.is_none() {
        return UpdateTimeResult::None;
    }

    if has_attr!(item_obj, ItemObjAttr::Rot) {
        if item.update_time(!rule.prevent_rot) {
            let total_weight = item_obj.w * n;
            let n_rot_pile = std::cmp::max(total_weight / RULES.item.rotten_item_gen_per_gram, 1);

            let rotten_item = gen_item_from_id(&RULES.item.rotten_item, 1);

            if (ItemListLocation::Chara {
                cid: CharaId::Player,
            }) == il.0
            {
                game_log!("inventory-item-rotten"; item=item, n=n);
            }

            return UpdateTimeResult::Transform(rotten_item, n_rot_pile, n);
        } else {
            return UpdateTimeResult::None;
        }
    }

    if let Some(ContainerFunction::ConvertMixed {
        product,
        product_multiplier,
        ingredients,
        ..
    }) = find_attr!(item_obj, ItemObjAttr::Container { function, .. } => function)
    {
        if item.update_time(true) {
            let container_item_list = if let Some(ItemAttr::Container(container)) =
                find_attr_mut!(item, ItemAttr::Container)
            {
                &mut container.item_list
            } else {
                return UpdateTimeResult::None;
            };
            super::convert_container::do_mixed_convert(
                container_item_list,
                product,
                *product_multiplier,
                ingredients,
            );
            item.time = None;
        }
        return UpdateTimeResult::None;
    }

    item.update_time(true);
    UpdateTimeResult::None
}

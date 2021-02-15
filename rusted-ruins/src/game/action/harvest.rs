use crate::game::extrait::*;
use crate::game::Game;
use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use geom::*;

pub fn harvest_item(gd: &mut GameData, il: ItemLocation) -> bool {
    let item = gd.get_item(il).0;
    let item_idx = item.idx;
    let item_obj = gobj::get_obj(item_idx);

    let harvest = item_obj
        .harvest
        .as_ref()
        .expect("Tried to harvest item that is not harvestable");

    if item.compare_time() != Some(true) {
        game_log_i!("harvest-plant-not-ready"; item=item);
        return false;
    }

    let target_item_idx: ItemIdx = gobj::id_to_idx(&harvest.item);
    let target_item = crate::game::item::gen::gen_item_from_idx(target_item_idx, 0);
    let n_yield = harvest.n_yield;

    game_log_i!("harvest-plant"; chara=gd.chara.get(CharaId::Player), item=&target_item, n=n_yield);
    let item_list = gd.get_item_list_mut(ItemListLocation::PLAYER);
    item_list.append(target_item, n_yield);
    true
}

pub fn harvest_by_tool(game: &mut Game, chara_id: CharaId, pos: Vec2d) {
    let gd = &mut game.gd;
    let player_pos = gd.player_pos();

    if !pos.is_adjacent(player_pos) && player_pos != pos {
        game_log_i!("chopping-not-adjacent-tile");
        return;
    }

    let chara = gd.chara.get(chara_id);
    let tool = chara
        .equip
        .item(EquipSlotKind::Tool, 0)
        .expect("tried to non-existent tool");
    let tool_obj = tool.obj();

    let list = gd.search_harvestable_item(pos);
    for (il, item_idx) in &list {
        let item_obj = gobj::get_obj(*item_idx);
        let harvest = if let Some(harvest) = item_obj.harvest.as_ref() {
            harvest
        } else {
            continue;
        };
        let needed_turn = 10;

        match harvest.kind {
            HarvestKind::Chop => {
                if tool_obj.tool_effect == Some(ToolEffect::Chop) {
                    let work = Work::Harvest {
                        item_idx: *item_idx,
                        il: *il,
                    };
                    let chara = gd.chara.get_mut(chara_id);
                    chara.add_status(CharaStatus::Work {
                        turn_left: needed_turn,
                        needed_turn,
                        work,
                    });
                    game.anim_queue.push_work(1.0);
                    return;
                }
            }
            _ => (),
        }
    }

    match tool_obj.tool_effect {
        Some(ToolEffect::Chop) => {
            game_log_i!("chopping-no-tree");
        }
        _ => (),
    }
}

pub fn finish_harvest(gd: &mut GameData, chara_id: CharaId, item_idx: ItemIdx, il: ItemLocation) {
    let item = gd.get_item(il);
    if item.0.idx != item_idx {
        return;
    }

    let item_obj = gobj::get_obj(item_idx);
    if item_obj.harvest.is_none() {
        return;
    }
    let harvest = item_obj.harvest.as_ref().unwrap();

    let target_item_idx: ItemIdx = gobj::id_to_idx(&harvest.item);
    let target_item = crate::game::item::gen::gen_item_from_idx(target_item_idx, 1);
    let n_yield = harvest.n_yield;

    match harvest.kind {
        HarvestKind::Chop => {
            game_log_i!("harvest-chop"; chara=gd.chara.get(chara_id), item=&target_item, n=n_yield);
            audio::play_sound("chop-tree");
        }
        _ => (),
    }
    gd.add_item_on_tile(gd.player_pos(), target_item.clone(), n_yield);
    gd.remove_item(il, 1);
    return;
}

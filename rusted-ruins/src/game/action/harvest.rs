use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;
use geom::*;

// pub fn harvest_item(gd: &mut GameData, il: ItemLocation) {
//     let item = gd.remove_item_and_get(il, 1);
//     let item_idx = item.idx;
//     let item_obj = gobj::get_obj(item_idx);

//     let harvest = item_obj
//         .harvest
//         .as_ref()
//         .expect("Tried to harvest item that is not harvestable");

//     let target_item_idx: ItemIdx = gobj::id_to_idx(&harvest.target_item);
//     let target_item = crate::game::item::gen::gen_item_from_idx(target_item_idx);
//     let n_yield = harvest.n_yield;

//     game_log_i!("harvest-chop"; chara=gd.chara.get(CharaId::Player), item=&target_item, n=n_yield);
//     audio::play_sound("chop-tree");
//     gd.add_item_on_tile(gd.player_pos(), target_item.clone(), n_yield);
// }

pub fn harvest_by_tool(gd: &mut GameData, chara_id: CharaId, pos: Vec2d) {
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
        let target_item_idx: ItemIdx = gobj::id_to_idx(&harvest.target_item);
        let target_item = crate::game::item::gen::gen_item_from_idx(target_item_idx);
        let n_yield = harvest.n_yield;

        match harvest.harvest_type {
            HarvestType::Chop => {
                if tool_obj.tool_effect == ToolEffect::Chop {
                    game_log_i!("harvest-chop"; chara=gd.chara.get(chara_id), item=&target_item, n=n_yield);
                    audio::play_sound("chop-tree");
                    gd.add_item_on_tile(gd.player_pos(), target_item.clone(), n_yield);
                    gd.remove_item(*il, 1);
                    return;
                }
            }
            _ => (),
        }
    }

    match tool_obj.tool_effect {
        ToolEffect::Chop => {
            game_log_i!("chopping-no-tree");
        }
        _ => (),
    }
}

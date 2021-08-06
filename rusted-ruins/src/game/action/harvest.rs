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

    if item.remaining().map(|remaining| remaining.is_zero()) == Some(false) {
        game_log_i!("harvest-plant-not-ready"; item=item);
        return false;
    }

    finish_harvest(gd, CharaId::Player, item_idx, il);

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
        let harvest = if let Some(harvest) = item_obj
            .attrs
            .iter()
            .filter_map(|attr| match attr {
                ItemObjAttr::Harvest(harvest) => Some(harvest),
                _ => None,
            })
            .next()
        {
            harvest
        } else {
            continue;
        };
        let needed_turn = 10;

        let tool_effect = tool_obj
            .attrs
            .iter()
            .filter_map(|attr| match attr {
                ItemObjAttr::Tool(tool_effect) => Some(*tool_effect),
                _ => None,
            })
            .next();

        if harvest.kind == HarvestKind::Chop && tool_effect == Some(ToolEffect::Chop) {
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

    let tool_effect = tool_obj
        .attrs
        .iter()
        .filter_map(|attr| match attr {
            ItemObjAttr::Tool(tool_effect) => Some(*tool_effect),
            _ => None,
        })
        .next();

    if let Some(ToolEffect::Chop) = tool_effect {
        game_log_i!("chopping-no-tree");
    }
}

pub fn finish_harvest(gd: &mut GameData, cid: CharaId, item_idx: ItemIdx, il: ItemLocation) {
    let item = gd.get_item(il);
    if item.0.idx != item_idx {
        return;
    }

    let item_obj = gobj::get_obj(item_idx);

    let harvest = if let Some(harvest) = item_obj
        .attrs
        .iter()
        .filter_map(|attr| match attr {
            ItemObjAttr::Harvest(harvest) => Some(harvest),
            _ => None,
        })
        .next()
    {
        harvest
    } else {
        return;
    };

    let skill_level = gd.chara.get(cid).skill_level(harvest.kind.related_skill());

    for item in &harvest.item {
        let target_item_idx: ItemIdx = gobj::id_to_idx(&item.0);
        let target_item = crate::game::item::gen::gen_item_from_idx(target_item_idx, 0);
        let min_yield = std::cmp::min(item.1, item.2);
        let max_yield = std::cmp::max(item.1, item.2);
        let n_yield = if harvest.difficulty > skill_level {
            min_yield
        } else {
            std::cmp::min(max_yield, item.1 + skill_level - harvest.difficulty)
        };

        match harvest.kind {
            HarvestKind::Chop => {
                game_log_i!("harvest-chop"; chara=gd.chara.get(cid), item=&target_item, n=n_yield);
                audio::play_sound("chop-tree");
            }
            HarvestKind::Deconstruct => {
                game_log_i!("harvest-deconstruct"; chara=gd.chara.get(cid), item=&target_item, n=n_yield);
            }
            HarvestKind::Plant => {
                game_log_i!("harvest-plant"; chara=gd.chara.get(cid), item=&target_item, n=n_yield);
            }
            _ => (),
        }

        let item_list = gd.get_item_list_mut(ItemListLocation::Chara { cid });
        item_list.append(target_item, n_yield);
    }

    if let Some(&Some(repeat_duration)) =
        find_attr!(item_obj, ItemObjAttr::Plant { repeat_duration, ..} => repeat_duration)
    {
        gd.get_item_mut(il).0.reset_time(repeat_duration);
    } else {
        gd.remove_item(il, 1);
    }
}

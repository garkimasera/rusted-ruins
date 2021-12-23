use crate::game::extrait::*;
use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;
use script::GameMethod;

pub fn game_method_caller(gd: &mut GameData, method: GameMethod) -> Value {
    match method {
        GameMethod::GenDungeons => {
            let mid = gd.get_current_mapid();
            crate::game::region::gen_dungeon_max(gd, mid.rid());
            Value::None
        }
        GameMethod::GenPartyChara { id, lv } => {
            gd.gen_party_chara(&id, lv);
            Value::None
        }
        GameMethod::HasEmptyForParty => gd.has_empty_for_party().into(),
        GameMethod::NumberOfItem { id } => gd.has_item_by_id(&id).unwrap_or(0).into(),
        GameMethod::ReceiveItem { id, n } => {
            let item = crate::game::item::gen::gen_item_from_id(&id, 1);
            let il = gd.get_item_list_mut(ItemListLocation::PLAYER);
            il.append(item.clone(), n);
            let player = gd.chara.get_mut(CharaId::Player);
            game_log!("player-receive-item"; chara=player, item=item, n=n);
            player.update();
            Value::None
        }
        GameMethod::ReceiveMoney { amount } => {
            gd.player.add_money(amount);
            let player = gd.chara.get(CharaId::Player);
            game_log!("player-receive-money"; chara=player, amount=amount);
            Value::None
        }
        GameMethod::RemoveItem { id, n } => {
            let item_list = gd.get_item_list_mut(ItemListLocation::PLAYER);
            item_list.consume(gobj::id_to_idx(&id), n, |_, _| (), false);
            gd.chara.get_mut(CharaId::Player).update();
            Value::None
        }
        GameMethod::ResurrectPartyMembers => {
            crate::game::party::resurrect_party_members(gd);
            Value::None
        }
    }
}

use crate::game::extrait::*;
use common::gamedata::*;
use script::{set_game_methods, GameMethods};

pub fn init() {
    set_game_methods(GameMethods {
        gen_dungeons: self::gen_dungeons,
        receive_quest_rewards: crate::game::quest::receive_rewards,
        receive_item: self::receive_item,
        receive_money: self::receive_money,
    });
}

fn gen_dungeons(gd: &mut GameData) {
    let mid = gd.get_current_mapid();
    crate::game::region::gen_dungeon_max(gd, mid.rid());
}

fn receive_item(gd: &mut GameData, id: &str, n: u32) {
    let item = crate::game::item::gen::gen_item_from_id(&id, 1);
    let il = gd.get_item_list_mut(ItemListLocation::PLAYER);
    il.append(item.clone(), n);
    let player = gd.chara.get_mut(CharaId::Player);
    game_log_i!("player-receive-item"; chara=player, item=item, n=n);
    player.update();
}

fn receive_money(gd: &mut GameData, amount: u32) {
    gd.player.add_money(amount.into());
    let player = gd.chara.get(CharaId::Player);
    game_log_i!("player-receive-money"; chara=player, amount=amount);
}

use crate::game::{Game, InfoGetter};
use crate::text::ToText;
use common::gamedata::*;
use common::gobj;
use common::objholder::*;

pub fn exec_debug_command(game: &mut Game, command: &str) {
    let mut args = command.split_whitespace();
    let arg0 = if let Some(arg0) = args.next() {
        arg0
    } else {
        game_log_i!("debug-command-invalid");
        return;
    };

    match arg0 {
        "genchara" => {
            if let Some(arg1) = args.next() {
                gen_chara(game, arg1);
            } else {
                game_log_i!("debug-command-need-1arg"; command="genchara");
            }
        }
        "genitem" => {
            if let Some(arg1) = args.next() {
                gen_item(game, arg1);
            } else {
                game_log_i!("debug-command-need-1arg"; command="genitem");
            }
        }
        _ => {
            game_log_i!("debug-command-invalid");
        }
    }
}

fn gen_chara(game: &mut Game, arg1: &str) {
    let idx = if let Some(idx) = gobj::id_to_idx_checked::<CharaTemplateIdx>(arg1) {
        idx
    } else {
        game_log_i!("debug-command-failed"; command="genchara");
        return;
    };

    let gd = &mut game.gd;
    let mid = gd.get_current_mapid();
    if let Some(p) = super::map::choose_empty_tile(gd.region.get_map(mid)) {
        let chara = super::chara::gen::create_chara(idx, 1, FactionId::UNKNOWN);
        trace!("Generate new npc {}", chara.to_text());
        game_log_i!("debug-command-genchara"; chara=chara);
        let cid = gd.add_chara_to_map(chara, mid);
        let map = gd.region.get_map_mut(mid);
        map.locate_chara(cid, p);
    } else {
        warn!("Failed npc generating because empty tile not found");
    }
}

fn gen_item(game: &mut Game, arg1: &str) {
    let item_gen = ItemGen {
        id: arg1.to_owned(),
    };
    let item = if let Some(item) = crate::game::item::gen::from_item_gen(&item_gen) {
        item
    } else {
        game_log_i!("debug-command-failed"; command="genitem");
        return;
    };

    game_log_i!("debug-command-genitem"; item=item);
    let pos = game.gd.player_pos();
    game.gd.get_current_map_mut().locate_item(item, pos, 1);
}

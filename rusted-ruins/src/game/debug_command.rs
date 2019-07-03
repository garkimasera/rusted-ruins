
use crate::game::{Game, InfoGetter};
use common::gamedata::*;

pub fn exec_debug_command(game: &mut Game, command: &str) {
    let mut args = command.split_whitespace();
    let arg0 = if let Some(arg0) = args.next() {
        arg0
    } else {
        game_log_i!("debug-command-invalid");
        return;
    };

    match arg0 {
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


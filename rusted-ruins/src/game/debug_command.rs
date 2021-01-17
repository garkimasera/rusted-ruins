use crate::game::extrait::*;
use crate::game::Game;
use crate::text::ToText;
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use std::str::FromStr;

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
                let n = args
                    .next()
                    .map(|s| s.parse::<u32>().unwrap_or(1))
                    .unwrap_or(1);
                gen_item(game, arg1, n);
            } else {
                game_log_i!("debug-command-need-1arg"; command="genitem");
            }
        }
        "anim" => {
            if let Some(arg1) = args.next() {
                if let Some(idx) = gobj::id_to_idx_checked::<AnimImgIdx>(arg1) {
                    debug!("animation test: {}", arg1);
                    let anim = crate::game::Animation::img_onetile(
                        idx,
                        game.gd.chara_pos(CharaId::Player).unwrap(),
                    );

                    let n = args
                        .next()
                        .map(|s| s.parse::<u32>().unwrap_or(1))
                        .unwrap_or(1);

                    for _ in 0..n {
                        game.anim_queue.push(anim.clone());
                    }
                } else {
                    debug!("unknown animation id: {}", arg1);
                }
            } else {
                game_log_i!("debug-command-need-1arg"; command="anim");
            }
        }
        "learn_skill" => {
            let arg1 = if let Some(arg1) = args.next() {
                arg1
            } else {
                game_log_i!("debug-command-need-1arg"; command="learn_skill");
                return;
            };
            let skill_kind = match SkillKind::from_str(arg1) {
                Ok(o) => o,
                Err(e) => {
                    debug!("unknown skill kind: {}", e);
                    return;
                }
            };
            let player = game.gd.chara.get_mut(CharaId::Player);
            player.skills.learn_new_skill(skill_kind);
        }
        "print_ids" => {
            if let Some(arg1) = args.next() {
                let obj_holder = common::gobj::get_objholder();
                obj_holder.debug_print(arg1);
            } else {
                game_log_i!("debug-command-need-1arg"; command="print_ids");
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
        let chara = super::chara::gen::create_chara(idx, 1, FactionId::unknown(), None);
        trace!("Generate new npc {}", chara.to_text());
        game_log_i!("debug-command-genchara"; chara=chara);
        let cid = gd.add_chara_to_map(chara, mid);
        let map = gd.region.get_map_mut(mid);
        map.locate_chara(cid, p);
    } else {
        warn!("Failed npc generating because empty tile not found");
    }
}

fn gen_item(game: &mut Game, arg1: &str, n: u32) {
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
    game.gd.get_current_map_mut().locate_item(item, pos, n);
}

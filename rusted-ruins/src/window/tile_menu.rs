use super::commonuse::*;
use crate::config::CONTROL_CFG;
use crate::game::map::tile_info::*;
use crate::game::{DialogOpenRequest, InfoGetter};
use common::gamedata::*;
use common::gobj;
use geom::*;

use super::main_window::{CENTERING_START_REQ, CENTERING_STOP_REQ};

pub fn create_menu(
    game: &Game,
    tile: Coords,
    x: i32,
    y: i32,
    centering_mode: bool,
) -> Box<dyn DialogWindow> {
    let winpos = super::winpos::WindowPos::from_left_top(x, y);

    let mut text_ids = vec![];
    let mut callbacks: Vec<Box<dyn FnMut(&mut DoPlayerAction<'_>) + 'static>> = vec![];

    let t = tile_info_query(&game.gd, tile);
    let player_pos = game.gd.player_pos();
    let player_same_tile = tile == player_pos;
    let is_region_map = game.gd.get_current_mapid().is_region_map();

    if player_same_tile {
        match t.move_symbol {
            Some(SpecialTileKind::Stairs { kind, .. }) => {
                text_ids.push(match kind {
                    StairsKind::UpStairs => "tile-menu-up-stairs",
                    StairsKind::DownStairs => "tile-menu-down-stairs",
                });
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
                    pa.goto_next_floor(Direction::NONE, false);
                }));
            }
            Some(SpecialTileKind::SiteSymbol { .. }) => {
                text_ids.push("tile-menu-enter-site");
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
                    pa.goto_next_floor(Direction::NONE, false);
                }));
            }
            _ => (),
        }
        match t.boundary {
            None | Some((_, None)) => (),
            Some((dir, Some(Destination::Exit))) => {
                text_ids.push("tile-menu-exit-to-region-map");
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
                    pa.goto_next_floor(dir, false);
                }));
            }
            Some((dir, _)) => {
                text_ids.push("tile-menu-move-to-next-map");
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
                    pa.goto_next_floor(dir, false);
                }));
            }
        }
    }

    // Same tile or adjacent tile
    if player_same_tile || tile.is_adjacent(player_pos) {
        // Add harvest items
        let list = game.gd.search_harvestable_item(tile);
        for (il, item_idx) in &list {
            let item_obj = gobj::get_obj(*item_idx);

            let harvest = item_obj
                .attrs
                .iter()
                .filter_map(|attr| attr.harvest())
                .next()
                .unwrap();

            let il = *il;

            match harvest.kind {
                HarvestKind::Deconstruct => {
                    text_ids.push("tile-menu-deconstruct");
                    callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
                        pa.harvest_item(il);
                    }));
                }
                HarvestKind::Plant => {
                    text_ids.push("tile-menu-harvest");
                    callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
                        pa.harvest_item(il);
                    }));
                }
                _ => (),
            }
        }
    }

    // Pick up item
    if player_same_tile && !game.gd.item_on_player_tile().is_empty() {
        text_ids.push("tile-menu-pick-up-items");
        callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
            pa.request_dialog_open(DialogOpenRequest::PickUpItem);
        }));
    }

    // In region map
    if player_same_tile && is_region_map && t.move_symbol.is_none() {
        text_ids.push("tile-menu-enter-wilderness");
        callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
            pa.enter_wilderness(tile);
        }));
    }

    if !player_same_tile && t.chara.is_some() {
        text_ids.push("tile-menu-target");
        callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
            pa.set_target(tile);
        }));
    }

    if CONTROL_CFG.menu_centering {
        text_ids.push("tile-menu-start-centering");
        callbacks.push(Box::new(move |_| {
            *CENTERING_START_REQ.lock().unwrap() = Some(tile);
        }));
    }

    if centering_mode {
        text_ids.push("tile-menu-stop-centering");
        callbacks.push(Box::new(move |_| {
            CENTERING_STOP_REQ.store(true, std::sync::atomic::Ordering::Relaxed)
        }));
    }

    text_ids.push("tile-menu-information");
    callbacks.push(Box::new(move |pa: &mut DoPlayerAction<'_>| {
        pa.print_tile_info(tile);
    }));

    Box::new(super::choose_window::ChooseWindow::menu(
        winpos, text_ids, callbacks,
    ))
}

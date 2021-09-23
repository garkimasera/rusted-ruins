use super::DoPlayerAction;
use crate::game::map::builder::EntranceMethod;
use crate::game::map::MapExt;
use crate::game::{action, DialogOpenRequest, InfoGetter};
use crate::text::ToText;
use common::gamedata::*;
use geom::*;

impl<'a, 's> DoPlayerAction<'a, 's> {
    pub fn try_move(&mut self, dir: Direction) {
        let dest_tile = self
            .gd()
            .get_current_map()
            .chara_pos(CharaId::Player)
            .unwrap()
            + dir.as_vec();
        // If there is friendy chara on target tile, and have a trigger to start talk
        let will_talk = {
            if dir.as_vec() != (0, 0) {
                let gd = self.gd();
                if let Some(other_chara) = gd.get_current_map().get_chara(dest_tile) {
                    let relation = gd.chara_relation(CharaId::Player, other_chara);
                    let other_chara = gd.chara.get(other_chara);
                    match relation {
                        Relationship::Ally | Relationship::Friendly | Relationship::Neutral => {
                            other_chara.trigger_talk.is_some()
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            } else {
                false
            }
        };
        if will_talk {
            self.try_talk(dir);
            return;
        }
        // If destination is out of boundary
        if !self.gd().get_current_map().is_inside(dest_tile) {
            self.goto_next_floor(dir, true);
            return;
        }
        // Move to the next tile
        if action::try_move(self.0, CharaId::Player, dir) {
            self.0.finish_player_turn();
        }
    }

    pub fn move_to(&mut self, dest: Vec2d) {
        let d = dest - self.gd().player_pos();
        let hdir = if d.0 < 0 {
            HDirection::Left
        } else if d.0 > 0 {
            HDirection::Right
        } else {
            HDirection::None
        };
        let vdir = if d.1 < 0 {
            VDirection::Up
        } else if d.1 > 0 {
            VDirection::Down
        } else {
            VDirection::None
        };
        let dir = Direction::new(hdir, vdir);
        if dir.is_none() {
            return;
        }
        let map = self.gd().get_current_map();
        let player = self.gd().chara.get(CharaId::Player);
        let is_passable = |dir: Direction| {
            let dest_tile = map.chara_pos(CharaId::Player).unwrap() + dir.as_vec();
            map.is_passable(player, dest_tile)
        };

        let dir = if is_passable(dir) {
            dir
        } else if is_passable(Direction::new(dir.hdir, VDirection::None)) {
            Direction::new(dir.hdir, VDirection::None)
        } else if is_passable(Direction::new(HDirection::None, dir.vdir)) {
            Direction::new(HDirection::None, dir.vdir)
        } else {
            return;
        };

        if dir.is_none() {
            return;
        }

        self.try_move(dir);
    }

    /// Try to go to next floor
    /// This function will be called when players use stairs or try to exit from map boundaries.
    /// In the latter case, dir is not None and represents player's move direction.
    pub fn goto_next_floor(&mut self, dir: Direction, dialog: bool) {
        enum LogMessage {
            ExitToOutside,
            EnterSite(String),
            ChangeFloor,
        }

        let log_msg: LogMessage;

        // Use stairs
        if dir.is_none() {
            let (dest, msg) = {
                let gd = self.gd_mut();
                let mid = gd.get_current_mapid();
                let special_tile_kind = &gd.get_current_map().tile[gd.player_pos()].special;
                match *special_tile_kind {
                    SpecialTileKind::Stairs { dest_floor, .. } => {
                        // Use stairs on map
                        let dest = if dest_floor == FLOOR_OUTSIDE {
                            log_msg = LogMessage::ExitToOutside;
                            Destination::Exit
                        } else {
                            log_msg = LogMessage::ChangeFloor;
                            Destination::Floor(dest_floor)
                        };
                        let next_mid = crate::game::map::destination_to_mid(self.gd(), dest);
                        (dest, msg_switch_map(next_mid))
                    }
                    SpecialTileKind::SiteSymbol { .. } => {
                        // Enter other site from region map
                        let pos = gd.player_pos();
                        let region = gd.region.get(mid.rid());
                        if let Some(sid) = region.get_id_by_pos(pos) {
                            let mid = MapId::site_first_floor(sid);
                            let site = gd.region.get_site(sid);
                            let msg = ui_txt_format!("dialog-enter_site"; site_name=site);
                            log_msg = LogMessage::EnterSite(site.to_text().to_string());
                            (Destination::MapIdWithEntrance(mid, 0), msg)
                        } else {
                            warn!("No site existed at {:?}", pos);
                            return;
                        }
                    }
                    _ => {
                        warn!("Tried to use stairs that don't exist");
                        return;
                    }
                }
            };

            let cb = Box::new(move |pa: &mut DoPlayerAction<'_, '_>, result: bool| {
                if !result {
                    return;
                }
                match &log_msg {
                    LogMessage::ExitToOutside => {
                        game_log!("exit-to-outside"; player=pa.gd().chara.get(CharaId::Player));
                    }
                    LogMessage::EnterSite(s) => {
                        game_log!("enter-site"; player=pa.gd().chara.get(CharaId::Player), site=s);
                    }
                    LogMessage::ChangeFloor => {
                        game_log!("change-floor"; player=pa.gd().chara.get(CharaId::Player));
                    }
                }
                crate::game::map::switch_map(pa.0, dest);
            });
            if dialog {
                self.0
                    .request_dialog_open(DialogOpenRequest::YesNo { callback: cb, msg });
            } else {
                cb(self, true);
            }
        } else {
            // Crossing boundary
            let boundary_dest = {
                let player_pos = self.gd().player_pos();
                let map = self.gd().get_current_map();
                map.get_boundary_by_tile_and_dir(player_pos, dir)
            };
            let dest = if let Some(dest) = boundary_dest {
                dest
            } else {
                return;
            };
            let log_msg = match dest {
                Destination::Exit => LogMessage::ExitToOutside,
                _ => LogMessage::ChangeFloor,
            };
            let cb = Box::new(move |pa: &mut DoPlayerAction<'_, '_>, result: bool| {
                if !result {
                    return;
                }
                match &log_msg {
                    LogMessage::ExitToOutside => {
                        game_log!("exit-to-outside"; player=pa.gd().chara.get(CharaId::Player));
                    }
                    LogMessage::EnterSite(s) => {
                        game_log!("enter-site"; player=pa.gd().chara.get(CharaId::Player), site=s);
                    }
                    LogMessage::ChangeFloor => {
                        game_log!("change-floor"; player=pa.gd().chara.get(CharaId::Player));
                    }
                }
                crate::game::map::switch_map(pa.0, dest);
            });
            let next_mid = crate::game::map::destination_to_mid(self.gd(), dest);
            if dialog {
                self.0.request_dialog_open(DialogOpenRequest::YesNo {
                    callback: cb,
                    msg: msg_switch_map(next_mid),
                });
            } else {
                cb(self, true);
            }
        }
    }

    pub fn enter_wilderness(&mut self, pos: Vec2d) {
        if let Some(map) = crate::game::map::wilderness::generate_wilderness(
            self.gd(),
            pos,
            EntranceMethod::Original,
        ) {
            let mid = self.gd().get_current_mapid();
            let rid = mid.rid();
            let site_content = SiteContent::Temp {
                return_map: mid,
                return_pos: pos,
                is_open_air: true,
            };
            let wilderness_mid = crate::game::site::temp::gen_temp_site_from_map(
                self.gd_mut(),
                rid,
                map,
                "wilderness",
                site_content,
            );
            crate::game::map::switch_map(self.0, Destination::MapId(wilderness_mid));
        } else {
            warn!("cannot generate wilderness map for given position");
        }
    }
}

fn msg_switch_map(next_mid: MapId) -> String {
    if next_mid.is_region_map() {
        crate::text::ui_txt("dialog-exit_to_regionmap")
    } else {
        crate::text::ui_txt("dialog-move_floor")
    }
}

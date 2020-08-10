use crate::config::{CONTROL_CFG, SCREEN_CFG};
use crate::context::*;
use crate::draw::mainwin::MainWinDrawer;
use crate::game::command::MouseButton;
use crate::game::{Animation, Command, DialogOpenRequest, DoPlayerAction, Game, InfoGetter};
use crate::window::{DialogWindow, Window};
use common::gobj;
use geom::*;
use sdl2::rect::Rect;
use std::sync::Mutex;

lazy_static! {
    static ref CENTERING_START_REQ: Mutex<Option<Vec2d>> = Mutex::new(None);
    static ref CENTERING_STOP_REQ: Mutex<bool> = Mutex::new(false);
}

pub struct MainWindow {
    rect: Rect,
    drawer: MainWinDrawer,
    centering_tile: Option<Vec2d>,
    hover_tile: Option<Vec2d>,
}

pub enum ConvertMouseEventResult {
    None,
    Command(Command),
    OpenWindow(Box<dyn DialogWindow>),
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let rect = SCREEN_CFG.main_window.into();
        MainWindow {
            rect,
            drawer: MainWinDrawer::new(rect),
            centering_tile: None,
            hover_tile: None,
        }
    }

    pub fn start_centering_mode(&mut self, tile: Vec2d) {
        info!("Start centering mode");
        self.centering_tile = Some(tile);
    }

    pub fn stop_centering_mode(&mut self) {
        info!("Stop centering mode");
        self.centering_tile = None;
    }

    /// Convert mouse event on main window to Command
    pub fn convert_mouse_event(
        &mut self,
        command: Command,
        game: &Game,
    ) -> ConvertMouseEventResult {
        match command {
            Command::MouseButtonDown { .. } => ConvertMouseEventResult::None,
            Command::MouseButtonUp { x, y, button, .. } => {
                if !self.rect.contains_point((x, y)) {
                    return ConvertMouseEventResult::None;
                }
                let tile = self.cursor_pos_to_tile(x, y);
                if button == MouseButton::Right {
                    return ConvertMouseEventResult::OpenWindow(create_menu(
                        game,
                        tile,
                        x,
                        y,
                        self.centering_tile.is_some(),
                    ));
                }

                if button == MouseButton::Middle {
                    let tile = self.cursor_pos_to_tile(x, y);
                    self.start_centering_mode(tile);
                }

                ConvertMouseEventResult::None
            }
            Command::MouseWheel { .. } => ConvertMouseEventResult::None,
            Command::MouseState {
                x,
                y,
                left_button,
                key_state,
                ui_only,
                ..
            } => {
                if ui_only {
                    return ConvertMouseEventResult::None;
                }
                if !self.rect.contains_point((x, y)) {
                    return ConvertMouseEventResult::None;
                }
                let tile = self.cursor_pos_to_tile(x, y);
                self.hover_tile = Some(tile);

                if left_button {
                    if key_state.ctrl {
                        return ConvertMouseEventResult::Command(Command::Shoot { target: tile });
                    } else if key_state.shift {
                        return ConvertMouseEventResult::Command(Command::UseTool { target: tile });
                    } else {
                        return ConvertMouseEventResult::Command(Command::MoveTo { dest: tile });
                    }
                }

                ConvertMouseEventResult::None
            }
            _ => ConvertMouseEventResult::Command(command),
        }
    }

    fn cursor_pos_to_tile(&self, x: i32, y: i32) -> Vec2d {
        let x = x - self.rect.x;
        let y = y - self.rect.y;
        self.drawer.pos_to_tile(x, y)
    }
}

impl Window for MainWindow {
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {
        let mut centering_start_req = CENTERING_START_REQ.lock().unwrap();
        if let Some(tile) = *centering_start_req {
            self.centering_tile = Some(tile);
            *centering_start_req = None;
        }

        let mut centering_stop_req = CENTERING_STOP_REQ.lock().unwrap();
        if *centering_stop_req {
            self.centering_tile = None;
            *centering_stop_req = false;
        }

        self.drawer
            .draw(context, game, anim, self.centering_tile, self.hover_tile);
    }
}

fn create_menu(
    game: &Game,
    tile: Vec2d,
    x: i32,
    y: i32,
    centering_mode: bool,
) -> Box<dyn DialogWindow> {
    use crate::game::map::tile_info::*;
    use common::gamedata::{BoundaryBehavior, SpecialTileKind, StairsKind};

    let winpos = super::winpos::WindowPos::from_left_top(x, y);

    let mut text_ids = vec![];
    let mut callbacks: Vec<Box<dyn FnMut(&mut DoPlayerAction) + 'static>> = vec![];

    let t = tile_info_query(&game.gd, tile);
    let player_pos = game.gd.player_pos();
    let player_same_tile = tile == player_pos;

    if player_same_tile {
        match t.move_symbol {
            Some(SpecialTileKind::Stairs { kind, .. }) => {
                text_ids.push(match kind {
                    StairsKind::UpStairs => "tile-menu-up-stairs",
                    StairsKind::DownStairs => "tile-menu-down-stairs",
                });
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
                    pa.goto_next_floor(Direction::NONE, false);
                }));
            }
            Some(SpecialTileKind::SiteSymbol { .. }) => {
                text_ids.push("tile-menu-enter-site");
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
                    pa.goto_next_floor(Direction::NONE, false);
                }));
            }
            _ => (),
        }
        match t.boundary {
            None | Some((_, BoundaryBehavior::None)) => (),
            Some((dir, BoundaryBehavior::RegionMap)) => {
                text_ids.push("tile-menu-exit-to-region-map");
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
                    pa.goto_next_floor(dir, false);
                }));
            }
            Some((dir, _)) => {
                text_ids.push("tile-menu-move-to-next-map");
                callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
                    pa.goto_next_floor(dir, false);
                }));
            }
        }
        if !game.gd.item_on_player_tile().is_empty() {
            text_ids.push("tile-menu-pick-up-items");
            callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
                pa.request_dialog_open(DialogOpenRequest::PickUpItem);
            }));
        }
    }

    // Same tile or adjacent tile
    if player_same_tile || tile.is_adjacent(player_pos) {
        // Add harvest items
        let list = game.gd.search_harvestable_item(tile);
        for (_il, item_idx) in &list {
            let item_obj = gobj::get_obj(*item_idx);
            let harvest = item_obj.harvest.as_ref().unwrap();

            match harvest.harvest_type {
                _ => (),
            }
        }
    }

    if !player_same_tile {
        if t.chara.is_some() {
            text_ids.push("tile-menu-target");
            callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
                pa.set_target(tile);
            }));
        }
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
            *CENTERING_STOP_REQ.lock().unwrap() = true;
        }));
    }

    text_ids.push("tile-menu-infomation");
    callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
        pa.print_tile_info(tile);
    }));

    Box::new(super::choose_window::ChooseWindow::menu(
        winpos, text_ids, callbacks,
    ))
}

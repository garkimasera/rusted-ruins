use crate::config::SCREEN_CFG;
use crate::context::*;
use crate::draw::mainwin::{MainWinDrawer, TargetModeDrawInfo};
use crate::game::command::MouseButton;
use crate::game::{Animation, Command, DoPlayerAction, Game, InfoGetter, Target};
use crate::window::{DialogWindow, Window};
use common::gamedata::Effect;
use geom::*;
use once_cell::sync::Lazy;
use sdl2::rect::Rect;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub(super) static CENTERING_START_REQ: Lazy<Mutex<Option<Coords>>> = Lazy::new(|| Mutex::new(None));
pub(super) static CENTERING_STOP_REQ: AtomicBool = AtomicBool::new(false);

enum MainWindowMode {
    Normal,
    Target {
        callback: Box<dyn Fn(&mut DoPlayerAction<'_>, Target) + 'static>,
        draw_info: TargetModeDrawInfo,
    },
    // TargetPreview,
}

impl MainWindowMode {
    fn input_target(&self) -> bool {
        matches!(self, MainWindowMode::Target { .. })
    }

    fn get_draw_info(&mut self) -> Option<&mut TargetModeDrawInfo> {
        match self {
            MainWindowMode::Target { draw_info, .. } => Some(draw_info),
            _ => None,
        }
    }
}

pub struct MainWindow {
    rect: Rect,
    drawer: MainWinDrawer,
    centering_tile: Option<Coords>,
    hover_tile: Option<Coords>,
    mode: MainWindowMode,
}

pub enum ConvertMouseEventResult {
    None,
    Command(Command),
    OpenWindow(Box<dyn DialogWindow>),
    DoAction(Box<dyn Fn(&mut DoPlayerAction<'_>) + 'static>),
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let rect = SCREEN_CFG.main_window.into();
        MainWindow {
            rect,
            drawer: MainWinDrawer::new(rect),
            centering_tile: None,
            hover_tile: None,
            mode: MainWindowMode::Normal,
        }
    }

    pub fn start_centering_mode(&mut self, tile: Coords) {
        info!("Start centering mode");
        self.centering_tile = Some(tile);
    }

    pub fn stop_centering_mode(&mut self) {
        info!("Stop centering mode");
        self.centering_tile = None;
    }

    pub fn update_tile_cursor(&mut self, pos: (i32, i32)) -> Option<Coords> {
        if self.rect.contains_point(pos) {
            let tile = self.cursor_pos_to_tile(pos.0, pos.1);
            self.hover_tile = Some(tile);
            Some(tile)
        } else {
            None
        }
    }

    pub fn reset_tile_cursor(&mut self) {
        self.hover_tile = None;
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
                    if self.mode.input_target() {
                        self.mode = MainWindowMode::Normal;
                    }
                    return ConvertMouseEventResult::None;
                }
                let tile = self.cursor_pos_to_tile(x, y);

                if button == MouseButton::Right {
                    if self.mode.input_target() {
                        self.mode = MainWindowMode::Normal;
                        return ConvertMouseEventResult::None;
                    } else {
                        return ConvertMouseEventResult::OpenWindow(super::tile_menu::create_menu(
                            game,
                            tile,
                            x,
                            y,
                            self.centering_tile.is_some(),
                        ));
                    }
                }

                if button == MouseButton::Middle {
                    let tile = self.cursor_pos_to_tile(x, y);
                    self.start_centering_mode(tile);
                }

                if button == MouseButton::Left {
                    let pos = self.cursor_pos_to_tile(x, y);
                    if let Some(info) = self.mode.get_draw_info() {
                        let map = game.gd.get_current_map();
                        if info.range.is_inside(pos)
                            && map.is_inside(pos)
                            && game.view_map.get_tile_visible(pos)
                        {
                            let mode = std::mem::replace(&mut self.mode, MainWindowMode::Normal);
                            if let MainWindowMode::Target { callback, .. } = mode {
                                let callback: Box<dyn Fn(&mut DoPlayerAction<'_>) + 'static> =
                                    Box::new(move |pa| callback(pa, Target::Tile(pos)));
                                return ConvertMouseEventResult::DoAction(callback);
                            }
                        }
                    }
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

                if self.mode.input_target() {
                    return ConvertMouseEventResult::None;
                }

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

    pub fn start_targeting_mode(
        &mut self,
        game: &Game,
        effect: Effect,
        callback: Box<dyn Fn(&mut DoPlayerAction<'_>, Target) + 'static>,
    ) {
        let center = game.gd.player_pos();
        let range = crate::game::effect::effect_to_range(&effect, center);
        let draw_info = TargetModeDrawInfo { range };
        self.mode = MainWindowMode::Target {
            callback,
            draw_info,
        };
    }

    fn cursor_pos_to_tile(&self, x: i32, y: i32) -> Coords {
        let x = x - self.rect.x;
        let y = y - self.rect.y;
        self.drawer.pos_to_tile(x, y)
    }
}

impl Window for MainWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        let mut centering_start_req = CENTERING_START_REQ.lock().unwrap();
        if let Some(tile) = *centering_start_req {
            self.centering_tile = Some(tile);
            *centering_start_req = None;
        }

        if CENTERING_STOP_REQ.load(Ordering::Relaxed) {
            self.centering_tile = None;
            CENTERING_STOP_REQ.store(false, Ordering::Relaxed);
        }

        self.drawer.draw(
            context,
            game,
            anim,
            self.centering_tile,
            self.hover_tile,
            self.mode.get_draw_info().map(|i| &*i),
        );
    }
}

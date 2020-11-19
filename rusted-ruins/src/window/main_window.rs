use crate::config::SCREEN_CFG;
use crate::context::*;
use crate::draw::mainwin::MainWinDrawer;
use crate::game::command::MouseButton;
use crate::game::{Animation, Command, Game};
use crate::window::{DialogWindow, Window};
use geom::*;
use sdl2::rect::Rect;
use std::sync::Mutex;

lazy_static! {
    pub(super) static ref CENTERING_START_REQ: Mutex<Option<Vec2d>> = Mutex::new(None);
    pub(super) static ref CENTERING_STOP_REQ: Mutex<bool> = Mutex::new(false);
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
                    return ConvertMouseEventResult::OpenWindow(super::tile_menu::create_menu(
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

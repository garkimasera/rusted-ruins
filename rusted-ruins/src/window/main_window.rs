use crate::config::SCREEN_CFG;
use crate::context::*;
use crate::draw::mainwin::MainWinDrawer;
use crate::game::command::MouseButton;
use crate::game::{Animation, Command, DoPlayerAction, Game, InfoGetter};
use crate::window::{DialogWindow, Window};
use geom::*;
use sdl2::rect::Rect;

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

    pub fn start_targeting_mode(&mut self, game: &Game) {
        info!("Start targeting mode");
        self.centering_tile = Some(game.gd.player_pos());
    }

    pub fn stop_targeting_mode(&mut self) {
        info!("Stop targeting mode");
        self.centering_tile = None;
    }

    pub fn get_current_centering_tile(&mut self) -> Vec2d {
        self.centering_tile
            .expect("get_current_centering tile must called when targeting mode")
    }

    pub fn move_centering_tile(&mut self, dir: Direction, game: &Game) {
        let mut c = if let Some(c) = self.centering_tile {
            c
        } else {
            return;
        };
        let limit = game.gd.map_size();

        match dir.hdir {
            HDirection::Left => {
                if c.0 > 0 {
                    c.0 -= 1;
                }
            }
            HDirection::Right => {
                if c.0 < limit.0 as i32 - 1 {
                    c.0 += 1;
                }
            }
            HDirection::None => (),
        }

        match dir.vdir {
            VDirection::Up => {
                if c.1 > 0 {
                    c.1 -= 1;
                }
            }
            VDirection::Down => {
                if c.1 < limit.1 as i32 - 1 {
                    c.1 += 1;
                }
            }
            VDirection::None => (),
        }

        self.centering_tile = Some(c);
    }

    /// Convert mouse event on main window to Command
    pub fn convert_mouse_event(
        &mut self,
        command: Command,
        game: &Game,
    ) -> ConvertMouseEventResult {
        match command {
            Command::MouseButtonDown { .. } => ConvertMouseEventResult::None,
            Command::MouseButtonUp { x, y, button } => {
                if !self.rect.contains_point((x, y)) {
                    return ConvertMouseEventResult::None;
                }
                let tile = self.cursor_pos_to_tile(x, y);
                if button == MouseButton::Right {
                    ConvertMouseEventResult::OpenWindow(create_menu(game, tile, x, y))
                } else {
                    ConvertMouseEventResult::None
                }
            }
            Command::MouseWheel { .. } => ConvertMouseEventResult::None,
            Command::MouseState {
                x, y, left_button, ..
            } => {
                if !self.rect.contains_point((x, y)) {
                    return ConvertMouseEventResult::None;
                }
                let tile = self.cursor_pos_to_tile(x, y);
                self.hover_tile = Some(tile);

                if left_button {
                    return ConvertMouseEventResult::Command(Command::MoveTo { dest: tile });
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
        self.drawer
            .draw(context, game, anim, self.centering_tile, self.hover_tile);
    }
}

fn create_menu(game: &Game, tile: Vec2d, x: i32, y: i32) -> Box<dyn DialogWindow> {
    let winpos = super::winpos::WindowPos::from_left_top(x, y);

    let mut text_ids = vec![];
    let mut callbacks: Vec<Box<dyn FnMut(&mut DoPlayerAction) + 'static>> = vec![];

    text_ids.push("tile-menu-infomation");
    callbacks.push(Box::new(move |pa: &mut DoPlayerAction| {
        pa.print_tile_info(tile);
    }));

    Box::new(super::choose_window::ChooseWindow::menu(
        winpos, text_ids, callbacks,
    ))
}

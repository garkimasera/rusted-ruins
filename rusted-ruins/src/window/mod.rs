
mod mainwindow;
mod logwindow;
mod textwindow;
mod itemwindow;
mod exitwindow;
mod yesnodialog;
mod textinputdialog;
mod indicator;
mod minimap;
mod widget;

use game::{GameState, DoPlayerAction, InfoGetter};
use eventhandler::EventHandler;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::keyboard::TextInputUtil;
use SdlContext;
use self::mainwindow::MainWindow;
use self::logwindow::LogWindow;
use self::widget::WidgetTrait;
use array2d::*;

mod commonuse {
    pub use window::{Window, DialogWindow, DialogResult};
    pub use sdl2::render::WindowCanvas;
    pub use sdl2::rect::Rect;
    pub use sdlvalues::SdlValues;
    pub use game::{Game, Animation, Command, DoPlayerAction};
    pub use config::{SCREEN_CFG, UI_CFG};
    pub use draw::border::draw_rect_border;
    pub use eventhandler::InputMode;
}

use self::commonuse::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DialogResult {
    Continue, Close, CloseAll, Quit,
}

pub trait Window {
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>);
}

pub trait DialogWindow: Window {
    fn process_command(&mut self, command: Command, pa: DoPlayerAction) -> DialogResult;
    /// Return InputMode for this window
    fn mode(&self) -> InputMode;
}

/// Manage all windows
pub struct WindowManager<'sdl, 't> {
    game: Game,
    sdl_values: SdlValues<'sdl, 't>,
    text_input_util: TextInputUtil,
    game_windows: GameWindows,
    anim: Option<Animation>,
    passed_frame: u32,
    window_stack: Vec<Box<DialogWindow>>,
}

impl<'sdl, 't> WindowManager<'sdl, 't> {
    pub fn new(
        sdl_context: &'sdl SdlContext,
        texture_creator: &'t TextureCreator<WindowContext>) -> WindowManager<'sdl, 't> {
        
        let game = Game::new();
        let sdl_values = SdlValues::new(sdl_context, texture_creator);
        let text_input_util = sdl_context.sdl_context.video().unwrap().text_input();
        
        WindowManager {
            game: game,
            sdl_values: sdl_values,
            text_input_util: sdl_context.sdl_context.video().unwrap().text_input(),
            game_windows: GameWindows::new(),
            anim: None,
            passed_frame: 0,
            window_stack: Vec::new(),
        }
    }

    // If return value is false, quit.
    pub fn advance_turn(&mut self, event_handler: &mut EventHandler) -> bool {
        // Animation must be finished before 
        assert!(self.anim.is_none());
        
        if self.game.get_state() == GameState::WaitingForNextTurn {
            self.game.advance_turn();
        }

        if self.game.get_state() == GameState::PlayerTurn {
            if !self.process_command(event_handler) { return false; }
        }
        // After advancing turn and processing command, game may start animation.
        self.anim = self.game.pop_animation();

        true
    }

    pub fn redraw(&mut self, canvas: &mut WindowCanvas) {
        let mut is_animation_over = false;
        if let Some(anim) = self.anim.as_mut() {
            if self.passed_frame >= anim.get_n_frame() {
                is_animation_over = true;
                self.passed_frame = 0;
            }
        }

        // Pop next animation
        if is_animation_over {
            self.anim = self.game.pop_animation();
        }

        let anim = self.anim.as_ref().map(|a| (a, self.passed_frame));

        self.game_windows.redraw(canvas, &self.game, &mut self.sdl_values, anim);

        for w in &mut self.window_stack {
            w.redraw(canvas, &self.game, &mut self.sdl_values, anim);
        }

        if anim.is_some() {
            self.passed_frame += 1;
        }
    }

    pub fn animation_now(&self) -> bool {
        self.anim.is_some()
    }

    // If return value is false, quit.
    pub fn process_command(&mut self, event_handler: &mut EventHandler) -> bool {
        text_input::check_mode(&self.text_input_util);
        
        let mode = if self.window_stack.len() > 0 {
            self.window_stack[self.window_stack.len() - 1].mode()
        }else{
            InputMode::Normal
        };
        
        let command = event_handler.get_command(mode);
        if command.is_none() { return true; }
        let command = command.unwrap();
        
        use game::playeract::DoPlayerAction;
        use game::Command;
        let mut pa = DoPlayerAction::new(&mut self.game);

        if self.window_stack.len() > 0 {
            let tail = self.window_stack.len() - 1;
            match self.window_stack[tail].process_command(command, pa) {
                DialogResult::Continue => (),
                DialogResult::Close => { self.window_stack.pop(); },
                DialogResult::CloseAll => { self.window_stack.clear(); },
                DialogResult::Quit => { return false; },
            }
            return true;
        }
        
        // Process when any window are not opened
        match command {
            Command::Move{ dir } => {
                pa.try_move(dir);
            },
            Command::Enter => {
                if pa.gd().on_map_entrance() {
                    self.window_stack.push(Box::new(yesnodialog::YesNoDialog::new(
                        ::text::ui_txt("dialog.move_floor"),
                        |pa| {
                            pa.goto_next_floor(Direction::none());
                            DialogResult::Close
                        }
                    )));
                }
            },
            Command::OpenExitWin => {
                self.window_stack.push(Box::new(exitwindow::ExitWindow::new()));
            },
            Command::OpenItemMenu => {
                use self::itemwindow::*;
                self.window_stack.push(Box::new(ItemWindow::new(ItemWindowMode::List)));
            },
            Command::PickUpItem => {
                use self::itemwindow::*;
                self.window_stack.push(Box::new(ItemWindow::new(ItemWindowMode::PickUp)));
            }
            _ => (),
        }
        true
    }
}

/// Functions for setting text_input_util state
pub mod text_input {
    use sdl2::keyboard::TextInputUtil;
    use std::cell::Cell;
    thread_local!(static TEXT_INPUT: Cell<bool> = Cell::new(false));

    pub fn get() -> bool {
        TEXT_INPUT.with(|text_input| {
            text_input.get()
        })
    }

    pub fn check_mode(text_input_util: &TextInputUtil) {
        let active = text_input_util.is_active();
        if !active && get() {
            text_input_util.start();
        }
        if active && !get() {
            text_input_util.stop();
        }
    }

    pub fn start() {
        TEXT_INPUT.with(|text_input| {
            text_input.set(true);
        });
    }

    pub fn end() {
        TEXT_INPUT.with(|text_input| {
            text_input.set(false);
        });
    }
}

/// These windows are displayed after a game is started
struct GameWindows {
    main_window: MainWindow,
    log_window: LogWindow,
    minimap_window: minimap::MiniMapWindow,
    indicator: indicator::HPIndicator,
    floor_info: indicator::FloorInfo,
    hborders: Vec<self::widget::HBorder>,
    vborders: Vec<self::widget::VBorder>,
}

impl GameWindows {
    fn new() -> GameWindows {
        use config::SCREEN_CFG;
        use self::widget::{HBorder, VBorder};
        let mut hborders = Vec::new();
        for hborder in &SCREEN_CFG.hborders {
            hborders.push(HBorder::new(
                (hborder.x, hborder.y), hborder.len));
        }
        let mut vborders = Vec::new();
        for vborder in &SCREEN_CFG.vborders {
            vborders.push(VBorder::new(
                (vborder.x, vborder.y), vborder.len));
        }
        
        GameWindows {
            main_window: MainWindow::new(),
            log_window:  LogWindow ::new(),
            minimap_window: minimap::MiniMapWindow::new(),
            indicator: indicator::HPIndicator::new(),
            floor_info: indicator::FloorInfo::new(),
            hborders: hborders,
            vborders: vborders,
        }
    }

    fn redraw(&mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
              anim: Option<(&Animation, u32)>) {
        
        self.main_window.redraw(canvas, game, sv, anim);
        self.log_window.redraw(canvas, game, sv, anim);
        self.minimap_window.redraw(canvas, game, sv, anim);
        self.indicator.redraw(canvas, game, sv, anim);
        self.floor_info.redraw(canvas, game, sv, anim);

        for hborder in self.hborders.iter_mut() {
            hborder.draw(canvas, sv);
        }
        for vborder in self.vborders.iter_mut() {
            vborder.draw(canvas, sv);
        }
    }
}


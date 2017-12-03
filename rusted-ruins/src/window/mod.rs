
mod mainwindow;
mod logwindow;
mod textwindow;
mod itemwindow;
mod exitwindow;
mod equipwindow;
mod yesnodialog;
mod textinputdialog;
mod indicator;
mod minimap;
mod startwindow;
mod widget;

use game::{GameState, DoPlayerAction, InfoGetter, Command};
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

pub enum DialogResult {
    Continue, Close, CloseAll, Quit, OpenChildDialog(Box<DialogWindow>), User(u32),
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

/// The current main mode
pub enum WindowManageMode {
    Start(self::startwindow::StartWindow), OnGame(GameWindows),
}

impl WindowManageMode {
    pub fn is_on_game(&self) -> bool {
        match self {
            &WindowManageMode::OnGame(_) => true,
            _ => false,
        }
    }
}

/// Manage all windows
pub struct WindowManager<'sdl, 't> {
    game: Game,
    mode: WindowManageMode,
    sdl_values: SdlValues<'sdl, 't>,
    text_input_util: TextInputUtil,
    anim: Option<Animation>,
    passed_frame: u32,
    window_stack: Vec<Box<DialogWindow>>,
    child_window_closed: bool,
}

impl<'sdl, 't> WindowManager<'sdl, 't> {
    pub fn new(
        sdl_context: &'sdl SdlContext,
        texture_creator: &'t TextureCreator<WindowContext>) -> WindowManager<'sdl, 't> {
        
        let game = Game::empty();
        let sdl_values = SdlValues::new(sdl_context, texture_creator);
        let text_input_util = sdl_context.sdl_context.video().unwrap().text_input();
        let mut window_stack: Vec<Box<DialogWindow>> = Vec::new();
        window_stack.push(Box::new(self::startwindow::StartDialog::new()));
        
        WindowManager {
            game: game,
            mode: WindowManageMode::Start(self::startwindow::StartWindow::new()),
            sdl_values: sdl_values,
            text_input_util: sdl_context.sdl_context.video().unwrap().text_input(),
            anim: None,
            passed_frame: 0,
            window_stack: window_stack,
            child_window_closed: false,
        }
    }

    // If return value is false, quit.
    pub fn advance_turn(&mut self, event_handler: &mut EventHandler) -> bool {
        // Animation must be finished before 
        assert!(self.anim.is_none());
        
        if self.game.get_state() == GameState::WaitingForNextTurn && self.mode.is_on_game() {
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

        // Draw windows
        match self.mode {
            WindowManageMode::Start(ref mut start_window) => {
                start_window.redraw(canvas, &self.game, &mut self.sdl_values, anim);
            }
            WindowManageMode::OnGame(ref mut game_windows) => {
                game_windows.redraw(canvas, &self.game, &mut self.sdl_values, anim);
            }
        }

        // Draw dialog windows
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
        } else {
            InputMode::Normal
        };
        
        let command = if self.child_window_closed {
            self.child_window_closed = false;
            if self.window_stack.len() > 0 {
                Some(Command::ChildWindowClosed)
            } else {
                event_handler.get_command(mode)
            }
        } else {
            event_handler.get_command(mode)
        };
        if command.is_none() { return true; }
        let command = command.unwrap();
        
        use game::playeract::DoPlayerAction;

        if self.window_stack.len() > 0 {
            let dialog_result = {
                let pa = DoPlayerAction::new(&mut self.game);
                let tail = self.window_stack.len() - 1;
                self.window_stack[tail].process_command(command, pa)
            };
            match dialog_result {
                DialogResult::Continue => (),
                DialogResult::Close => {
                    self.window_stack.pop();
                    self.child_window_closed = true;
                }
                DialogResult::CloseAll => { self.window_stack.clear(); }
                DialogResult::Quit => { return false; }
                DialogResult::OpenChildDialog(child) => {
                    self.window_stack.push(child);
                }
                DialogResult::User(n) => { self.process_user_result(n); }
            }
            return true;
        }
        
        // If self.mode is OnGame
        let mut pa = DoPlayerAction::new(&mut self.game);
        match command {
            Command::Move{ dir } => {
                pa.try_move(dir);
            }
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
            }
            Command::OpenExitWin => {
                self.window_stack.push(Box::new(exitwindow::ExitWindow::new()));
            }
            Command::OpenItemMenu => {
                use self::itemwindow::*;
                self.window_stack.push(Box::new(ItemWindow::new(ItemWindowMode::List, pa)));
            }
            Command::OpenEquipWin => {
                use common::gamedata::chara::CharaId;
                use self::equipwindow::*;
                self.window_stack.push(Box::new(equipwindow::EquipWindow::new(pa, CharaId::Player)));
            }
            Command::PickUpItem => {
                if pa.gd().item_on_player_tile().is_some() {
                    use self::itemwindow::*;
                    let item_window = ItemWindow::new(ItemWindowMode::PickUp, pa);
                    self.window_stack.push(Box::new(item_window));
                }
            }
            _ => (),
        }
        true
    }

    pub fn process_user_result(&mut self, n: u32) {

        match self.mode {
            WindowManageMode::Start(_) => {
                match n {
                    // Start new game
                    self::startwindow::START_DIALOG_RESULT_NEWGAME => {
                        self.window_stack.pop();
                        self.mode = WindowManageMode::OnGame(GameWindows::new());

                        let game = Game::new();
                        self.game = game;
                    }
                    // Load game from saved data
                    self::startwindow::START_DIALOG_RESULT_LOADGAME => {
                        unimplemented!();
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        }
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


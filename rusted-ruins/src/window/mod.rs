mod choose_window;
mod creation_window;
mod dialogreq;
mod equip_window;
mod exit_window;
mod game_info_window;
mod group_window;
mod indicator;
mod item_info_window;
mod item_window;
mod log_window;
mod main_window;
mod minimap;
mod misc_window;
mod msg_dialog;
mod newgame_window;
mod quest_window;
mod start_window;
mod status_window;
mod talk_window;
mod text_input_dialog;
mod text_window;
mod widget;
mod winpos;

use self::log_window::LogWindow;
use self::main_window::MainWindow;
use self::widget::WidgetTrait;
use crate::eventhandler::EventHandler;
use crate::game::{Command, DoPlayerAction, GameState, InfoGetter};
use crate::SdlContext;
use array2d::*;
use common::gamedata::*;
use sdl2::keyboard::TextInputUtil;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::any::Any;

mod commonuse {
    pub use crate::config::{SCREEN_CFG, UI_CFG};
    pub use crate::context::*;
    pub use crate::draw::border::draw_rect_border;
    pub use crate::eventhandler::InputMode;
    pub use crate::game::{Animation, Command, DoPlayerAction, Game};
    pub use crate::window::{DialogResult, DialogWindow, Window, WindowDrawMode};
    pub use sdl2::rect::Rect;
    pub use sdl2::render::WindowCanvas;
}

use self::commonuse::*;

pub enum DialogResult {
    Continue,
    Close,
    CloseWithValue(Box<dyn Any>),
    CloseAll,
    Quit,
    OpenChildDialog(Box<dyn DialogWindow>),
    Special(SpecialDialogResult),
}

pub enum SpecialDialogResult {
    StartDialogNewGame,
    StartDialogLoadGame,
    NewGameStart(GameData),
    ReturnToStartScreen,
}

pub trait Window {
    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>);
}

pub trait DialogWindow: Window {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult;
    /// Return InputMode for this window
    fn mode(&self) -> InputMode;
    fn callback_child_closed(
        &mut self,
        _result: Option<Box<dyn Any>>,
        _pa: &mut DoPlayerAction,
    ) -> DialogResult {
        DialogResult::Continue
    }
    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::Normal
    }
}

/// The current main mode
enum WindowManageMode {
    /// On start screen
    Start(self::start_window::StartWindow),
    /// Creating new game
    NewGame(self::newgame_window::NewGameWindow),
    /// Game playing
    OnGame(GameWindows),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WindowDrawMode {
    Normal,
    SkipUnderWindows,
}

impl WindowManageMode {
    pub fn is_on_game(&self) -> bool {
        match self {
            WindowManageMode::OnGame(_) => true,
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
    window_stack: Vec<Box<dyn DialogWindow>>,
    targeting_mode: bool,
}

impl<'sdl, 't> WindowManager<'sdl, 't> {
    pub fn new(
        sdl_context: &'sdl SdlContext,
        texture_creator: &'t TextureCreator<WindowContext>,
    ) -> WindowManager<'sdl, 't> {
        let game = Game::empty();
        let sdl_values = SdlValues::new(sdl_context, texture_creator);
        let mut window_stack: Vec<Box<dyn DialogWindow>> = Vec::new();
        window_stack.push(Box::new(start_window::StartDialog::new()));

        WindowManager {
            game,
            mode: WindowManageMode::Start(start_window::StartWindow::new()),
            sdl_values,
            text_input_util: sdl_context.sdl_context.video().unwrap().text_input(),
            anim: None,
            passed_frame: 0,
            window_stack,
            targeting_mode: false,
        }
    }

    // If return value is false, quit.
    pub fn advance_turn(&mut self, event_handler: &mut EventHandler) -> bool {
        // Animation must be finished before
        assert!(self.anim.is_none());

        if self.game.get_state() == GameState::WaitingForNextTurn && self.mode.is_on_game() {
            self.game.advance_turn();
        }

        // If game requests dialog popup for player
        if let Some(dialog_open_request) = self.game.pop_dialog_open_request() {
            let dialog = dialogreq::create_dialog_from_request(dialog_open_request, &mut self.game);
            if let Some(dialog) = dialog {
                self.window_stack.push(dialog);
            }
        }

        if self.game.get_state() == GameState::PlayerTurn {
            if !self.process_command(event_handler) {
                self.game.end_game();
                return false;
            }
        }

        // After advancing turn and processing command, game may start animation.
        self.anim = self.game.pop_animation();

        true
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
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
        let mut context = Context::new(canvas, &mut self.sdl_values);

        // Draw windows
        match self.mode {
            WindowManageMode::OnGame(ref mut game_windows) => {
                self.game.update_before_drawing();
                game_windows.draw(&mut context, &self.game, anim);
            }
            WindowManageMode::Start(ref mut start_window) => {
                start_window.draw(&mut context, &self.game, anim);
            }
            WindowManageMode::NewGame(ref mut newgame_window) => {
                newgame_window.draw(&mut context, &self.game, anim);
            }
        }

        // Draw dialog windows
        let mut windows_to_draw = Vec::new();
        for (i, w) in &mut self.window_stack.iter().enumerate() {
            match w.draw_mode() {
                WindowDrawMode::Normal => windows_to_draw.push(i),
                WindowDrawMode::SkipUnderWindows => {
                    windows_to_draw.clear();
                    windows_to_draw.push(i);
                }
            }
        }

        for i in &windows_to_draw {
            self.window_stack[*i].draw(&mut context, &self.game, anim);
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

        let mode = if !self.window_stack.is_empty() {
            self.window_stack[self.window_stack.len() - 1].mode()
        } else {
            if self.targeting_mode {
                InputMode::Targeting
            } else {
                InputMode::Normal
            }
        };

        let command = event_handler.get_command(mode);
        if command.is_none() {
            return true;
        }
        let command = command.unwrap();

        if self.targeting_mode {
            self.process_command_targeting_mode(command);
            return true;
        }

        use crate::game::playeract::DoPlayerAction;

        if !self.window_stack.is_empty() {
            let mut tail = self.window_stack.len() - 1;
            let mut dialog_result = {
                let mut pa = DoPlayerAction::new(&mut self.game);
                self.window_stack[tail].process_command(&command, &mut pa)
            };
            loop {
                match dialog_result {
                    DialogResult::Continue => (),
                    DialogResult::Close => {
                        self.window_stack.pop();
                        if tail > 0 {
                            tail -= 1;
                            let mut pa = DoPlayerAction::new(&mut self.game);
                            dialog_result =
                                self.window_stack[tail].callback_child_closed(None, &mut pa);
                            continue;
                        }
                    }
                    DialogResult::CloseWithValue(v) => {
                        self.window_stack.pop();
                        if tail > 0 {
                            tail -= 1;
                            let mut pa = DoPlayerAction::new(&mut self.game);
                            dialog_result =
                                self.window_stack[tail].callback_child_closed(Some(v), &mut pa);
                            continue;
                        }
                    }
                    DialogResult::CloseAll => {
                        self.window_stack.clear();
                    }
                    DialogResult::Quit => {
                        return false;
                    }
                    DialogResult::OpenChildDialog(child) => {
                        self.window_stack.push(child);
                    }
                    DialogResult::Special(result) => {
                        self.process_special_result(result);
                    }
                }
                return true;
            }
        }

        // If self.mode is OnGame
        let mut pa = DoPlayerAction::new(&mut self.game);
        use self::item_window::*;
        match command {
            Command::Move { dir } => {
                pa.try_move(dir);
            }
            Command::Enter => {
                // If player is on stairs, move from this map
                if pa.gd().on_map_entrance() {
                    pa.goto_next_floor(Direction::none());
                }
            }
            Command::Shot => {
                pa.shot();
            }
            Command::OpenCreationWin => {
                self.window_stack
                    .push(Box::new(creation_window::CreationWindow::new()));
            }
            Command::OpenExitWin => {
                self.window_stack
                    .push(Box::new(exit_window::ExitWindow::new()));
            }
            Command::OpenItemMenu => {
                self.window_stack
                    .push(Box::new(item_window::create_item_window_group(
                        pa.game(),
                        ItemWindowMode::List,
                    )));
            }
            Command::OpenEquipWin => {
                self.window_stack
                    .push(Box::new(equip_window::EquipWindow::new(
                        &mut pa,
                        CharaId::Player,
                    )));
            }
            Command::OpenStatusWin => {
                self.window_stack
                    .push(Box::new(status_window::create_status_window_group(
                        pa.game(),
                    )));
            }
            Command::OpenGameInfoWin => {
                self.window_stack
                    .push(Box::new(game_info_window::GameInfoWindow::new(pa.game())));
            }
            Command::PickUpItem => {
                if pa.gd().item_on_player_tile().is_some() {
                    let item_window = ItemWindow::new(ItemWindowMode::PickUp, pa.game());
                    self.window_stack.push(Box::new(item_window));
                }
            }
            Command::DropItem => {
                self.window_stack
                    .push(Box::new(item_window::create_item_window_group(
                        pa.game(),
                        ItemWindowMode::Drop,
                    )));
            }
            Command::DrinkItem => {
                self.window_stack
                    .push(Box::new(item_window::create_item_window_group(
                        pa.game(),
                        ItemWindowMode::Drink,
                    )));
            }
            Command::EatItem => {
                self.window_stack
                    .push(Box::new(item_window::create_item_window_group(
                        pa.game(),
                        ItemWindowMode::Eat,
                    )));
            }
            Command::TargetingMode => {
                self.targeting_mode = true;
                match self.mode {
                    WindowManageMode::OnGame(ref mut game_windows) => {
                        game_windows.main_window.start_targeting_mode(pa.game());
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        }
        true
    }

    pub fn process_special_result(&mut self, result: SpecialDialogResult) {
        match self.mode {
            WindowManageMode::Start(_) => {
                match result {
                    // Start new game
                    SpecialDialogResult::StartDialogNewGame => {
                        info!("Start newgame dialog");
                        self.window_stack.clear();
                        self.mode = WindowManageMode::NewGame(newgame_window::NewGameWindow::new());
                        self.window_stack
                            .push(Box::new(newgame_window::DummyNewGameDialog::new()));
                    }
                    // Load game from saved data
                    SpecialDialogResult::StartDialogLoadGame => {
                        self.window_stack
                            .push(Box::new(start_window::ChooseSaveFileDialog::new()));
                    }
                    // Load from file
                    SpecialDialogResult::NewGameStart(gd) => {
                        info!("Load game from file");
                        self.window_stack.clear();
                        self.mode = WindowManageMode::OnGame(GameWindows::new());

                        let game = Game::new(gd);
                        self.game = game;
                        self.game.update_before_player_turn();
                        game_log_i!("start"; version=env!("CARGO_PKG_VERSION"));
                    }
                    _ => unreachable!(),
                }
            }
            WindowManageMode::NewGame(_) => match result {
                SpecialDialogResult::NewGameStart(gd) => {
                    info!("Create newgame from dialog result");
                    self.window_stack.clear();
                    self.mode = WindowManageMode::OnGame(GameWindows::new());

                    let game = Game::new(gd);
                    self.game = game;
                    self.game.update_before_player_turn();
                    game_log_i!("start"; version=env!("CARGO_PKG_VERSION"));
                }
                _ => unreachable!(),
            },
            WindowManageMode::OnGame(_) => match result {
                SpecialDialogResult::ReturnToStartScreen => {
                    info!("Return to start screen");
                    crate::log::clear();
                    self.window_stack.clear();
                    self.window_stack
                        .push(Box::new(start_window::StartDialog::new()));
                    self.mode = WindowManageMode::Start(start_window::StartWindow::new());
                }
                _ => unreachable!(),
            },
        }
    }

    fn process_command_targeting_mode(&mut self, command: Command) {
        let main_window = match self.mode {
            WindowManageMode::OnGame(ref mut game_windows) => &mut game_windows.main_window,
            _ => unreachable!(),
        };

        match command {
            Command::Move { dir } => {
                main_window.move_centering_tile(dir, &self.game);
            }
            Command::Cancel => {
                self.targeting_mode = false;
                main_window.stop_targeting_mode();
            }
            Command::Enter => {
                // Set target
                let ct = main_window.get_current_centering_tile();
                if self.game.set_target(ct) {
                    self.targeting_mode = false;
                    main_window.stop_targeting_mode();
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
        TEXT_INPUT.with(|text_input| text_input.get())
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
    indicator_hp: indicator::BarIndicator,
    indicator_sp: indicator::BarIndicator,
    floor_info: indicator::FloorInfo,
    status_info: indicator::StatusInfo,
    time_info: indicator::TimeInfo,
    hborders: Vec<self::widget::HBorder>,
    vborders: Vec<self::widget::VBorder>,
}

impl GameWindows {
    fn new() -> GameWindows {
        use self::widget::{HBorder, VBorder};
        use crate::config::SCREEN_CFG;
        use indicator::*;
        let mut hborders = Vec::new();
        for hborder in &SCREEN_CFG.hborders {
            hborders.push(HBorder::new((hborder.x, hborder.y), hborder.len));
        }
        let mut vborders = Vec::new();
        for vborder in &SCREEN_CFG.vborders {
            vborders.push(VBorder::new((vborder.x, vborder.y), vborder.len));
        }

        GameWindows {
            main_window: MainWindow::new(),
            log_window: LogWindow::new(),
            minimap_window: minimap::MiniMapWindow::new(),
            indicator_hp: BarIndicator::new(BarIndicatorKind::Hp),
            indicator_sp: BarIndicator::new(BarIndicatorKind::Sp),
            floor_info: FloorInfo::new(),
            status_info: StatusInfo::new(),
            time_info: TimeInfo::new(),
            hborders,
            vborders,
        }
    }

    fn draw(&mut self, context: &mut Context, game: &Game, anim: Option<(&Animation, u32)>) {
        self.main_window.draw(context, game, anim);
        self.log_window.draw(context, game, anim);
        self.minimap_window.draw(context, game, anim);
        self.indicator_hp.draw(context, game, anim);
        self.indicator_sp.draw(context, game, anim);
        self.floor_info.draw(context, game, anim);
        self.status_info.draw(context, game, anim);
        self.time_info.draw(context, game, anim);

        for hborder in self.hborders.iter_mut() {
            hborder.draw(context);
        }
        for vborder in self.vborders.iter_mut() {
            vborder.draw(context);
        }
    }
}

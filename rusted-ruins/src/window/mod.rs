#[macro_use]
mod tools;
#[macro_use]
mod closer;

mod ability_window;
mod build_obj_dialog;
mod choose_window;
mod creation_window;
mod dialogreq;
mod equip_window;
mod exit_window;
mod faction_window;
mod game_info_window;
mod group_window;
mod help_window;
mod indicator;
mod item_info_window;
mod item_menu;
mod item_window;
mod list_desc_window;
mod log_window;
mod main_window;
mod minimap;
mod misc_window;
mod msg_dialog;
mod newgame_window;
mod progress_bar;
mod quest_window;
mod read_window;
mod register_shortcut_dialog;
mod sidebar;
mod slot_window;
mod start_window;
mod status_window;
mod talk_window;
mod text_input_dialog;
mod text_window;
mod tile_menu;
mod toolbar;
mod widget;
mod winpos;

use self::log_window::LogWindow;
use self::main_window::MainWindow;
use self::widget::WidgetTrait;
use crate::eventhandler::EventHandler;
use crate::game::extrait::PlayTimeExt;
use crate::game::{Command, DoPlayerAction, GameState, InfoGetter, UiRequest};
use crate::SdlContext;
use common::gamedata::*;
use geom::*;
use script::ScriptEngine;
use sdl2::keyboard::TextInputUtil;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

mod commonuse {
    pub use crate::config::{SCREEN_CFG, UI_CFG};
    pub use crate::context::*;
    pub use crate::draw::border::draw_window_border;
    pub use crate::eventhandler::InputMode;
    pub use crate::game::{Animation, Command, DoPlayerAction, Game, InfoGetter};
    pub use crate::window::closer::DialogCloser;
    pub use crate::window::widget::WidgetTrait;
    pub use crate::window::winpos::WindowPos;
    pub use crate::window::{
        DialogCloseValue, DialogResult, DialogWindow, SpecialDialogResult, Window, WindowDrawMode,
    };
    pub use sdl2::rect::Rect;
    pub use sdl2::render::WindowCanvas;
}

use self::commonuse::*;

pub enum DialogResult {
    Continue,
    Close,
    CloseWithValue(DialogCloseValue),
    CloseAll,
    CloseAllAndReprocess(Command),
    CloseAndOpen(Box<dyn DialogWindow>),
    Command(Option<Command>),
    Quit,
    OpenChildDialog(Box<dyn DialogWindow>),
    Reprocess(Command),
    Special(SpecialDialogResult),
}

impl std::fmt::Debug for DialogResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogResult::Continue => f.write_str("Continue"),
            DialogResult::Close => f.write_str("Close"),
            DialogResult::CloseWithValue(v) => f.write_fmt(format_args!("CloseWithValue({:?})", v)),
            DialogResult::CloseAll => f.write_str("CloseAll"),
            DialogResult::CloseAllAndReprocess(c) => {
                f.write_fmt(format_args!("CloseAllAndReprocess({:?})", c))
            }
            DialogResult::CloseAndOpen(_) => f.write_str("CloseAndOpen"),
            DialogResult::Command(c) => f.write_fmt(format_args!("Command({:?})", c)),
            DialogResult::Quit => f.write_str("Quit"),
            DialogResult::OpenChildDialog(_) => f.write_str("OpenChildDialog"),
            DialogResult::Reprocess(c) => f.write_fmt(format_args!("Reprocess({:?})", c)),
            DialogResult::Special(_) => f.write_str("Special"),
        }
    }
}

impl DialogResult {
    fn is_close(&self) -> bool {
        matches!(
            self,
            DialogResult::Close
                | DialogResult::CloseAll
                | DialogResult::CloseAllAndReprocess(_)
                | DialogResult::CloseAndOpen(_)
                | DialogResult::CloseWithValue(_)
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DialogCloseValue {
    Index(u32),
    _Dummy,
}

pub enum SpecialDialogResult {
    StartDialogNewGame,
    StartDialogLoadGame,
    TempGameData(Box<GameData>),
    NewGameStart(Box<GameData>),
    ReturnToStartScreen,
    ItemListUpdate,
}

pub trait Window {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    );
}

pub trait DialogWindow: Window {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult;

    /// Return InputMode for this window
    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn callback_child_closed(
        &mut self,
        _result: Option<DialogCloseValue>,
        _pa: &mut DoPlayerAction<'_>,
    ) -> DialogResult {
        DialogResult::Continue
    }

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::Normal
    }

    fn update(&mut self, _gd: &GameData) {}

    fn sound(&self, open: bool) {
        if open {
            audio::play_sound("window-open");
        } else {
            audio::play_sound("window-close");
        }
    }

    fn mainwin_cursor(&self) -> bool {
        false
    }

    fn tab_switched(&mut self) {}
}

/// The current main mode
#[allow(clippy::large_enum_variant)]
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
        matches!(self, WindowManageMode::OnGame(_))
    }

    pub fn update_game_windows_per_advance_turn(&mut self, game: &Game) {
        if let WindowManageMode::OnGame(GameWindows {
            supplement_info, ..
        }) = self
        {
            supplement_info.update(game);
        }
    }
}

/// Manage all windows
pub struct WindowManager<'sdl, 't> {
    pub game: Game,
    mode: WindowManageMode,
    sdl_values: SdlValues<'sdl, 't>,
    text_input_util: TextInputUtil,
    anim: Option<Animation>,
    passed_frame: u32,
    window_stack: Vec<Box<dyn DialogWindow>>,
}

impl<'sdl, 't> WindowManager<'sdl, 't> {
    pub fn new(
        sdl_context: &'sdl SdlContext,
        texture_creator: &'t TextureCreator<WindowContext>,
        se: ScriptEngine,
    ) -> WindowManager<'sdl, 't> {
        let game = Game::empty(se);
        let sdl_values = SdlValues::new(sdl_context, texture_creator);
        let window_stack: Vec<Box<dyn DialogWindow>> =
            vec![Box::new(start_window::StartDialog::new())];

        WindowManager {
            game,
            mode: WindowManageMode::Start(start_window::StartWindow::new()),
            sdl_values,
            text_input_util: sdl_context.sdl_context.video().unwrap().text_input(),
            anim: None,
            passed_frame: 0,
            window_stack,
        }
    }

    // If return value is false, quit.
    pub fn advance_turn(&mut self, event_handler: &mut EventHandler) -> bool {
        // Animation must be finished before
        assert!(self.anim.is_none());

        if self.game.get_state() == GameState::WaitingForNextTurn && self.mode.is_on_game() {
            self.game.advance_turn();
            self.mode.update_game_windows_per_advance_turn(&self.game);
        }

        // Process ui requests
        self.process_ui_request();

        // If game requests dialog popup for player
        if let Some(dialog_open_request) = self.game.pop_dialog_open_request() {
            let dialog = dialogreq::create_dialog_from_request(dialog_open_request, &mut self.game);
            if let Some(dialog) = dialog {
                self.push_dialog_window(dialog);
            }
        }

        if self.game.get_state() == GameState::PlayerTurn && !self.process_command(event_handler) {
            self.game.end_game();
            return false;
        }

        // After advancing turn and processing command, game may start animation.
        self.anim = self.game.pop_animation();

        true
    }

    pub fn update_cursor(&mut self, pos: (i32, i32)) {
        if let WindowManageMode::OnGame(ref mut game_windows) = self.mode {
            let tile = game_windows.main_window.update_tile_cursor(pos);
            if let Some(tile) = tile {
                game_windows
                    .supplement_info
                    .update_hover_tile(&self.game, tile);
            }
        }
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
                game_windows.covered = !self.window_stack.is_empty();
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

        // Advance animation frame
        if anim.is_some() {
            self.passed_frame += 1;
        }
        crate::damage_popup::advance_frame();
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
            InputMode::Normal
        };

        let command = event_handler.get_command(mode);
        if command.is_none() {
            return true;
        }
        let command = command.unwrap();

        let command = if !self.window_stack.is_empty() {
            let mut tail = self.window_stack.len() - 1;
            let mut dialog_result = {
                let mut pa = DoPlayerAction::new(&mut self.game);
                self.window_stack[tail].process_command(&command, &mut pa)
            };
            loop {
                match dialog_result {
                    DialogResult::Continue => (),
                    DialogResult::Close => {
                        if let Some(w) = self.window_stack.pop() {
                            w.sound(false);
                        }

                        if tail > 0 {
                            tail -= 1;
                            let mut pa = DoPlayerAction::new(&mut self.game);
                            dialog_result =
                                self.window_stack[tail].callback_child_closed(None, &mut pa);
                            continue;
                        }
                    }
                    DialogResult::CloseWithValue(v) => {
                        if let Some(w) = self.window_stack.pop() {
                            w.sound(false);
                        }

                        if tail > 0 {
                            tail -= 1;
                            let mut pa = DoPlayerAction::new(&mut self.game);
                            dialog_result =
                                self.window_stack[tail].callback_child_closed(Some(v), &mut pa);
                            continue;
                        }
                    }
                    DialogResult::CloseAllAndReprocess(command) => {
                        if let Some(w) = self.window_stack.pop() {
                            w.sound(false);
                        }
                        self.window_stack.clear();
                        break command;
                    }
                    DialogResult::CloseAll => {
                        if let Some(w) = self.window_stack.pop() {
                            w.sound(false);
                        }
                        self.window_stack.clear();
                    }
                    DialogResult::CloseAndOpen(win) => {
                        self.window_stack.clear();
                        self.push_dialog_window(win);
                    }
                    DialogResult::Command(_) => (),
                    DialogResult::Quit => {
                        return false;
                    }
                    DialogResult::OpenChildDialog(child) => {
                        self.push_dialog_window(child);
                    }
                    DialogResult::Reprocess(command) => {
                        break command;
                    }
                    DialogResult::Special(result) => {
                        self.process_special_result(result);
                    }
                }
                return true;
            }
        } else {
            command
        };

        // If self.mode is OnGame
        let command = match &mut self.mode {
            WindowManageMode::OnGame(game_windows) => {
                let command = match game_windows
                    .sidebar
                    .process_command(&command, &mut DoPlayerAction::new(&mut self.game))
                {
                    DialogResult::Command(command) => {
                        if let Some(command) = command {
                            command
                        } else {
                            return true;
                        }
                    }
                    _ => command,
                };

                let command = match game_windows
                    .toolbar
                    .process_command(&command, &mut DoPlayerAction::new(&mut self.game))
                {
                    DialogResult::Command(command) => {
                        if let Some(command) = command {
                            command
                        } else {
                            return true;
                        }
                    }
                    DialogResult::OpenChildDialog(child) => {
                        self.push_dialog_window(child);
                        return true;
                    }
                    _ => command,
                };

                let command = match game_windows
                    .shortcut_list
                    .process_command(&command, &mut DoPlayerAction::new(&mut self.game))
                {
                    DialogResult::Command(command) => {
                        if let Some(command) = command {
                            command
                        } else {
                            return true;
                        }
                    }
                    _ => command,
                };

                match game_windows
                    .main_window
                    .convert_mouse_event(command, &self.game)
                {
                    main_window::ConvertMouseEventResult::Command(command) => command,
                    main_window::ConvertMouseEventResult::OpenWindow(window) => {
                        self.push_dialog_window(window);
                        return true;
                    }
                    main_window::ConvertMouseEventResult::DoAction(callback) => {
                        let mut pa = DoPlayerAction::new(&mut self.game);
                        callback(&mut pa);
                        return true;
                    }
                    main_window::ConvertMouseEventResult::None => {
                        return true;
                    }
                }
            }
            _ => unreachable!(),
        };

        let mut pa = DoPlayerAction::new(&mut self.game);

        use self::item_window::*;
        match command {
            Command::Move { dir } => {
                pa.try_move(dir);
            }
            Command::MoveTo { dest } => {
                pa.move_to(dest);
            }
            Command::Shoot { target } => {
                pa.shoot(target);
            }
            Command::UseTool { target } => {
                pa.use_tool(target);
            }
            Command::Enter => {
                // If player is on stairs, move from this map
                if pa.gd().on_map_entrance() {
                    pa.goto_next_floor(Direction::none(), true);
                }
            }
            Command::OpenAbilityWin => {
                let dialog = Box::new(ability_window::AbilityWindow::new(pa.gd(), CharaId::Player));
                self.push_dialog_window(dialog);
            }
            Command::OpenCreationWin => {
                let dialog = Box::new(creation_window::create_creation_window_group(
                    pa.game(),
                    None,
                ));
                self.push_dialog_window(dialog);
            }
            Command::OpenExitWin => {
                self.push_dialog_window(Box::new(exit_window::ExitWindow::new()));
            }
            Command::OpenHelpWin => {
                self.push_dialog_window(Box::new(help_window::HelpWindow::new()));
            }
            Command::OpenItemWin => {
                let dialog = Box::new(item_window::create_item_window_group(pa.game(), None));
                self.push_dialog_window(dialog);
            }
            Command::OpenDebugCommandWin => {
                let mut win = text_input_dialog::TextInputDialog::new();
                win.set_callback(|pa, s| {
                    pa.exec_debug_command(s);
                });
                self.push_dialog_window(Box::new(win));
            }
            Command::OpenEquipWin => {
                let dialog = Box::new(equip_window::EquipWindow::new(
                    pa.gd(),
                    CharaId::Player,
                    true,
                ));
                self.push_dialog_window(dialog);
            }
            Command::OpenStatusWin => {
                let dialog = Box::new(status_window::create_status_window_group(
                    pa.game(),
                    CharaId::Player,
                    true,
                ));
                self.push_dialog_window(dialog);
            }
            Command::OpenGameInfoWin => {
                let dialog = Box::new(game_info_window::create_game_info_window_group(&mut pa));
                self.push_dialog_window(dialog);
            }
            Command::PickUpItem => {
                if !pa.gd().item_on_player_tile().is_empty() {
                    let item_window = ItemWindow::new(ItemWindowMode::PickUp, pa.game());
                    self.push_dialog_window(Box::new(item_window));
                }
            }
            Command::DropItem => {
                let dialog = Box::new(item_window::create_item_window_group(
                    pa.game(),
                    Some(ItemWindowMode::Drop),
                ));
                self.push_dialog_window(dialog);
            }
            Command::DrinkItem => {
                let dialog = Box::new(item_window::create_item_window_group(
                    pa.game(),
                    Some(ItemWindowMode::Drink),
                ));
                self.push_dialog_window(dialog);
            }
            Command::EatItem => {
                let dialog = Box::new(item_window::create_item_window_group(
                    pa.game(),
                    Some(ItemWindowMode::Eat),
                ));
                self.push_dialog_window(dialog);
            }
            Command::ReleaseItem => {
                let dialog = Box::new(item_window::create_item_window_group(
                    pa.game(),
                    Some(ItemWindowMode::Release),
                ));
                self.push_dialog_window(dialog);
            }
            Command::ActionShortcut(n) => {
                pa.exec_shortcut(n);
            }
            Command::ChangeEquip { kind } => {
                let dialog = Box::new(item_window::ItemWindow::new_select_and_equip(
                    CharaId::Player,
                    (kind, 0),
                    &mut pa,
                ));
                self.push_dialog_window(dialog);
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
                        self.push_dialog_window(
                            Box::new(newgame_window::DummyNewGameDialog::new()),
                        );
                    }
                    // Load game from saved data
                    SpecialDialogResult::StartDialogLoadGame => {
                        self.push_dialog_window(
                            Box::new(start_window::ChooseSaveFileDialog::new()),
                        );
                    }
                    // Load from file
                    SpecialDialogResult::NewGameStart(mut gd) => {
                        info!("Load game from file");
                        gd.play_time.start();
                        self.window_stack.clear();
                        self.mode = WindowManageMode::OnGame(GameWindows::new());

                        let game = Game::new(*gd, self.game.se.clone());
                        self.game = game;
                        self.game.update_before_player_turn();
                        game_log!("start"; version=env!("CARGO_PKG_VERSION"));
                        audio::play_music(&self.game.gd.get_current_map().music);
                    }
                    _ => unreachable!(),
                }
            }
            WindowManageMode::NewGame(_) => match result {
                SpecialDialogResult::NewGameStart(gd) => {
                    info!("Create newgame from dialog result");
                    self.window_stack.clear();
                    self.mode = WindowManageMode::OnGame(GameWindows::new());

                    let game = Game::new(*gd, self.game.se.clone());
                    self.game = game;
                    self.game.update_before_player_turn();
                    self.game.start_new_game();
                    game_log!("start"; version=env!("CARGO_PKG_VERSION"));
                }
                SpecialDialogResult::TempGameData(gd) => {
                    self.game.gd = *gd;
                    info!("Temporary GameData");
                }
                SpecialDialogResult::ReturnToStartScreen => {
                    self.window_stack.clear();
                    self.push_dialog_window(Box::new(start_window::StartDialog::new()));
                    self.mode = WindowManageMode::Start(start_window::StartWindow::new());
                }
                _ => unreachable!(),
            },
            WindowManageMode::OnGame(_) => match result {
                SpecialDialogResult::ReturnToStartScreen => {
                    info!("Return to start screen");
                    crate::log::clear();
                    self.window_stack.clear();
                    self.push_dialog_window(Box::new(start_window::StartDialog::new()));
                    self.mode = WindowManageMode::Start(start_window::StartWindow::new());
                }
                _ => unreachable!(),
            },
        }
    }

    fn push_dialog_window(&mut self, w: Box<dyn DialogWindow>) {
        w.sound(true);
        if !w.mainwin_cursor() {
            if let WindowManageMode::OnGame(windows) = &mut self.mode {
                windows.main_window.reset_tile_cursor();
            }
        }
        crate::eventhandler::open_dialog();
        self.window_stack.push(w);
    }

    fn process_ui_request(&mut self) {
        while let Some(req) = self.game.pop_ui_request() {
            match req {
                UiRequest::StopCentering => {
                    if let WindowManageMode::OnGame(ref mut windows) = self.mode {
                        windows.main_window.stop_centering_mode();
                    }
                }
                UiRequest::StartTargeting { effect, callback } => {
                    if let WindowManageMode::OnGame(ref mut windows) = self.mode {
                        windows
                            .main_window
                            .start_targeting_mode(&self.game, effect, callback);
                    }
                }
            }
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
    sidebar: sidebar::Sidebar,
    toolbar: toolbar::Toolbar,
    shortcut_list: toolbar::ShortcutList,
    indicator_hp: indicator::BarIndicator,
    indicator_mp: indicator::BarIndicator,
    indicator_sp: indicator::BarIndicator,
    floor_info: indicator::FloorInfo,
    status_info: indicator::StatusInfo,
    supplement_info: indicator::SupplementInfo,
    time_info: indicator::TimeInfo,
    hborders: Vec<self::widget::HBorder>,
    vborders: Vec<self::widget::VBorder>,
    progress_bar: progress_bar::ProgressBar,
    covered: bool,
}

impl GameWindows {
    fn new() -> GameWindows {
        use self::widget::{HBorder, VBorder};
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
            sidebar: sidebar::Sidebar::new(),
            toolbar: toolbar::Toolbar::new(),
            shortcut_list: toolbar::ShortcutList::new(),
            indicator_hp: BarIndicator::new(BarIndicatorKind::Hp),
            indicator_mp: BarIndicator::new(BarIndicatorKind::Mp),
            indicator_sp: BarIndicator::new(BarIndicatorKind::Sp),
            floor_info: FloorInfo::new(),
            status_info: StatusInfo::new(),
            supplement_info: SupplementInfo::default(),
            time_info: TimeInfo::new(),
            hborders,
            vborders,
            progress_bar: progress_bar::ProgressBar::new(),
            covered: false,
        }
    }

    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        for hborder in self.hborders.iter_mut() {
            hborder.draw(context);
        }
        for vborder in self.vborders.iter_mut() {
            vborder.draw(context);
        }
        self.main_window.draw(context, game, anim);
        self.log_window.draw(context, game, anim);
        self.minimap_window.draw(context, game, anim);
        self.sidebar.draw(context, game, anim);
        self.toolbar.draw(context, game, anim);
        self.shortcut_list.draw(context, game, anim);
        self.indicator_hp.draw(context, game, anim);
        self.indicator_mp.draw(context, game, anim);
        self.indicator_sp.draw(context, game, anim);
        self.floor_info.draw(context, game, anim);
        self.status_info.draw(context, game, anim);
        if !self.covered {
            self.supplement_info.draw(context, game, anim);
        }
        self.time_info.draw(context, game, anim);
        self.progress_bar.draw(context, game, anim);
    }
}


mod mainwindow;
mod logwindow;
mod textwindow;
mod itemwindow;
mod exitwindow;
mod yesnodialog;
mod widget;

use game::{GameState, DoPlayerAction};
use eventhandler::EventHandler;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use SdlContext;
use config::SCREEN_CFG;
use self::mainwindow::MainWindow;
use self::logwindow::LogWindow;

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

pub struct WindowManager<'sdl, 't> {
    game: Game,
    sdl_values: SdlValues<'sdl, 't>,
    main_window: MainWindow,
    log_window: LogWindow,
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
        
        WindowManager {
            game: game,
            sdl_values: sdl_values,
            main_window: MainWindow::new(SCREEN_CFG.main_window.into()),
            log_window:  LogWindow ::new(SCREEN_CFG.log_window.into()),
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
        
        self.main_window.redraw(canvas, &self.game, &mut self.sdl_values, anim);
        self.log_window.redraw(canvas, &self.game, &mut self.sdl_values, anim);

        for mut w in &mut self.window_stack {
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
                self.window_stack.push(Box::new(yesnodialog::YesNoDialog::new(
                    ::text::ui_txt("dialog.move_floor"),
                    |pa| {
                        pa.move_next_floor();
                        DialogResult::Close
                    }
                )));
            },
            Command::OpenExitWin => {
                self.window_stack.push(Box::new(exitwindow::ExitWindow::new()));
            },
            Command::OpenItemMenu => {
                self.window_stack.push(Box::new(itemwindow::ItemWindow::new()));
            },
            _ => (),
        }
        true
    }
}



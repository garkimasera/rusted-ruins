use crate::config::{INPUT_CFG, UI_CFG};
use crate::game::command::KeyState;
use crate::game::Command;
use geom::*;

use sdl2::event::Event;
use sdl2::joystick::Joystick;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

/// Convert from SDL Event to Command
pub struct EventHandler {
    _joystick_subsystem: sdl2::JoystickSubsystem,
    joystick: Option<Joystick>,
    command_queue: VecDeque<RawCommand>,
    conv_table: CommandConvTable,
    hdir: HDirection,
    vdir: VDirection,
    last_dir_changed: Option<Instant>,
    is_instant: bool,
    prev_input_mode: InputMode,
    waiting_dir_release: WaitingDirRelease,
    mouse_state: Option<MouseState>,
    key_state: KeyState,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputMode {
    Normal,
    Dialog,
    TextInput,
}

/// Used to prevent unintentional cursor moving after dialog opening
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WaitingDirRelease {
    No,
    Waiting,
    Skip,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RawCommand {
    KeyPress(Keycode),
    TextInput(String),
    MouseButtonDown {
        x: i32,
        y: i32,
        mouse_btn: MouseButton,
        key_state: KeyState,
    },
    MouseButtonUp {
        x: i32,
        y: i32,
        mouse_btn: MouseButton,
        key_state: KeyState,
    },
    MouseWheel {
        x: i32,
        y: i32,
    },
}

impl EventHandler {
    pub fn new(sdl_context: &sdl2::Sdl) -> EventHandler {
        let joystick_subsystem = sdl_context
            .joystick()
            .expect("Joysticksubsystem Initialization Failed.");
        let num_joysticks = joystick_subsystem.num_joysticks().unwrap_or(0);
        let joystick = if num_joysticks > 0 {
            match joystick_subsystem.open(0) {
                Ok(joystick) => {
                    info!("Opened Joystick \"{}\"", joystick.name());
                    Some(joystick)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        EventHandler {
            _joystick_subsystem: joystick_subsystem,
            joystick,
            command_queue: VecDeque::new(),
            conv_table: CommandConvTable::new(),
            hdir: HDirection::None,
            vdir: VDirection::None,
            last_dir_changed: None,
            is_instant: false,
            prev_input_mode: InputMode::Dialog,
            waiting_dir_release: WaitingDirRelease::No,
            mouse_state: None,
            key_state: KeyState::default(),
        }
    }

    pub fn process_event(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. } => {
                return false;
            }
            // Direction
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            // WASD Direction
            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } if INPUT_CFG.wasd_mode => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } if INPUT_CFG.wasd_mode => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } if INPUT_CFG.wasd_mode => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } if INPUT_CFG.wasd_mode => {
                self.set_waiting_dir_release();
            }
            // Ctrl & Shift keys
            Event::KeyDown {
                keycode: Some(Keycode::LCtrl),
                ..
            } => {
                self.key_state.ctrl = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::LCtrl),
                ..
            } => {
                self.key_state.ctrl = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                self.key_state.shift = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::LShift),
                ..
            } => {
                self.key_state.shift = false;
            }
            // Shortcut keys
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                self.command_queue.push_back(RawCommand::KeyPress(keycode));
            }
            // Joystick events
            Event::JoyButtonDown { button_idx, .. } => {
                trace!("ButtonDown: {}", button_idx);
            }
            Event::JoyAxisMotion { .. } => {
                self.set_waiting_dir_release();
            }
            // Text input events
            Event::TextInput { text, .. } => {
                self.command_queue.push_back(RawCommand::TextInput(text));
            }
            // Mouse events
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if let Some(command) = dialog_command(RawCommand::MouseButtonDown {
                    x,
                    y,
                    mouse_btn,
                    key_state: self.key_state,
                }) {
                    self.command_queue.push_back(command);
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if let Some(command) = dialog_command(RawCommand::MouseButtonUp {
                    x,
                    y,
                    mouse_btn,
                    key_state: self.key_state,
                }) {
                    self.command_queue.push_back(command);
                }
            }
            Event::MouseWheel {
                direction, x, y, ..
            } => {
                if direction == sdl2::mouse::MouseWheelDirection::Flipped {
                    self.command_queue
                        .push_back(RawCommand::MouseWheel { x: -x, y: -y });
                } else {
                    self.command_queue
                        .push_back(RawCommand::MouseWheel { x, y });
                }
            }
            _ => {}
        }
        true
    }

    pub fn get_command(&mut self, mode: InputMode) -> Option<Command> {
        // If input mode switched normal, cursor shouldn't move until direction key released once
        if mode == InputMode::Dialog && self.prev_input_mode == InputMode::Normal {
            self.waiting_dir_release = WaitingDirRelease::Waiting;
        }
        self.prev_input_mode = mode;

        while let Some(rawc) = self.command_queue.pop_front() {
            if let Some(command) = self.conv_table.conv(rawc, mode) {
                return Some(command);
            }
        }

        if mode == InputMode::Dialog {
            match self.waiting_dir_release {
                WaitingDirRelease::No => (),
                WaitingDirRelease::Waiting => {
                    if self.hdir == HDirection::None && self.vdir == VDirection::None {
                        self.waiting_dir_release = WaitingDirRelease::No;
                    }
                    return None;
                }
                WaitingDirRelease::Skip => {
                    self.waiting_dir_release = WaitingDirRelease::No;
                    return None;
                }
            }
        }
        if self.hdir != HDirection::None || self.vdir != VDirection::None {
            let c = Command::Move {
                dir: Direction::new(self.hdir, self.vdir),
            };

            // Slow down cursor moving speed in dialog mode
            if mode != InputMode::Normal {
                let now = Instant::now();
                let d = now.duration_since(self.last_dir_changed.unwrap());
                let d = d.as_secs() * 1000 + d.subsec_nanos() as u64 / 1_000_000; // to milli secs

                if !self.is_instant && d < UI_CFG.cursor_move_duration {
                    return None;
                }

                if !self.is_instant {
                    self.last_dir_changed = Some(now);
                }
                self.is_instant = false;
            }
            return Some(c);
        }

        // returns mouse state
        if let Some(mouse_state) = self.mouse_state {
            return Some(Command::MouseState {
                x: mouse_state.x(),
                y: mouse_state.y(),
                left_button: mouse_state.left(),
                right_button: mouse_state.right(),
                key_state: self.key_state,
                ui_only: false,
            });
        }

        None
    }

    /// Update event handler at each frame. Handles direction input and mouse cursor.
    pub fn update(&mut self, event_pump: &sdl2::EventPump) {
        // Mouse cursor
        self.mouse_state = Some(MouseState::new(event_pump));

        // Direction
        let keyboard = sdl2::keyboard::KeyboardState::new(event_pump);
        let mut hdir = HDirection::None;
        let mut vdir = VDirection::None;

        for scancode in keyboard.pressed_scancodes() {
            use sdl2::keyboard::Scancode;
            match scancode {
                Scancode::Up => {
                    vdir = VDirection::Up;
                }
                Scancode::Down => {
                    vdir = VDirection::Down;
                }
                Scancode::Left => {
                    hdir = HDirection::Left;
                }
                Scancode::Right => {
                    hdir = HDirection::Right;
                }
                Scancode::W if INPUT_CFG.wasd_mode => {
                    vdir = VDirection::Up;
                }
                Scancode::A if INPUT_CFG.wasd_mode => {
                    hdir = HDirection::Left;
                }
                Scancode::S if INPUT_CFG.wasd_mode => {
                    vdir = VDirection::Down;
                }
                Scancode::D if INPUT_CFG.wasd_mode => {
                    hdir = HDirection::Right;
                }
                _ => (),
            }
        }

        if let Some(ref joystick) = self.joystick {
            if let Ok(pos) = joystick.axis(0) {
                if pos < 0 {
                    hdir = HDirection::Left;
                } else if pos > 0 {
                    hdir = HDirection::Right;
                }
            }
            if let Ok(pos) = joystick.axis(1) {
                if pos < 0 {
                    vdir = VDirection::Up;
                } else if pos > 0 {
                    vdir = VDirection::Down;
                }
            }
        }

        let mut need_time_update = false;
        if hdir != self.hdir {
            if hdir != HDirection::None {
                need_time_update = true;
            }
            self.hdir = hdir;
        }
        if vdir != self.vdir {
            if vdir != VDirection::None {
                need_time_update = true;
            }
            self.vdir = vdir;
        }
        if need_time_update {
            self.is_instant = true;
            self.last_dir_changed = Some(Instant::now());
        }
    }

    fn set_waiting_dir_release(&mut self) {
        if self.waiting_dir_release == WaitingDirRelease::Waiting {
            self.waiting_dir_release = WaitingDirRelease::Skip;
        }
    }
}

pub struct CommandConvTable {
    normal: HashMap<RawCommand, Command>,
    dialog: HashMap<RawCommand, Command>,
}

impl CommandConvTable {
    fn new() -> CommandConvTable {
        let mut normal = HashMap::new();
        let mut dialog = HashMap::new();

        for (k, v) in INPUT_CFG.normal.iter() {
            let k = conv_str_to_keycode(k);

            normal.insert(RawCommand::KeyPress(k), v.clone());
        }

        for (k, v) in INPUT_CFG.dialog.iter() {
            let k = conv_str_to_keycode(k);

            dialog.insert(RawCommand::KeyPress(k), v.clone());
        }

        CommandConvTable { normal, dialog }
    }

    fn conv(&self, raw: RawCommand, mode: InputMode) -> Option<Command> {
        let table = match mode {
            InputMode::Normal => &self.normal,
            InputMode::Dialog => &self.dialog,
            InputMode::TextInput => {
                return text_input_conv(raw);
            }
        };

        // For mouse event, don't use table
        match raw {
            RawCommand::MouseButtonDown {
                x,
                y,
                mouse_btn,
                key_state,
            } => {
                let button = match mouse_btn {
                    MouseButton::Left => crate::game::command::MouseButton::Left,
                    MouseButton::Right => crate::game::command::MouseButton::Right,
                    MouseButton::Middle => crate::game::command::MouseButton::Middle,
                    _ => {
                        return None;
                    }
                };
                return Some(Command::MouseButtonDown {
                    x,
                    y,
                    button,
                    key_state,
                });
            }
            RawCommand::MouseButtonUp {
                x,
                y,
                mouse_btn,
                key_state,
            } => {
                let button = match mouse_btn {
                    MouseButton::Left => crate::game::command::MouseButton::Left,
                    MouseButton::Right => crate::game::command::MouseButton::Right,
                    MouseButton::Middle => crate::game::command::MouseButton::Middle,
                    _ => {
                        return None;
                    }
                };
                return Some(Command::MouseButtonUp {
                    x,
                    y,
                    button,
                    key_state,
                });
            }
            RawCommand::MouseWheel { x, y } => {
                return Some(Command::MouseWheel { x, y });
            }
            // Shortcuts by number key
            RawCommand::KeyPress(Keycode::Num1) => {
                return Some(Command::ActionShortcut(0));
            }
            RawCommand::KeyPress(Keycode::Num2) => {
                return Some(Command::ActionShortcut(1));
            }
            RawCommand::KeyPress(Keycode::Num3) => {
                return Some(Command::ActionShortcut(2));
            }
            RawCommand::KeyPress(Keycode::Num4) => {
                return Some(Command::ActionShortcut(3));
            }
            RawCommand::KeyPress(Keycode::Num5) => {
                return Some(Command::ActionShortcut(4));
            }
            RawCommand::KeyPress(Keycode::Num6) => {
                return Some(Command::ActionShortcut(5));
            }
            RawCommand::KeyPress(Keycode::Num7) => {
                return Some(Command::ActionShortcut(6));
            }
            RawCommand::KeyPress(Keycode::Num8) => {
                return Some(Command::ActionShortcut(7));
            }
            RawCommand::KeyPress(Keycode::Num9) => {
                return Some(Command::ActionShortcut(8));
            }
            RawCommand::KeyPress(Keycode::Num0) => {
                return Some(Command::ActionShortcut(9));
            }
            _ => (),
        }

        // Conversion by table
        table.get(&raw).cloned()
    }
}

/// In text input mode, all event is ignored except for text input or finish key press
fn text_input_conv(raw: RawCommand) -> Option<Command> {
    match raw {
        RawCommand::TextInput(text) => Some(Command::TextInput { text }),
        RawCommand::KeyPress(keycode) if keycode == Keycode::Return => Some(Command::Enter),
        RawCommand::KeyPress(keycode) if keycode == Keycode::Escape => Some(Command::Cancel),
        RawCommand::KeyPress(keycode)
            if keycode == Keycode::Backspace || keycode == Keycode::Delete =>
        {
            Some(Command::TextDelete)
        }
        _ => None,
    }
}

/// Convert strings of input configfile to keycodes
macro_rules! impl_conv_str_to_keycode {
    ($($m:ident),*) => {
        fn conv_str_to_keycode(s: &str) -> Keycode {
            $(
                if s.eq_ignore_ascii_case(stringify!($m)) {
                    return Keycode::$m;
                }
            )*
                panic!("Invalid keycode field : \"{}\"", s);
        }
    }
}

impl_conv_str_to_keycode!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Space, Return,
    Tab, Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12
);

thread_local!(static LEFT_BTN_DOWNED: Cell<bool> = Cell::new(false));

/// Call this when open new dialog.
pub fn open_dialog() {
    LEFT_BTN_DOWNED.with(|left_btn_downed| {
        left_btn_downed.set(false);
    });
}

/// Ignores button command event if button has not downed since last dialog open.
pub fn dialog_command(command: RawCommand) -> Option<RawCommand> {
    LEFT_BTN_DOWNED.with(|left_btn_downed| match command {
        RawCommand::MouseButtonUp { mouse_btn, .. } => {
            if mouse_btn == MouseButton::Left {
                if left_btn_downed.get() {
                    Some(command)
                } else {
                    None
                }
            } else {
                Some(command)
            }
        }
        RawCommand::MouseButtonDown { mouse_btn, .. } => {
            if mouse_btn == MouseButton::Left {
                left_btn_downed.set(true);
            }
            Some(command)
        }
        _ => Some(command),
    })
}

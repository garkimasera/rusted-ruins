use crate::config::{INPUT_CFG, UI_CFG};
use crate::game::Command;
use geom::*;
use sdl2;
use sdl2::event::Event;
use sdl2::joystick::Joystick;
use sdl2::keyboard::Keycode;
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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputMode {
    Normal,
    Dialog,
    Targeting,
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
        }
    }

    pub fn process_event(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. } => {
                return false;
            }
            // Direction
            Event::KeyDown {
                keycode: Option::Some(Keycode::Up),
                ..
            } => {
                //self.update_dir(None, Some(VDirection::Up));
            }
            Event::KeyDown {
                keycode: Option::Some(Keycode::Down),
                ..
            } => {
                //self.update_dir(None, Some(VDirection::Down));
            }
            Event::KeyDown {
                keycode: Option::Some(Keycode::Left),
                ..
            } => {
                //self.update_dir(Some(HDirection::Left), None);
            }
            Event::KeyDown {
                keycode: Option::Some(Keycode::Right),
                ..
            } => {
                //self.update_dir(Some(HDirection::Right), None);
            }
            Event::KeyUp {
                keycode: Option::Some(Keycode::Up),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Option::Some(Keycode::Down),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Option::Some(Keycode::Left),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            Event::KeyUp {
                keycode: Option::Some(Keycode::Right),
                ..
            } => {
                self.set_waiting_dir_release();
            }
            // Shortcut keys
            Event::KeyUp {
                keycode: Option::Some(keycode),
                ..
            } => {
                self.command_queue.push_back(RawCommand::KeyPress(keycode));
            }
            // Joystick events
            Event::JoyButtonDown { button_idx, .. } => {
                println!("ButtonDown: {}", button_idx);
            }
            Event::JoyAxisMotion { .. } => {
                self.set_waiting_dir_release();
            }
            // Text input events
            Event::TextInput { text, .. } => {
                self.command_queue.push_back(RawCommand::TextInput(text));
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

        if self.command_queue.is_empty() {
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
            return None;
        }

        let rawc = self.command_queue.pop_front().unwrap();

        self.conv_table.conv(rawc, mode)
    }

    /// Update direction input
    pub fn update_dir(&mut self, event_pump: &sdl2::EventPump) {
        let keyboard = sdl2::keyboard::KeyboardState::new(event_pump);
        let mut hdir = HDirection::None;
        let mut vdir = VDirection::None;

        for scancode in keyboard.pressed_scancodes() {
            use sdl2::keyboard::Scancode::*;
            match scancode {
                Up => {
                    vdir = VDirection::Up;
                }
                Down => {
                    vdir = VDirection::Down;
                }
                Left => {
                    hdir = HDirection::Left;
                }
                Right => {
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
    targeting: HashMap<RawCommand, Command>,
}

impl CommandConvTable {
    fn new() -> CommandConvTable {
        let mut normal = HashMap::new();
        let mut dialog = HashMap::new();
        let mut targeting = HashMap::new();

        for (k, v) in INPUT_CFG.normal.iter() {
            let k = conv_str_to_keycode(k);

            normal.insert(RawCommand::KeyPress(k), v.clone());
        }

        for (k, v) in INPUT_CFG.dialog.iter() {
            let k = conv_str_to_keycode(k);

            dialog.insert(RawCommand::KeyPress(k), v.clone());
        }

        for (k, v) in INPUT_CFG.targeting.iter() {
            let k = conv_str_to_keycode(k);

            targeting.insert(RawCommand::KeyPress(k), v.clone());
        }

        CommandConvTable {
            normal,
            dialog,
            targeting,
        }
    }

    fn conv(&self, raw: RawCommand, mode: InputMode) -> Option<Command> {
        let table = match mode {
            InputMode::Normal => &self.normal,
            InputMode::Dialog => &self.dialog,
            InputMode::Targeting => &self.targeting,
            InputMode::TextInput => {
                return text_input_conv(raw);
            }
        };

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

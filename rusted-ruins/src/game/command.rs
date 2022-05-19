use common::gamedata::*;
use geom::*;

/// User inputs are converted to command
/// Command represents user's input, and independent from configuration
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Move {
        dir: Direction,
    },
    MoveTo {
        dest: Coords,
    },
    Shoot {
        target: Coords,
    },
    UseTool {
        target: Coords,
    },
    Enter,
    Cancel,
    RotateWindowRight,
    RotateWindowLeft,
    ItemInformation,
    OpenAbilityWin,
    OpenCreationWin,
    OpenDebugCommandWin,
    OpenEquipWin,
    OpenExitWin,
    OpenGameInfoWin,
    OpenHelpWin,
    OpenStatusWin,
    OpenItemWin,
    PickUpItem,
    DropItem,
    DrinkItem,
    EatItem,
    ReleaseItem,
    ActionShortcut(usize),
    ChangeEquip {
        kind: EquipSlotKind,
    },
    TextInput {
        text: String,
    },
    TextDelete,
    // Mouse
    MouseButtonDown {
        x: i32,
        y: i32,
        button: MouseButton,
        key_state: KeyState,
    },
    MouseButtonUp {
        x: i32,
        y: i32,
        button: MouseButton,
        key_state: KeyState,
    },
    MouseWheel {
        x: i32,
        y: i32,
    },
    MouseState {
        x: i32,
        y: i32,
        left_button: bool,
        right_button: bool,
        key_state: KeyState,
        ui_only: bool,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct KeyState {
    pub ctrl: bool,
    pub shift: bool,
}

impl Command {
    pub fn relative_to<R: Into<(i32, i32, u32, u32)>>(&self, rect: R) -> Command {
        let rect = rect.into();
        self.relative_to_point((rect.0, rect.1))
    }

    pub fn relative_to_point<P: Into<(i32, i32)>>(&self, point: P) -> Command {
        let point = point.into();
        match *self {
            Command::MouseButtonDown {
                x,
                y,
                button,
                key_state,
            } => Command::MouseButtonDown {
                x: x - point.0,
                y: y - point.1,
                button,
                key_state,
            },
            Command::MouseButtonUp {
                x,
                y,
                button,
                key_state,
            } => Command::MouseButtonUp {
                x: x - point.0,
                y: y - point.1,
                button,
                key_state,
            },
            Command::MouseWheel { x, y } => Command::MouseWheel { x, y },
            Command::MouseState {
                x,
                y,
                left_button,
                right_button,
                key_state,
                ui_only,
            } => Command::MouseState {
                x: x - point.0,
                y: y - point.1,
                left_button,
                right_button,
                key_state,
                ui_only,
            },
            _ => self.clone(),
        }
    }
}

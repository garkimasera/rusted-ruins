use geom::*;

/// User inputs are converted to command
/// Command represents user's input, and independent from configuration
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Move {
        dir: Direction,
    },
    Enter,
    Cancel,
    RotateWindowRight,
    RotateWindowLeft,
    ItemInfomation,
    Shot,
    OpenCreationWin,
    OpenDebugCommandWin,
    OpenEquipWin,
    OpenExitWin,
    OpenGameInfoWin,
    OpenHelpWin,
    OpenStatusWin,
    OpenItemMenu,
    PickUpItem,
    DropItem,
    DrinkItem,
    EatItem,
    ReleaseItem,
    TargetingMode,
    TextInput {
        text: String,
    },
    TextDelete,
    // Mouse
    MouseButtonDown {
        x: i32,
        y: i32,
        button: MouseButton,
    },
    MouseButtonUp {
        x: i32,
        y: i32,
        button: MouseButton,
    },
    MouseWheel {
        x: i32,
        y: i32,
        wheel_direction: WheelDirection,
    },
    MouseState {
        x: i32,
        y: i32,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum WheelDirection {
    Normal,
    Flipped,
}

impl Command {
    pub fn relative_to<R: Into<(i32, i32, u32, u32)>>(&self, rect: R) -> Command {
        let rect = rect.into();
        self.relative_to_point((rect.0, rect.1))
    }

    pub fn relative_to_point<P: Into<(i32, i32)>>(&self, point: P) -> Command {
        let point = point.into();
        match *self {
            Command::MouseButtonDown { x, y, button } => Command::MouseButtonDown {
                x: x - point.0,
                y: y - point.1,
                button,
            },
            Command::MouseButtonUp { x, y, button } => Command::MouseButtonUp {
                x: x - point.0,
                y: y - point.1,
                button,
            },
            Command::MouseWheel {
                x,
                y,
                wheel_direction,
            } => Command::MouseWheel {
                x: x - point.0,
                y: y - point.1,
                wheel_direction,
            },
            Command::MouseState { x, y } => Command::MouseState {
                x: x - point.0,
                y: y - point.1,
            },
            _ => self.clone(),
        }
    }
}

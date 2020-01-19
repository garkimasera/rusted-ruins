use geom::*;

/// User inputs are converted to command
/// Command represents user's input, and independent from configuration
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Move { dir: Direction },
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
    TextInput { text: String },
    TextDelete,
    // Mouse
    MouseButtonDown { x: i32, y: i32, button: MouseButton },
    MouseButtonUp { x: i32, y: i32, button: MouseButton },
    MouseWheel { x: i32, y: i32, wheel_direction: WheelDirection },
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

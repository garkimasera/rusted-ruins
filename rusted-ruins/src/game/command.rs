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
}

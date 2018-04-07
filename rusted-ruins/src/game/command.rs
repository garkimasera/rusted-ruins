
use array2d::*;

/// User inputs are converted to command
/// Command represents user's input, and independent from configuration
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Command {
    Move { dir: Direction },
    Enter,
    Cancel,
    RotateWindowRight, RotateWindowLeft,
    OpenExitWin, OpenItemMenu, OpenEquipWin, OpenStatusWin, PickUpItem, DrinkItem, EatItem,
    TargetingMode,
    TextInput { text: String },
    TextDelete,
}

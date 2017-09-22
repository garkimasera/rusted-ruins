
use array2d::*;

/// User inputs are converted to command
/// Command represents user's input, and independent from configuration
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    Move { dir: Direction },
    Enter,
    Cancel,
    OpenExitWin, OpenItemMenu,
}

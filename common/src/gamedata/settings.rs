use crate::basic::MAX_ACTION_SHORTCUTS;
use crate::objholder::*;

/// Custom settings for each game playing
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub action_shortcuts: Vec<Option<ActionShortcut>>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            action_shortcuts: vec![None; MAX_ACTION_SHORTCUTS],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ActionShortcut {
    Throw(ItemIdx),
    Drink(ItemIdx),
    Eat(ItemIdx),
    Use(ItemIdx),
    Release(ItemIdx),
    Read(ItemIdx),
}

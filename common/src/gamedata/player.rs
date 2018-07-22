
/// Unique data for player
#[derive(Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Player {
    money: i64,
}

impl Player {
    pub fn money(&self) -> i64 {
        self.money
    }
}


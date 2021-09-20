use super::CharaId;

/// Unique data for player
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Player {
    money: i64,
    pub party: fnv::FnvHashSet<CharaId>,
    pub party_dead: fnv::FnvHashSet<CharaId>,
}

impl Player {
    pub fn money(&self) -> i64 {
        self.money
    }

    pub fn set_money(&mut self, a: i64) {
        assert!(a >= 0);
        self.money = a;
    }

    pub fn add_money(&mut self, diff: i64) {
        self.money += diff;
    }

    pub fn sub_money(&mut self, diff: i64) {
        self.money -= diff;
    }

    pub fn has_money(&self, a: i64) -> bool {
        self.money >= a
    }
}

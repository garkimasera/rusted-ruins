#[derive(Debug, Serialize, Deserialize)]
pub struct PlayTime(
    /// Number of seconds elapsed since the start of game
    u64,
);

impl Default for PlayTime {
    fn default() -> Self {
        PlayTime(0)
    }
}

impl PlayTime {
    pub fn seconds(&self) -> u64 {
        self.0
    }

    pub fn advance(&mut self, secs: u64) {
        self.0 += secs;
    }
}

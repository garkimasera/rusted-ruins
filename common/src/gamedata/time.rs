
/// Time and date date of game
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Time {
    turn: u64,
    minute: f32,
    hour: u32,
    day: u32,
    month: u32,
    year: u32,
}

/// Represents which time parameter changed
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct TimeChanged {
    hour: bool,
    day: bool,
    month: bool,
    year: bool,
}

impl Default for Time {
    fn default() -> Time {
        Time {
            turn: 0,
            minute: 0.0,
            hour: 0,
            day: 1,
            month: 1,
            year: 1,
        }
    }
}

impl Time {
    pub fn new(year: u32, month: u32, day: u32, hour: u32) -> Time {
        Time {
            turn: 0,
            minute: 0.0,
            hour,
            day,
            month,
            year,
        }
    }
    
    pub fn turn(&self) -> u64 {
        self.turn
    }

    pub fn minute(&self) -> u32 {
        let m = self.minute as u32;
        m - (m % 10)
    }

    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    /// Advance time by given minutes
    pub fn advance_by(&mut self, m: f32) -> TimeChanged {
        let mut changed = TimeChanged::default();

        self.turn += 1;

        self.minute += m;
        if self.minute < 60.0 {
            return changed;
        }
        self.minute -= 60.0;
        
        changed.hour = true;
        self.hour += 1;
        if self.hour < 24 {
            return changed;
        }
        self.hour = 0;

        changed.day = true;
        self.day += 1;
        if self.day <= 30 {
            return changed;
        }
        self.day = 1;

        changed.month = true;
        self.month += 1;
        if self.month <= 12 {
            return changed;
        }
        self.month = 1;

        changed.year = true;
        self.year += 1;
        
        changed
    }
}


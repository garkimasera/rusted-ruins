pub const DAYS_PER_MONTH: u64 = 30;
pub const SECS_PER_MIN: u64 = 60;
pub const SECS_PER_HOUR: u64 = SECS_PER_MIN * 60;
pub const SECS_PER_DAY: u64 = SECS_PER_HOUR * 24;
pub const SECS_PER_MONTH: u64 = SECS_PER_DAY * DAYS_PER_MONTH;
pub const SECS_PER_YEAR: u64 = SECS_PER_MONTH * 12;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GameTime {
    start: Time,
    current: Time,
}

impl GameTime {
    pub fn new(years: u32, months: u32, days: u32, hours: u32) -> GameTime {
        assert!(1 <= months && months <= 12);
        assert!(1 <= days && days <= DAYS_PER_MONTH as u32);

        let start = years as u64 * SECS_PER_YEAR
            + (months - 1) as u64 * SECS_PER_MONTH
            + (days - 1) as u64 * SECS_PER_DAY
            + hours as u64 * SECS_PER_HOUR;
        let start = Time::from_seconds(start);
        GameTime {
            start,
            current: start,
        }
    }

    pub fn current_time(&self) -> Time {
        self.current
    }

    pub fn current_date(&self) -> Date {
        self.current.into_date()
    }

    pub fn advance(&mut self, secs: u64) {
        self.current.advance(secs);
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Time {
    secs: u64,
}

impl Time {
    pub const fn from_seconds(secs: u64) -> Time {
        Time { secs }
    }

    pub fn duration_from(&self, t: Time) -> Duration {
        assert!(t.secs <= self.secs);
        Duration::from_seconds(self.secs - t.secs)
    }

    pub fn into_date(self) -> Date {
        let s = self.secs;
        let year = s / SECS_PER_YEAR;
        let s = s % SECS_PER_YEAR;
        let month = s / SECS_PER_MONTH;
        let s = s % SECS_PER_MONTH;
        let day = s / SECS_PER_DAY;
        let s = s % SECS_PER_DAY;
        let hour = s / SECS_PER_HOUR;
        let s = s % SECS_PER_HOUR;
        let minute = s / SECS_PER_MIN;
        let sec = s % SECS_PER_MIN;

        Date {
            sec: sec as u16,
            minute: minute as u16,
            hour: hour as u16,
            day: day as u16 + 1,
            month: month as u16 + 1,
            year: year as u32,
        }
    }

    pub fn advance(&mut self, secs: u64) {
        self.secs += secs;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Duration {
    secs: u64,
}

impl Duration {
    pub const fn new(
        years: u64,
        months: u64,
        days: u64,
        hours: u64,
        mins: u64,
        secs: u64,
    ) -> Duration {
        Duration {
            secs: years * SECS_PER_YEAR
                + months * SECS_PER_MONTH
                + days * SECS_PER_DAY
                + hours * SECS_PER_HOUR
                + mins * SECS_PER_MIN
                + secs,
        }
    }

    pub const fn from_seconds(secs: u64) -> Duration {
        Duration { secs }
    }

    pub const fn from_minutes(mins: u64) -> Duration {
        Duration {
            secs: mins * SECS_PER_MIN,
        }
    }

    pub const fn from_hours(hours: u64) -> Duration {
        Duration {
            secs: hours * SECS_PER_HOUR,
        }
    }

    pub const fn from_days(days: u64) -> Duration {
        Duration {
            secs: days * SECS_PER_DAY,
        }
    }

    pub const fn as_hours(self) -> i32 {
        (self.secs / SECS_PER_HOUR) as i32
    }
}

#[derive(Debug)]
pub struct Date {
    pub sec: u16,
    pub minute: u16,
    pub hour: u16,
    pub day: u16,
    pub month: u16,
    pub year: u32,
}

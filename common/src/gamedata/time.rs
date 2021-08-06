use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{self, Deserialize, Visitor};
use serde::Serialize;
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use std::ops::Add;

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
        let start = Time::new(years, months, days, hours);

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
#[serde(transparent)]
pub struct Time(u64);

impl Time {
    pub fn new(years: u32, months: u32, days: u32, hours: u32) -> Time {
        assert!((1..=12).contains(&months));
        assert!(1 <= days && days <= DAYS_PER_MONTH as u32);

        let start = years as u64 * SECS_PER_YEAR
            + (months - 1) as u64 * SECS_PER_MONTH
            + (days - 1) as u64 * SECS_PER_DAY
            + hours as u64 * SECS_PER_HOUR;
        Time::from_secs(start)
    }

    pub const fn from_secs(secs: u64) -> Time {
        Time(secs)
    }

    pub const fn as_secs(&self) -> u64 {
        self.0
    }

    pub fn duration_from(&self, t: Time) -> Duration {
        assert!(t.0 <= self.0);
        Duration::from_seconds(self.0 - t.0)
    }

    pub fn into_date(self) -> Date {
        let s = self.0;
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
        self.0 += secs;
    }
}

impl Add<Duration> for Time {
    type Output = Time;

    fn add(self, duration: Duration) -> Self::Output {
        Time(self.0 + duration.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Debug)]
pub struct Duration(u64);

impl Duration {
    pub const fn new(
        years: u64,
        months: u64,
        days: u64,
        hours: u64,
        mins: u64,
        secs: u64,
    ) -> Duration {
        Duration(
            years * SECS_PER_YEAR
                + months * SECS_PER_MONTH
                + days * SECS_PER_DAY
                + hours * SECS_PER_HOUR
                + mins * SECS_PER_MIN
                + secs,
        )
    }

    pub const fn from_seconds(secs: u64) -> Duration {
        Duration(secs)
    }

    pub const fn from_minutes(mins: u64) -> Duration {
        Duration(mins * SECS_PER_MIN)
    }

    pub const fn from_hours(hours: u64) -> Duration {
        Duration(hours * SECS_PER_HOUR)
    }

    pub const fn from_days(days: u64) -> Duration {
        Duration(days * SECS_PER_DAY)
    }

    pub const fn as_secs(self) -> u64 {
        self.0
    }

    pub const fn as_hours(self) -> i32 {
        (self.0 / SECS_PER_HOUR) as i32
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn days(self) -> u64 {
        self.0 / SECS_PER_DAY
    }

    pub fn hours(self) -> u64 {
        self.0 / SECS_PER_HOUR
    }

    pub fn minutes(self) -> u64 {
        self.0 / SECS_PER_MIN
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

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an positive integer or digits string with suffix [dmhs]")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Duration(value.into()))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Duration(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Duration(value))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0-9]+)([dhms])$").unwrap());

        let err_msg = || de::Error::custom(format!("invalid duration string \"{}\"", s));

        let caps = RE.captures(s).ok_or_else(err_msg)?;

        let number = caps
            .get(1)
            .ok_or_else(err_msg)?
            .as_str()
            .parse::<u64>()
            .map_err(|_| err_msg())?;

        let factor = match caps.get(2).ok_or_else(err_msg)?.as_str() {
            "d" => 3600 * 24,
            "h" => 3600,
            "m" => 60,
            "s" => 1,
            _ => return Err(err_msg()),
        };

        Ok(Duration(number * factor))
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(DurationVisitor)
    }
}

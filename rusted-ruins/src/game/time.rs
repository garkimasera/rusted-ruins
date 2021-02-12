use super::Game;
use common::basic::WAIT_TIME_NUMERATOR;
use common::gamedata::time::Time;
use once_cell::sync::Lazy;
use rules::RULES;
use std::sync::Mutex;

static CURRENT_TIME: Lazy<Mutex<Time>> = Lazy::new(|| {
    Mutex::new(Time::new(
        RULES.params.initial_date_year,
        RULES.params.initial_date_month,
        RULES.params.initial_date_day,
        RULES.params.initial_date_hour,
    ))
});

pub fn current_time() -> Time {
    *CURRENT_TIME.lock().unwrap()
}

pub fn advance_game_time(game: &mut Game, advanced_clock: u32) {
    let mid = game.gd.get_current_mapid();
    let minutes_per_turn = if mid.is_region_map() {
        RULES.params.minutes_per_turn_region
    } else {
        RULES.params.minutes_per_turn_normal
    };
    const AVERAGE_CLOCK_PER_TURN: u32 = WAIT_TIME_NUMERATOR / 100;
    let advanced_secs =
        minutes_per_turn * 60.0 * advanced_clock as f32 / AVERAGE_CLOCK_PER_TURN as f32;
    game.gd.time.advance(advanced_secs as u64);
    *CURRENT_TIME.lock().unwrap() = game.gd.time.current_time();
}

pub fn update_time(game: &mut Game) {
    *CURRENT_TIME.lock().unwrap() = game.gd.time.current_time();
}

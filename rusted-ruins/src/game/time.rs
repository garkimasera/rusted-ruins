use super::Game;
use common::basic::WAIT_TIME_NUMERATOR;
use common::gamedata::time::Time;
use once_cell::sync::Lazy;
use rules::RULES;
use std::sync::Mutex;

static CURRENT_TIME: Lazy<Mutex<Time>> = Lazy::new(|| {
    Mutex::new(Time::new(
        RULES.newgame.initial_date_year,
        RULES.newgame.initial_date_month,
        RULES.newgame.initial_date_day,
        RULES.newgame.initial_date_hour,
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
    let before = game.gd.time.current_time();
    game.gd.time.advance(advanced_secs as u64);
    let now = game.gd.time.current_time();
    *CURRENT_TIME.lock().unwrap() = now;

    // Update checks
    let before = before.into_date();
    let now = now.into_date();

    // 10 minutes
    if before.minute / 10 != now.minute / 10 {
        info!("time update process (10 minutes)");
        crate::game::item::time::update_item_time(&mut game.gd);
    }

    // day
    if before.day != now.day {
        info!("time update process (day)");
    }
}

pub fn update_time(game: &mut Game) {
    *CURRENT_TIME.lock().unwrap() = game.gd.time.current_time();
}

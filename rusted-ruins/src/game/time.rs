use super::Game;
use common::basic::WAIT_TIME_NUMERATOR;
use common::gamedata::{time::*, CharaId, GameData};
use common::gobj;
use once_cell::sync::Lazy;
use rules::RULES;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

static CURRENT_TIME: Lazy<Mutex<Time>> = Lazy::new(|| {
    Mutex::new(Time::new(
        RULES.newgame.initial_date_year,
        RULES.newgame.initial_date_month,
        RULES.newgame.initial_date_day,
        RULES.newgame.initial_date_hour,
    ))
});

/// Player moved at the previous turn or not
static PLAYER_MOVED: AtomicBool = AtomicBool::new(false);

pub fn player_moved() -> bool {
    PLAYER_MOVED.load(Ordering::Relaxed)
}

pub fn set_player_moved() {
    PLAYER_MOVED.store(true, Ordering::Relaxed);
}

pub fn clear_player_moved() {
    PLAYER_MOVED.store(false, Ordering::Relaxed);
}

pub fn current_time() -> Time {
    *CURRENT_TIME.lock().unwrap()
}

pub fn advance_game_time_by_clock(game: &mut Game, advanced_clock: u32) {
    let mid = game.gd.get_current_mapid();
    let minutes_per_turn = if mid.is_region_map() && player_moved() {
        minutes_per_turn_region(&game.gd)
    } else {
        RULES.params.minutes_per_turn_normal
    };
    const AVERAGE_CLOCK_PER_TURN: u32 = WAIT_TIME_NUMERATOR / 100;
    let advanced_secs =
        minutes_per_turn * 60.0 * advanced_clock as f32 / AVERAGE_CLOCK_PER_TURN as f32;
    advance_game_time_by_secs(game, advanced_secs as u64);
}

pub fn advance_game_time_by_secs(game: &mut Game, advanced_secs: u64) {
    let before = game.gd.time.current_time();
    game.gd.time.advance(advanced_secs);
    let now = game.gd.time.current_time();
    *CURRENT_TIME.lock().unwrap() = now;

    // Update checks
    let duration_s = now.duration_from(before).as_secs();
    let before = before.into_date();
    let now = now.into_date();

    // 10 minutes
    if before.minute / 10 != now.minute / 10 || duration_s >= SECS_PER_MIN * 10 {
        info!("time update process (10 minutes)");
        crate::game::item::time::update_item_time(&mut game.gd);
    }

    // day
    if before.day != now.day || duration_s >= SECS_PER_DAY {
        info!("time update process (day)");
    }
}

pub fn reset_time(time: Time) {
    *CURRENT_TIME.lock().unwrap() = time;
}

pub fn update_time(game: &mut Game) {
    *CURRENT_TIME.lock().unwrap() = game.gd.time.current_time();
}

fn minutes_per_turn_region(gd: &GameData) -> f32 {
    let speed = regionmap_speed(gd).0 as f32;
    (RULES.params.regionmap_tile_size / speed) * 60.0
}

pub fn regionmap_speed(gd: &GameData) -> (u32, f32) {
    let idx = gd.chara.get(CharaId::Player).idx;
    let obj = gobj::get_obj(idx);
    let player_speed = obj.base_attr.travel_speed;

    let party = gd.player.party.clone();
    let cruise_speed = party
        .into_iter()
        .map(|cid| {
            let idx = gd.chara.get(cid).idx;
            let obj = gobj::get_obj(idx);
            (obj.base_attr.carry, obj.base_attr.travel_speed)
        })
        .max_by_key(|(carry, _)| *carry)
        .map(|(_, cruise_speed)| cruise_speed)
        .unwrap_or_else(|| {
            let idx = gd.chara.get(CharaId::Player).idx;
            let obj = gobj::get_obj(idx);
            obj.base_attr.travel_speed
        }) as u32;
    (cruise_speed, cruise_speed as f32 / player_speed as f32)
}

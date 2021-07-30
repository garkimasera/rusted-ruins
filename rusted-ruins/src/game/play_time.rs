use common::gamedata::PlayTime;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

static LOAD_TIME: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));
static PLAY_TIME_ON_LOAD: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
static COUNT_ID_GEN: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));

fn elapsed_since_load() -> Duration {
    let now = Instant::now();
    let load_time = *LOAD_TIME.lock().unwrap();

    if now <= load_time {
        Duration::from_secs(0)
    } else {
        now.duration_since(load_time)
    }
}

#[extend::ext(pub)]
impl PlayTime {
    fn start(&mut self) {
        self.advance(1);

        *LOAD_TIME.lock().unwrap() = Instant::now();
        PLAY_TIME_ON_LOAD.store(self.seconds(), Ordering::Relaxed);
        COUNT_ID_GEN.store(0, Ordering::Relaxed);
    }

    fn update(&mut self) {
        let elapsed_secs_since_load = elapsed_since_load().as_secs();
        let new_play_time = PLAY_TIME_ON_LOAD.load(Ordering::Relaxed) + elapsed_secs_since_load;
        let current_play_time = self.seconds();
        dbg!(current_play_time);

        if current_play_time < new_play_time {
            self.advance(new_play_time - current_play_time);
            COUNT_ID_GEN.store(0, Ordering::Relaxed);
        }
    }
}

use common::gamedata::{PlayTime, UniqueId, UniqueIdGenerator};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

static LOAD_TIME: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));
static PLAY_TIME_ON_LOAD: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
static COUNT_ID_GEN: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
static CURRENT_PLAY_TIME: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));

pub fn play_time_as_secs() -> u64 {
    CURRENT_PLAY_TIME.load(Ordering::Relaxed)
}

fn elapsed_since_load() -> Duration {
    let now = Instant::now();
    let load_time = *LOAD_TIME.lock().unwrap();

    if now <= load_time {
        Duration::from_secs(0)
    } else {
        now.duration_since(load_time)
    }
}

#[derive(Debug)]
pub struct UniqueIdGeneratorByTime;

impl UniqueIdGenerator for UniqueIdGeneratorByTime {
    fn generate(&mut self) -> UniqueId {
        let count = COUNT_ID_GEN.fetch_add(1, Ordering::Relaxed);
        let current_play_time = CURRENT_PLAY_TIME.load(Ordering::Relaxed);

        current_play_time << 24 | count
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

        if current_play_time < new_play_time {
            self.advance(new_play_time - current_play_time);
            COUNT_ID_GEN.store(0, Ordering::Relaxed);
            CURRENT_PLAY_TIME.store(self.seconds(), Ordering::Relaxed);
        }

        debug_assert_eq!(new_play_time, self.seconds());
    }
}

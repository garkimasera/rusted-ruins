
pub use failure::Error;

macro_rules! check_draw {
    ($rst:expr) => {
        match $rst {
            Ok(_) => (),
            Err(e) => { eprintln!("SDL drawing error : {}", e) },
        }
    }
}


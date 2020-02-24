macro_rules! try_sdl {
    ($rst:expr) => {
        match $rst {
            Ok(_) => (),
            Err(e) => error!("SDL error : {}", e),
        }
    };
}

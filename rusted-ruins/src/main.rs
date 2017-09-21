
extern crate sdl2;
#[macro_use]
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_audio as audio;
extern crate rand;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log as applog;
extern crate env_logger;
extern crate toml;
extern crate walkdir;

#[macro_use]
mod error;
#[macro_use]
mod log;
mod util;
mod config;
mod game;
mod obj;
mod sdlvalues;
mod draw;
mod text;
mod window;
mod screen;
mod eventhandler;
mod sdltypeconv;

fn main() {
    setup_logger();
    init_lazy_statics();
    
    game_log!("start"; version="0.0.1");
    
    let sdl_context = SdlContext::init();
    let mut screen = screen::Screen::new(&sdl_context.sdl_context);

    screen.main_loop(&sdl_context);
}

pub struct SdlContext {
    pub sdl_context: sdl2::Sdl,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
    _image: sdl2::image::Sdl2ImageContext,
    _audio_context: audio::AudioContext,
}

impl SdlContext {
    fn init() -> SdlContext {
        SdlContext {
            sdl_context: sdl2::init().expect("Init Failed : SDL Context"),
            ttf_context: sdl2::ttf::init().expect("Init Failed : SDL_ttf Context"),
            _image: sdl2::image::init(sdl2::image::INIT_PNG).expect("Init Failed : SDL_Image"),
            _audio_context: audio::init(&[&*config::APP_DIR]),
        }
    }
}

/// Initialize lazy_static values
fn init_lazy_statics() {
    config::init();
    obj::init();
    text::init();
    log::init();
}

/// Setup logger. It is not game logger. It is for debug and warning infomation.
fn setup_logger() {
    use applog::{LogRecord, LogLevelFilter};
    use std::env;
    let mut builder = env_logger::LogBuilder::new();

    let format = |record: &LogRecord| {
        format!("{}: {}", record.level(), record.args())
    };

    builder.format(format);
    builder.filter(None, LogLevelFilter::Info);
    if let Ok(e) = env::var("RUST_LOG") {
        builder.parse(&e);
    }
    builder.init().unwrap();
}

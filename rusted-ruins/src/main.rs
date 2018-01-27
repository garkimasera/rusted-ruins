
extern crate sdl2;
extern crate rusted_ruins_array2d as array2d;
#[macro_use]
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_audio as audio;
extern crate rusted_ruins_rng as rng;
extern crate rusted_ruins_rules as rules;
extern crate rusted_ruins_map_generator as map_generator;
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
    init_obj();
    init_rules();
    
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
    text::init();
    log::init();
}

fn init_obj() {
    let mut data_dirs = ::config::get_data_dirs();
    for d in data_dirs.iter_mut() {
        d.push("paks");
    }
    ::common::gobj::init(data_dirs);
}

fn init_rules() {
    rules::init(&*::config::APP_DIR);
}

/// Setup logger. It is not game logger. It is for debug and warning infomation.
fn setup_logger() {
    use applog::LevelFilter;
    use std::env;
    use std::io::Write;
    let mut builder = env_logger::Builder::new();

    builder.format(|buf, record| { writeln!(buf, "{}: {}", record.level(), record.args()) });
    builder.filter(None, LevelFilter::Info);
    if let Ok(e) = env::var("RUST_LOG") {
        builder.parse(&e);
    }
    builder.init();
}

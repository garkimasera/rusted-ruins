#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]
#![allow(clippy::comparison_chain)]

extern crate rusted_ruins_audio as audio;
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_map_generator as map_generator;
extern crate rusted_ruins_rng as rng;
extern crate rusted_ruins_rules as rules;
extern crate rusted_ruins_script as script;
extern crate tile_geom as geom;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log as _;

#[macro_use]
mod util;
#[macro_use]
mod error;
#[macro_use]
mod log;
#[macro_use]
mod text;
mod config;
mod context;
mod cursor;
mod damage_popup;
mod draw;
mod eventhandler;
mod game;
mod msg_box;
mod screen;
mod sdltypeconv;
mod window;

fn main() {
    setup_logger();
    init_lazy();
    let se = script::ScriptEngine::start_init(crate::game::script_methods::game_method_caller);
    init_obj();
    // Must be after init_obj()
    init_rules();

    let sdl_context = SdlContext::init();
    let mut screen = screen::Screen::new(&sdl_context.sdl_context);
    cursor::load();

    se.wait_init();
    screen.main_loop(&sdl_context, se);
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
            sdl_context: sdl2::init().expect("init failed : SDL Context"),
            ttf_context: sdl2::ttf::init().expect("init failed : SDL_ttf Context"),
            _image: sdl2::image::init(sdl2::image::InitFlag::PNG).expect("init failed : SDL_Image"),
            _audio_context: audio::init(
                &config::get_data_dirs(),
                crate::config::CONFIG.sound_effect_volume,
                crate::config::CONFIG.music_volume,
            ),
        }
    }
}

/// Initialize lazy values
fn init_lazy() {
    config::init();
    text::init();
    log::init();
}

fn init_obj() {
    let mut data_dirs = crate::config::get_data_dirs();
    for d in data_dirs.iter_mut() {
        info!("loading objects from \"{}\"", d.to_string_lossy());
        d.push("paks");
    }
    common::gobj::init(data_dirs);
}

fn init_rules() {
    rules::init(
        &*crate::config::ASSETS_DIR,
        crate::config::ADDON_DIR.as_ref(),
    );
}

/// Setup logger. It is not game logger. It is for debug and warning information.
fn setup_logger() {
    env_logger::builder().format_timestamp(None).init();
}

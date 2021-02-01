extern crate rusted_ruins_audio as audio;
extern crate rusted_ruins_common as common;
extern crate rusted_ruins_geom as geom;
extern crate rusted_ruins_map_generator as map_generator;
extern crate rusted_ruins_rng as rng;
extern crate rusted_ruins_rules as rules;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log as applog;

#[macro_use]
mod error;
#[macro_use]
mod log;
#[macro_use]
mod text;
mod chara_log;
mod config;
mod context;
mod draw;
mod eventhandler;
mod game;
mod screen;
mod sdltypeconv;
mod window;

fn main() {
    setup_logger();
    init_lazy();
    init_obj();
    // Must be after init_obj()
    init_rules();

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
            _image: sdl2::image::init(sdl2::image::InitFlag::PNG).expect("Init Failed : SDL_Image"),
            _audio_context: audio::init(
                &config::get_data_dirs(),
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
        info!("Loading objects from \"{}\"", d.to_string_lossy());
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

/// Setup logger. It is not game logger. It is for debug and warning infomation.
fn setup_logger() {
    env_logger::builder().format_timestamp(None).init();
}

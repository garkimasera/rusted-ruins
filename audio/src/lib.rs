//! This crate provides functions for sound and music playing.
//! All function must be called from main thread only.

#![warn(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

#[macro_use]
extern crate log;

#[macro_use]
mod tool;
mod datwalker;
mod musictable;
mod wavtable;

use crate::musictable::MusicTable;
use crate::wavtable::WavTable;
use std::cell::RefCell;
use std::path::Path;

thread_local!(static AUDIO_PLAYER: RefCell<Option<AudioPlayer>> = RefCell::new(None));

/// MixerContext provides a context for systems with and without
/// audio mixer devices.
pub enum MixerContext {
    SDL2MixerContext(sdl2::mixer::Sdl2MixerContext),
    NullMixerContext,
}

pub struct AudioContext {
    _mixer_context: MixerContext,
}

/// Initialize AudioPlayer
pub fn init<P: AsRef<Path>>(
    data_dirs: &[P],
    sound_effect_volume: i32,
    music_volume: i32,
) -> AudioContext {
    let mixer_context = init_device();

    if let MixerContext::SDL2MixerContext(_) = mixer_context {
        // Setup the audio player for the SDL case
        AUDIO_PLAYER.with(|a| {
            assert!(a.borrow().is_none());
            *a.borrow_mut() = Some(AudioPlayer::new(data_dirs, sound_effect_volume));
        });

        sdl2::mixer::Music::set_volume(music_volume);
    }
    AudioContext {
        _mixer_context: mixer_context,
    }
}

pub fn with_audio_player<F: FnOnce(&AudioPlayer)>(f: F) {
    AUDIO_PLAYER.with(|a| {
        if a.borrow().is_some() {
            f(a.borrow().as_ref().unwrap());
        }
    });
}

/// Play an sound (wav file)
pub fn play_sound(name: &str) {
    if name.is_empty() {
        return;
    }
    with_audio_player(|a| {
        a.play_sound(name);
    });
}

/// Play an music (ogg file)
pub fn play_music(name: &str) {
    if name.is_empty() {
        return;
    }

    with_audio_player(|a| {
        a.play_music(name);
    });
}

fn finalize() {
    AUDIO_PLAYER.with(|a| {
        if a.borrow().is_some() {
            *a.borrow_mut() = None;
        }
    });
}

impl Drop for AudioContext {
    fn drop(&mut self) {
        finalize();
    }
}

pub struct AudioPlayer {
    wavtable: WavTable,
    musictable: MusicTable,
}

impl AudioPlayer {
    pub fn new<P: AsRef<Path>>(data_dirs: &[P], sound_effect_volume: i32) -> AudioPlayer {
        let wavtable = WavTable::new(data_dirs, sound_effect_volume);
        let musictable = MusicTable::new(data_dirs);
        AudioPlayer {
            wavtable,
            musictable,
        }
    }

    pub fn play_sound(&self, name: &str) {
        if let Err(e) = self.wavtable.play(name) {
            warn!("{}", e);
        }
    }

    pub fn play_music(&self, name: &str) {
        if let Err(e) = self.musictable.play(name) {
            warn!("{}", e);
        }
    }
}

/// Initialize an audio device and return a MixerContext
/// If no device is able to be initialized, a NullMixerContext is returned
fn init_device() -> MixerContext {
    use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

    // Initialization for sound
    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1024;

    let open_audio_result = sdl2::mixer::open_audio(frequency, format, channels, chunk_size);
    let mixer_context = if let Err(e) = open_audio_result {
        // Return null mixer context if we can't open the audio
        warn!("couldn't open audio: {}", e);
        MixerContext::NullMixerContext
    } else {
        match sdl2::mixer::init(InitFlag::OPUS) {
            Ok(m) => {
                sdl2::mixer::allocate_channels(1);
                MixerContext::SDL2MixerContext(m)
            }
            Err(s) => {
                warn!("{}", s);
                MixerContext::NullMixerContext
            }
        }
    };

    mixer_context
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::var;

    #[test]
    fn play_from_sound_dir() {
        let sdl = sdl2::init().unwrap();
        let _audio = sdl.audio().unwrap();

        let app_dir = match var("RUSTED_RUINS_ASSETS_DIR") {
            Ok(o) => o,
            Err(_) => {
                eprintln!("skip sound test");
                return;
            }
        };

        let _audio_context = init(&[app_dir], 80, 80);

        play_music("test");
        std::thread::sleep(std::time::Duration::from_millis(3000));
        play_sound("anim.club");
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }

    #[test]
    fn play_with_no_audio() {
        // Shouldn't panic
        assert_eq!(play_music("test"), ());

        // Setup an explicit empty AudioPlayer
        AUDIO_PLAYER.with(|a| {
            *a.borrow_mut() = None;
        });

        // Shouldn't panic
        assert_eq!(play_music("test"), ());
    }
}

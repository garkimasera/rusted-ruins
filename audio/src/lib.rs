//! This crate provides functions for sound and music playing.
//! All function must be called from main thread only.

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

pub struct AudioContext {
    _mixer_context: sdl2::mixer::Sdl2MixerContext,
}

/// Initialize AudioPlayer
pub fn init<P: AsRef<Path>>(data_dirs: &[P], music_volume: i32) -> AudioContext {
    let mixer_context = init_device();

    AUDIO_PLAYER.with(|a| {
        assert!(a.borrow().is_none());
        *a.borrow_mut() = Some(AudioPlayer::new(data_dirs));
    });
    sdl2::mixer::Music::set_volume(music_volume);
    AudioContext {
        _mixer_context: mixer_context,
    }
}

pub fn with_audio_player<F: FnOnce(&AudioPlayer)>(f: F) {
    AUDIO_PLAYER.with(|a| {
        assert!(a.borrow().is_some());
        f(a.borrow().as_ref().unwrap());
    });
}

/// Play an sound (wav file)
pub fn play_sound(name: &str) {
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
        assert!(a.borrow().is_some());
        *a.borrow_mut() = None;
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
    pub fn new<P: AsRef<Path>>(data_dirs: &[P]) -> AudioPlayer {
        let wavtable = WavTable::new(data_dirs);
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

fn init_device() -> sdl2::mixer::Sdl2MixerContext {
    use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

    // Initialization for sound
    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    let mixer_context = sdl2::mixer::init(InitFlag::OPUS).unwrap();

    sdl2::mixer::allocate_channels(1);

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

        let _audio_context = init(&[app_dir], 80);

        play_music("test");
        std::thread::sleep(std::time::Duration::from_millis(3000));
        play_sound("anim.club");
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}

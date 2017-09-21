
#[macro_use]
extern crate log;
extern crate sdl2;

#[macro_use]
mod tool;
mod wavtable;

use std::path::Path;
use wavtable::WavTable;

pub struct AudioPlayer {
    _mixer_context: sdl2::mixer::Sdl2MixerContext,
    wavtable: WavTable,
}

impl AudioPlayer {
    pub fn new<P: AsRef<Path>>(app_dirs: &[P]) -> AudioPlayer {
        let mixer_context = init_device();
        let wavtable = WavTable::new(app_dirs);
        AudioPlayer {
            _mixer_context: mixer_context,
            wavtable,
        }
    }

    pub fn play_sound(&self, name: &str) {
        if let Err(e) = self.wavtable.play(name) {
            warn!("{}", e);
        }
    }
}

pub fn init_device() -> sdl2::mixer::Sdl2MixerContext {
    use sdl2::mixer::{INIT_OGG, DEFAULT_CHANNELS, AUDIO_S16LSB};
    
    let mixer_context = sdl2::mixer::init(INIT_OGG).unwrap();

    // Initialization for sound
    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
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
        
        let app_dir = match var("RUSTED_RUINS_APP_DIR") {
            Ok(o) => o,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let audio_player = AudioPlayer::new(&[app_dir]);

        let wavname = "anim.club";
        println!("Play of wavfile \"{}\" : {:?}", wavname, audio_player.play_sound(wavname));
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

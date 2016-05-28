//
// Author: Joshua Holmes
//

extern crate sdl2;

use sdl2::Sdl;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};
use std::thread;
use std::time::Duration;

/// The audio level we use for system beeps
pub const AUDIO_LEVEL: f32 = 1.0;
/// The frequency of audio playback
pub const AUDIO_FREQUENCY: i32 = 44100;

/// Structure for a simple audio callback
pub struct MyAudioCallback {
    volume: f32,
}

impl AudioCallback for MyAudioCallback {
    type Channel = f32;

    /// The main audio callback
    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.volume;
        }
    }
}

/// Structure to manage audio playback
pub struct Audio {
    device: AudioDevice<MyAudioCallback>,
}

impl Audio {
    /// Construct a new Audio structure
    pub fn new(sdl_context: &Sdl) -> Audio {
        let callback = MyAudioCallback {
            volume: AUDIO_LEVEL,
        };

        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(AUDIO_FREQUENCY),
            channels: Some(1),
            samples: None,
        };

        // use default device
        let mut device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            callback
        }).unwrap();

        Audio {
            device: device,
        }
    }

    /// Make a beep noise
    pub fn beep(&self) {
        self.device.resume();
        thread::sleep(Duration::from_millis(100));
    }
}
//
// Author: Joshua Holmes
//

extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::keyboard::Keycode::*;

/// Structure to abstract away the keyboard
pub struct Keyboard {
    /// says whether or not the given key is pressed
    pub keys: [bool; 16],
}

impl Keyboard {
    /// Construct a new keyboard
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16],
        }
    }

    /// Says whether or not the given key is pressed
    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    /// Presses the given key and sets the appropriate flag
    pub fn update_key(&mut self, key: Keycode, state: bool) {
        match key {
            Num1 => self.keys[0x1] = state,
            Num2 => self.keys[0x2] = state,
            Num3 => self.keys[0x3] = state,
            Num4 => self.keys[0xC] = state,
            Q => self.keys[0x4] = state,
            W => self.keys[0x5] = state,
            E => self.keys[0x6] = state,
            R => self.keys[0xD] = state,
            A => self.keys[0x7] = state,
            S => self.keys[0x8] = state,
            D => self.keys[0x9] = state,
            F => self.keys[0xE] = state,
            Z => self.keys[0xA] = state,
            X => self.keys[0x0] = state,
            C => self.keys[0xB] = state,
            V => self.keys[0xF] = state,
            _ => {},
        }
    }
}
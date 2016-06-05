//
// Author: Joshua Holmes
//

extern crate rand;
extern crate sdl2;

use std::str;
use std::env;

mod cpu;
mod opcode;
mod display;
mod keyboard;

use cpu::Cpu;
use display::Display;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    // get the program filename from the commandline and load it up
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];

    let mut cpu = match Cpu::init_from_file_path(filename) {
        Err(e) => panic!("Failed to load user program. Error message: {:?}", e),
        Ok(v) => v
    };

    // initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::new(&sdl_context);

    // execute the program until the user presses escape
    println!("Done loading user program. Beginning execution.");
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: while cpu.fetch_and_execute(&mut display) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Escape => break 'running,
                        _ => cpu.keyboard.update_key(key, true),
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => cpu.keyboard.update_key(key, false),
                _ => {}
            }
        }
    }

    println!("Program execution complete.");
}

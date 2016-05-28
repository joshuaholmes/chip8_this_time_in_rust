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

    // initialize the window
    let mut display = Display::new();

    // execute the program until the user presses escape
    println!("Done loading user program. Beginning execution.");
    let mut event_pump = display.sdl_context.event_pump().unwrap();

    'running: while cpu.fetch_and_execute(&mut display) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }

    println!("Program execution complete.");
}

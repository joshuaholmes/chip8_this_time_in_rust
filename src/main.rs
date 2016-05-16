//
// Author: Joshua Holmes
//

use std::str;
use std::env;

mod cpu;
mod opcode;
mod display;

use cpu::Cpu;

fn main() {
    // get the program filename from the commandline and load it up
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];

    let cpu = match Cpu::init_from_file_path(filename) {
        Err(e) => panic!("Failed to load user program. Error message: {:?}", e),
        Ok(v) => v
    };

    println!("Done loading user program.");
}

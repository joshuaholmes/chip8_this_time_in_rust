//
// Author: Joshua Holmes
//

use std::str;
use std::env;

mod cpu;
mod opcode;
mod display;

use cpu::*;

extern crate rand;

fn main() {
    // get the program filename from the commandline and load it up
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];

    let mut cpu = match Cpu::init_from_file_path(filename) {
        Err(e) => panic!("Failed to load user program. Error message: {:?}", e),
        Ok(v) => v
    };

    // execute the program
    println!("Done loading user program. Beginning execution.");

    while cpu.fetch_and_execute() {}
}

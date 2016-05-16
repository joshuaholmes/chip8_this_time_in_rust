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

    let cpu = Cpu::init_from_file_path(filename);

    println!("Done");
}

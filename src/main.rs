//
// Author: Joshua Holmes
//

use std::str;
use std::env;

mod cpu;
mod opcode;
mod display;

use cpu::*;
use opcode::*;

fn main() {
    // get the program filename from the commandline and load it up
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];

    let cpu = match Cpu::init_from_file_path(filename) {
        Err(e) => panic!("Failed to load user program. Error message: {:?}", e),
        Ok(v) => v
    };

    println!("Done loading user program.");

    // testing -- let's disassemble the whole program to see if the 
    // opcode lookups and disassembly strings are working as intended
    let num_instructions = cpu.program_length / 2;
    println!("Program disassembly:");

    for i in 0..num_instructions {
        let index = (i * 2) + USER_PROGRAM_START_ADDR;
        let instruction = ((cpu.memory[index] as u16) << 8) | (cpu.memory[index + 1] as u16);
        //println!("Opcode: 0x{:04X}", instruction);

        let opcode = OpCode::from_u16(instruction).unwrap();
        println!("{}", opcode.disasm_str);
    }
}

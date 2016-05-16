// 
// Author: Joshua Holmes
//

use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub const MEMORY_LENGTH: usize = 0xFFF;
pub const STACK_LENGTH: usize = 0xF;
pub const USER_PROGRAM_START_ADDR: usize = 0x200;

#[derive(Debug)]
pub enum GameLoadError {
    IoError(io::Error),
}

impl From<io::Error> for GameLoadError {
    fn from(err: io::Error) -> Self {
        GameLoadError::IoError(err)
    }
}

/// Structure to represent the virtual CPU and perform execution
pub struct Cpu {
    /// the main system memory
    pub memory: [u8; MEMORY_LENGTH],
    /// the system data registers, V0 through VF
    pub data_registers: [u8; 0xF],
    /// the I register, used for storing addresses
    pub i_register: u16,
    /// the delay timer, decreased at 60Hz by default when non-zero
    pub delay_timer: u8,
    /// the sound timer, decreased at 60Hz by default when non-zero
    pub sound_timer: u8,
    // the program counter, points to the current instruction in memory
    pub program_counter: u16,
    // the stack pointer, points to the current index in the stack
    pub stack_pointer: u8,
    // the call stack, stores return addresses from subroutines
    pub stack: [u16; STACK_LENGTH],
}

impl Cpu {
    /// Init the system from a file path pointing to a CHIP-8 program file
    pub fn init_from_file_path(filepath: &str) -> Result<Cpu, GameLoadError> {
        let path = Path::new(filepath);

        let mut file = match File::open(&path) {
            Err(e) => panic!("Couldn't open program file. Error message: {}", Error::description(&e)),
            Ok(file) => file,
        };

        Cpu::init_from_file(&mut file)
    }

    /// Init the system from a File that contains a CHIP-8 program
    pub fn init_from_file(file: &mut File) -> Result<Cpu, GameLoadError> {
        // read the program into a buffer
        let mut buf = Vec::new();

        match file.read_to_end(&mut buf) {
            Err(e) => panic!("Couldn't read program file. Error message: {}", Error::description(&e)),
            Ok(_) => (),
        };

        Cpu::init_from_buffer(buf)
    }

    /// Init the system from a byte vector containing a CHIP-8 program
    pub fn init_from_buffer(buf: Vec<u8>) -> Result<Cpu, GameLoadError> {
        Ok(Cpu{
            memory: [0u8; MEMORY_LENGTH],
            data_registers: [0u8; 0xF],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0u16; STACK_LENGTH],
        })
    }
}
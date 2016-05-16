// 
// Author: Joshua Holmes
//

use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use opcode::OpCode;

/// How many bytes of system memory there are
pub const MEMORY_LENGTH: usize = 0xFFF;
/// How many items our stack holds
pub const STACK_LENGTH: usize = 0xF;
/// The number of data registers we have
pub const NUM_REGISTERS: usize = 0xF;
/// The address at which the system's font data starts in memory
pub const FONT_SET_START_ADDR: usize = 0x000;
/// The address in memory where the user program begins
pub const USER_PROGRAM_START_ADDR: usize = 0x200;
/// The fontset of the interpreter that can be referenced by user programs
pub const FONT_SET: [u8; 80] = [ 0xF0, 0x90, 0x90, 0x90, 0xF0,   // 0x0
                                 0x20, 0x60, 0x20, 0x20, 0x70,   // 0x1
                                 0xF0, 0x10, 0xF0, 0x80, 0xF0,   // 0x2
                                 0xF0, 0x10, 0xF0, 0x10, 0xF0,   // 0x3
                                 0x90, 0x90, 0xF0, 0x10, 0x10,   // 0x4
                                 0xF0, 0x80, 0xF0, 0x10, 0xF0,   // 0x5
                                 0xF0, 0x80, 0xF0, 0x90, 0xF0,   // 0x6
                                 0xF0, 0x10, 0x20, 0x40, 0x40,   // 0x7
                                 0xF0, 0x90, 0xF0, 0x90, 0xF0,   // 0x8
                                 0xF0, 0x90, 0xF0, 0x10, 0xF0,   // 0x9
                                 0xF0, 0x90, 0xF0, 0x90, 0x90,   // 0xA
                                 0xE0, 0x90, 0xE0, 0x90, 0xE0,   // 0xB
                                 0xF0, 0x80, 0x80, 0x80, 0xF0,   // 0xC
                                 0xE0, 0x90, 0x90, 0x90, 0xE0,   // 0xD
                                 0xF0, 0x80, 0xF0, 0x80, 0xF0,   // 0xE
                                 0xF0, 0x80, 0xF0, 0x80, 0x80 ]; // 0xF

#[derive(Debug)]
pub enum ProgramLoadError {
    IoError(io::Error),
}

impl From<io::Error> for ProgramLoadError {
    fn from(err: io::Error) -> Self {
        ProgramLoadError::IoError(err)
    }
}

/// Structure to represent the virtual CPU and perform execution
pub struct Cpu {
    /// the main system memory
    pub memory: [u8; MEMORY_LENGTH],
    /// the system data registers, V0 through VF
    pub data_registers: [u8; NUM_REGISTERS],
    /// the I register, used for storing addresses
    pub i_register: u16,
    /// the delay timer, decreased at 60Hz by default when non-zero
    pub delay_timer: u8,
    /// the sound timer, decreased at 60Hz by default when non-zero
    pub sound_timer: u8,
    /// the program counter, points to the current instruction in memory
    pub program_counter: usize,
    /// the stack pointer, points to the current index in the stack
    pub stack_pointer: usize,
    /// the call stack, stores return addresses from subroutines
    pub stack: [u16; STACK_LENGTH],
    /// use this to know if the PC is past the end of the program
    pub program_length: usize,
}

impl Cpu {
    /// Init the system from a file path pointing to a CHIP-8 program file
    pub fn init_from_file_path(filepath: &str) -> Result<Cpu, ProgramLoadError> {
        let path = Path::new(filepath);

        let mut file = match File::open(&path) {
            Err(e) => panic!("Couldn't open program file. Error message: {}", Error::description(&e)),
            Ok(file) => file,
        };

        Cpu::init_from_file(&mut file)
    }

    /// Init the system from a File that contains a CHIP-8 program
    pub fn init_from_file(file: &mut File) -> Result<Cpu, ProgramLoadError> {
        // read the program into a buffer
        let mut buf = Vec::new();

        match file.read_to_end(&mut buf) {
            Err(e) => panic!("Couldn't read program file. Error message: {}", Error::description(&e)),
            Ok(_) => (),
        };

        Cpu::init_from_buffer(buf)
    }

    /// Init the system from a byte vector containing a CHIP-8 program
    pub fn init_from_buffer(buf: Vec<u8>) -> Result<Cpu, ProgramLoadError> {
        // copy the user program into system memory
        if buf.len() > MEMORY_LENGTH - USER_PROGRAM_START_ADDR {
            panic!("Program file too big to fit into system memory. Size: {}", buf.len())
        }

        let mut memory = [0u8; MEMORY_LENGTH];

        for (i, x) in buf.iter().enumerate() {
            memory[USER_PROGRAM_START_ADDR + i] = *x;
        }

        // copy the font set into system memory
        for (arr_index, buf_index) in (FONT_SET_START_ADDR..FONT_SET_START_ADDR + FONT_SET.len()).enumerate() {
            memory[buf_index] = FONT_SET[arr_index];
        }

        Ok(Cpu{
            memory: memory,
            data_registers: [0u8; NUM_REGISTERS],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: USER_PROGRAM_START_ADDR,
            stack_pointer: 0,
            stack: [0u16; STACK_LENGTH],
            program_length: buf.len(),
        })
    }

    /// Fetches one opcode from memory and executes it.
    pub fn fetch_and_execute(&mut self) -> bool {
        // if the program counter is past the program, then we've completed execution
        if self.program_counter >= USER_PROGRAM_START_ADDR + self.program_length {
            return false;
        }

        // fetch the instruction and execute it
        let instruction = ((self.memory[self.program_counter] as u16) << 8) | (self.memory[self.program_counter + 1] as u16);
        let opcode = match OpCode::from_u16(instruction) {
            Some(o) => o,
            None => panic!("Error! Unimplemented opcode 0x{:4X}", instruction),
        };

        println!("{}", opcode.disasm_str);
        (opcode.operation)(&opcode.args, &mut *self);

        true
    }
}
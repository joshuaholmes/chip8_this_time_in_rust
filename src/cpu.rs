// 
// Author: Joshua Holmes
//

use rand;
use rand::distributions::{IndependentSample, Range};
use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::thread;
use std::time::{SystemTime, Duration};

use display::Display;
use keyboard::Keyboard;
use opcode::OpCode;

/// How many bytes of system memory there are
pub const MEMORY_LENGTH: usize = 0xFFF;
/// How many items our stack holds
pub const STACK_LENGTH: usize = 0x10;
/// The number of data registers we have
pub const NUM_REGISTERS: usize = 0x10;
/// The address at which the system's font data starts in memory
pub const FONT_SET_START_ADDR: usize = 0x000;
/// The address in memory where the user program begins
pub const USER_PROGRAM_START_ADDR: usize = 0x200;
/// The number of pixels in our virtual display width
pub const VIRTUAL_DISPLAY_WIDTH: usize = 64;
/// The number of pixels in our virtual display height
pub const VIRTUAL_DISPLAY_HEIGHT: usize = 32;
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
    pub i_register: usize,
    /// the delay timer, decreased at 60Hz by default when non-zero
    pub delay_timer: u8,
    /// the sound timer, decreased at 60Hz by default when non-zero
    pub sound_timer: u8,
    /// the program counter, points to the current instruction in memory
    pub program_counter: usize,
    /// the stack pointer, points to the current index in the stack
    pub stack_pointer: usize,
    /// the call stack, stores return addresses from subroutines
    pub stack: [usize; STACK_LENGTH],
    /// use this to know if the PC is past the end of the program
    pub program_length: usize,
    /// the system's "VRAM" -- the virtual screen buffer
    pub vram: [[bool; VIRTUAL_DISPLAY_WIDTH]; VIRTUAL_DISPLAY_HEIGHT],
    /// the flag that says whether we need to redraw the screen
    pub draw_flag: bool,
    /// the system's keyboard
    pub keyboard: Keyboard,
    /// the timestamp of the last timer decrement
    last_timer_decrease: SystemTime,
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
            stack: [0; STACK_LENGTH],
            program_length: buf.len(),
            last_timer_decrease: SystemTime::now(),
            vram: [[false; VIRTUAL_DISPLAY_WIDTH]; VIRTUAL_DISPLAY_HEIGHT],
            draw_flag: false,
            keyboard: Keyboard::new(),
        })
    }

    /// Fetches one opcode from memory and executes it.
    pub fn fetch_and_execute(&mut self, display: &mut Display) -> bool {
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

        //println!("{}", opcode.disasm_str);
        (opcode.operation)(&opcode.args, &mut *self);

        // see if we need to decrement the timers and draw the screen (both at 60Hz)
        let curr_time = SystemTime::now();

        match curr_time.duration_since(self.last_timer_decrease).unwrap().cmp(&Duration::new(0, 16_666_666)) {
            Ordering::Greater => {
                // decrement the timers
                if self.delay_timer > 0 {
                    self.delay_timer -= 1;
                }

                if self.sound_timer > 0 {
                    self.sound_timer -= 1;
                }

                self.last_timer_decrease = curr_time;
            },
            _ => ()
        }

        // refresh the screen, if necessary
        if self.draw_flag {
            display.draw_screen(&mut *self);
            self.draw_flag = false;
        }

        // terrible hack to make this thing run more slowly until proper timers are implemented
        thread::sleep(Duration::from_millis(2));

        true
    }

    /// Returns a random byte, used for the RND opcode
    pub fn get_random_byte(&self) -> u8 {
        let mut rng = rand::thread_rng();
        Range::new(0, 256).ind_sample(&mut rng) as u8
    } 
}
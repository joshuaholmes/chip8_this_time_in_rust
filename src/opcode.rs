//
// Author: Joshua Holmes
//

use cpu::Cpu;

// Represents a CHIP-8 opcode. Each opcode contains the actual
// opcode value read from memory (including arguments)
pub struct OpCode {
    opcode: u16,
    disasm_str: String,
    operation: fn(u16, Cpu),
}

impl OpCode {
    pub fn from_u16(val: u16) -> Option<OpCode> {
        None
    }
}
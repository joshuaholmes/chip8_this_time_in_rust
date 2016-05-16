//
// Author: Joshua Holmes
//

use std::fmt;

use cpu::Cpu;

/// Represents possible arguments to a CHIP-8 opcode. Not all argument
/// fields will be present for all opcodes.
#[derive(Debug, Copy, Clone)]
pub struct OpCodeArgs {
    /// Low nibble of the high byte (data register index)
    pub x: usize,
    /// High nibble of the low byte (data register index)
    pub y: usize,
    /// Lowest nibble (scalar value)
    pub n: u8,
    /// Lower byte (scalar value)
    pub kk: u8,
    /// Lower 3 nibbles (address)
    pub nnn: u16,
}

impl OpCodeArgs {
    /// Returns an OpCodeArgs structure given an opcode.
    pub fn from_u16(opcode: u16) -> OpCodeArgs {
        OpCodeArgs {
            x: ((opcode & 0x0F00) >> 8) as usize,
            y: ((opcode & 0x00F0) >> 4) as usize,
            n: (opcode & 0x000F) as u8,
            kk: (opcode & 0x00FF) as u8,
            nnn: opcode & 0x0FFF,
        }
    }
}

// Represents a CHIP-8 opcode. Each opcode contains the actual
// opcode value read from memory (including arguments)
pub struct OpCode {
    pub opcode: u16,
    pub args: OpCodeArgs,
    pub disasm_str: String,
    pub operation: fn(&OpCodeArgs, &mut Cpu),
}

impl OpCode {
    /// Contruct a new OpCode given its u16 opcode, its dissassembly string, and its operation delegate
    pub fn new(opcode: u16, args: OpCodeArgs, disasm_str: String, operation: fn(&OpCodeArgs, &mut Cpu)) -> OpCode {
        OpCode {
            opcode: opcode,
            args: args,
            disasm_str: disasm_str,
            operation: operation,
        }
    }

    /// Constructs a new OpCode object given a u16 opcode value
    pub fn from_u16(opcode: u16) -> Option<OpCode> {
        // get the opcode arguments and the first nibble then go 
        // down our lookups to determine which opcode this is
        let opcode_category = opcode & 0xF000;
        let args = OpCodeArgs::from_u16(opcode);

        match opcode_category {
            0x0000 => {
                match opcode {
                    0x00E0 => Some(OpCode::new(opcode, args, "CLS".to_owned(), OpCode::opcode_cls)),
                    0x00EE => Some(OpCode::new(opcode, args, "RET".to_owned(), OpCode::opcode_ret)),
                    _ => Some(OpCode::new(opcode, args, format!("SYS {:03X}", args.nnn), OpCode::opcode_sys))
                }
            },
            0x1000 => {
                Some(OpCode::new(opcode, args, format!("JP {:03X}", args.nnn), OpCode::opcode_jp_addr))
            },
            0x2000 => {
                Some(OpCode::new(opcode, args, format!("CALL {:03X}", args.nnn), OpCode::opcode_call_addr))
            },
            0x3000 => {
                Some(OpCode::new(opcode, args, format!("SE V{:X}, {:02X}", args.x, args.kk), OpCode::opcode_se_vx_byte))
            },
            0x4000 => {
                Some(OpCode::new(opcode, args, format!("SNE V{:X}, {:02X}", args.x, args.kk), OpCode::opcode_sne_vx_byte))
            },
            0x5000 => {
                Some(OpCode::new(opcode, args, format!("SE V{:X}, V{:X}", args.x, args.y), OpCode::opcode_sne_vx_vy))
            },
            0x6000 => {
                Some(OpCode::new(opcode, args, format!("LD V{:X}, {:02X}", args.x, args.kk), OpCode::opcode_ld_vx_byte))
            },
            0x7000 => {
                Some(OpCode::new(opcode, args, format!("ADD V{:X}, {:02X}", args.x, args.kk), OpCode::opcode_add_vx_byte))
            },
            0x8000 => {
                match args.n {
                    0x0 => Some(OpCode::new(opcode, args, format!("LD V{:X}, V{:X}", args.x, args.y), OpCode::opcode_ld_vx_vy)),
                    0x1 => Some(OpCode::new(opcode, args, format!("OR V{:X}, V{:X}", args.x, args.y), OpCode::opcode_or_vx_vy)),
                    0x2 => Some(OpCode::new(opcode, args, format!("AND V{:X}, V{:X}", args.x, args.y), OpCode::opcode_and_vx_vy)),
                    0x3 => Some(OpCode::new(opcode, args, format!("XOR V{:X}, V{:X}", args.x, args.y), OpCode::opcode_xor_vx_vy)),
                    0x4 => Some(OpCode::new(opcode, args, format!("ADD V{:X}, V{:X}", args.x, args.y), OpCode::opcode_add_vx_vy)),
                    0x5 => Some(OpCode::new(opcode, args, format!("SUB V{:X}, V{:X}", args.x, args.y), OpCode::opcode_sub_vx_vy)),
                    0x6 => Some(OpCode::new(opcode, args, format!("SHR V{:X}, V{:X}", args.x, args.y), OpCode::opcode_shr_vx_vy)),
                    0x7 => Some(OpCode::new(opcode, args, format!("SUBN V{:X}, V{:X}", args.x, args.y), OpCode::opcode_subn_vx_vy)),
                    0xE => Some(OpCode::new(opcode, args, format!("SHL V{:X}, V{:X}", args.x, args.y), OpCode::opcode_shl_vx_vy)),
                    _ => None
                }
            },
            0x9000 => {
                match args.n {
                    0x0 => Some(OpCode::new(opcode, args, format!("SNE V{:X}, V{:X}", args.x, args.y), OpCode::opcode_sne_vx_vy)),
                    _ => None
                }
            },
            0xA000 => {
                Some(OpCode::new(opcode, args, format!("LD I, {:03X}", args.nnn), OpCode::opcode_ld_i_addr))
            },
            0xB000 => {
                Some(OpCode::new(opcode, args, format!("JP V0, {:03X}", args.nnn), OpCode::opcode_jp_v0_addr))
            },
            0xC000 => {
                Some(OpCode::new(opcode, args, format!("RND V{:X}, {:02X}", args.x, args.kk), OpCode::opcode_rnd_vx_byte))
            },
            0xD000 => {
                Some(OpCode::new(opcode, args, format!("DRW V{:X}, V{:X}, {:X}", args.x, args.y, args.n), OpCode::opcode_drw_vx_vy_nibble))
            },
            0xE000 => {
                match args.kk {
                    0x9E => Some(OpCode::new(opcode, args, format!("SKP V{:X}", args.x), OpCode::opcode_skp_vx)),
                    0xA1 => Some(OpCode::new(opcode, args, format!("SKNP V{:X}", args.x), OpCode::opcode_sknp_vx)),
                    _ => None
                }
            },
            0xF000 => {
                match args.kk {
                    0x07 => Some(OpCode::new(opcode, args, format!("LD V{:X}, DT", args.x), OpCode::opcode_ld_vx_dt)),
                    0x0A => Some(OpCode::new(opcode, args, format!("LD V{:X}, K", args.x), OpCode::opcode_ld_vx_k)),
                    0x15 => Some(OpCode::new(opcode, args, format!("LD DT, V{:X}", args.x), OpCode::opcode_ld_dt_vx)),
                    0x18 => Some(OpCode::new(opcode, args, format!("LD ST, V{:X}", args.x), OpCode::opcode_ld_st_vx)),
                    0x1E => Some(OpCode::new(opcode, args, format!("ADD I, V{:X}", args.x), OpCode::opcode_add_i_vx)),
                    0x29 => Some(OpCode::new(opcode, args, format!("LD F, V{:X}", args.x), OpCode::opcode_ld_f_vx)),
                    0x33 => Some(OpCode::new(opcode, args, format!("LD B, V{:X}", args.x), OpCode::opcode_ld_b_vx)),
                    0x55 => Some(OpCode::new(opcode, args, format!("LD [I], V{:X}", args.x), OpCode::opcode_ld_i_vx)),
                    0x65 => Some(OpCode::new(opcode, args, format!("LD V{:X}, [I]", args.x), OpCode::opcode_ld_vx_i)),
                    _ => None
                }
            },
            _ => None
        }
    }

    // -------------------------------------------------------------
    // Below are the implementations for each of the opcodes. These
    // functions are the subjects of the function pointers in each
    // OpCode object.
    // -------------------------------------------------------------


    /// 0x0nnn
    /// "SYS addr" opcode. We don't *really* support this, nor does anyone else.
    fn opcode_sys(args: &OpCodeArgs, cpu: &mut Cpu) {
        println!("SYS opcode called.");
    }

    /// 0x00E0
    /// "CLS" opcode. Clears the display.
    fn opcode_cls(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x00EE
    /// "RET" opcode. Returns from a subroutine.
    fn opcode_ret(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x1nnn
    /// "JP addr" opcode. Jumps to a specified address.
    fn opcode_jp_addr(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x2nnn
    /// "CALL addr" opcode. Calls the subroutine at the given address.
    fn opcode_call_addr(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x3xkk
    /// "SE Vx, byte" opcode. Skip next instruction if Vx = kk.
    fn opcode_se_vx_byte(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x4xkk
    /// "SNE Vx, byte" opcode. Skip next instruction if Vx != kk.
    fn opcode_sne_vx_byte(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x5xy0
    /// "SE Vx, Vy" opcode. Skip next instruction if Vx = Vy.
    fn opcode_se_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x6xkk
    /// "LD Vx, byte" opcode. Set Vx = kk.
    fn opcode_ld_vx_byte(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x7xkk
    /// "ADD Vx, byte" opcode. Set Vx = Vx + kk.
    fn opcode_add_vx_byte(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy0
    /// "LD Vx, Vy" opcode. Set Vx = Vy.
    fn opcode_ld_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy1
    /// "OR Vx, Vy" opcode. Set Vx = Vx OR Vy.
    fn opcode_or_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy2
    /// "AND Vx, Vy" opcode. Set Vx = Vx AND Vy.
    fn opcode_and_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy3
    /// "XOR Vx, Vy" opcode. Set Vx = Vx XOR Vy.
    fn opcode_xor_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy4
    /// "ADD Vx, Vy" opcode. Set Vx = Vx + Vy, set VF = carry.
    fn opcode_add_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy5
    /// "SUB Vx, Vy" opcode. Set Vx = Vx - Vy, set VF = NOT borrow.
    fn opcode_sub_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy6
    /// "SHR Vx, Vy" opcode. Set Vx = Vy SHR 1.
    fn opcode_shr_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xy7
    /// "SUBN Vx, Vy" opcode. Set Vx = Vy - Vx, set VF = NOT borrow.
    fn opcode_subn_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x8xyE
    /// "SHL Vx, Vy" opcode. Set Vx = Vy SHL 1.
    fn opcode_shl_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0x9xy0
    /// "SNE Vx, Vy" opcode. Skip next instruction if Vx != Vy.
    fn opcode_sne_vx_vy(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xAnnn
    /// "LD I, addr" opcode. Set I = nnn.
    fn opcode_ld_i_addr(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xBnnn
    /// "JP V0, addr" opcode. Jump to location nnn + V0.
    fn opcode_jp_v0_addr(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xCxkk
    /// "RND Vx, byte" opcode. Set Vx = random byte AND kk.
    fn opcode_rnd_vx_byte(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xDxyn
    /// "DRW Vx, Vy, nibble" opcode. Display n-byte sprite starting at memory 
    /// location I at (Vx, Vy), set VF = collision.
    fn opcode_drw_vx_vy_nibble(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xEx9E
    /// "SKP Vx" opcode. Skip next instruction if key with the value of Vx is pressed.
    fn opcode_skp_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xExA1
    /// "SKNP Vx" opcode. Skip next instruction if key with the value of Vx is not pressed.
    fn opcode_sknp_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx07
    /// "LD Vx, DT" opcode. Set Vx = delay timer value.
    fn opcode_ld_vx_dt(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx0A
    /// "LD Vx, K" opcode. Wait for a key press, store the value of the key in Vx.
    fn opcode_ld_vx_k(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx15
    /// "LD DT, Vx" opcode. Set delay timer = Vx.
    fn opcode_ld_dt_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx18
    /// "LD ST, Vx" opcode. Set sound timer = Vx.
    fn opcode_ld_st_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx1E
    /// "ADD I, Vx" opcode. Set I = I + Vx.
    fn opcode_add_i_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx29
    /// "LD F, Vx" opcode. Set I = location of sprite for digit Vx.
    fn opcode_ld_f_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx33
    /// "LD B, Vx" opcode. Store BCD representation of Vx in memory locations I, I+1, and I+2.
    fn opcode_ld_b_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx55
    /// "LD [I], Vx" opcode. Store registers V0 through Vx in memory starting at location I.
    fn opcode_ld_i_vx(args: &OpCodeArgs, cpu: &mut Cpu) {

    }

    /// 0xFx65
    /// "LD Vx, [I]" opcode. Read registers V0 through Vx from memory starting at location I.
    fn opcode_ld_vx_i(args: &OpCodeArgs, cpu: &mut Cpu) {

    }
}
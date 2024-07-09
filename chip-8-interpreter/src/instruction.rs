use crate::{chip::Chip8Callback, memory::Memory, register::Registers};

pub struct Operands {
    pub nnn: u16,
    pub nibble: u8,
    pub x: u8,
    pub y: u8,
    pub kk: u8,
}

pub struct Instruction<'a> {
    disassembled: String,
    instruction: u16,
    pc: u16,
    operands: Operands,
    exec: Box<dyn FnMut(u16, &Operands, &mut Memory, &mut Memory, &mut Registers, &[bool], &mut [u8], &mut Chip8Callback<'a>) + 'a>,
}

impl<'a> Instruction<'a> {
    pub fn new(disassembled: String, instruction: u16, pc: u16) -> Self {
        // Les 12 bits de poids faible de l'instruction.
        let nnn = instruction & 0x0FFF;
        // Les 4 bits de poids faible de l'instruction.
        let nibble = (instruction & 0x000F) as u8;
        // Les 4 bits de poids faible sur l'octet de poids fort de l'instruction.
        let x = ((instruction & 0x0F00) >> 8) as u8;
        // Let 4 bits de poids fort sur l'octet de poids faible de l'instruction.
        let y = ((instruction & 0x00F0) >> 4) as u8;
        // Les 8 bits de poids faible de l'instruction.
        let kk = (instruction & 0x00FF) as u8;

        Self {
            disassembled,
            instruction,
            pc,
            operands: Operands { nnn, nibble, x, y, kk },
            exec: Box::new(unknown_instruction),
        }
    }

    pub fn execute(&mut self, ram: &mut Memory, stack: &mut Memory, reg: &mut Registers, keys: &[bool], screen: &mut [u8], callback: &mut Chip8Callback<'a>) {
        (self.exec)(self.instruction, &self.operands, ram, stack, reg, keys, screen, callback);
    }

    pub fn set_disassembled(&mut self, value: String) {
        self.disassembled = format!("{:04X} - {value}", self.pc);
    }

    pub fn get_disassembled(&self) -> &str {
        &self.disassembled
    }

    pub fn set_callback(&mut self, exec: impl FnMut(u16, &Operands, &mut Memory, &mut Memory, &mut Registers, &[bool], &mut [u8], &mut Chip8Callback<'a>) + 'a) {
        self.exec = Box::new(exec);
    }

    pub fn borrow_operands(&self) -> &Operands {
        &self.operands
    }
}

fn unknown_instruction(instruction: u16, _: &Operands, _: &mut Memory, _: &mut Memory, _: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    eprintln!("[CHIP-8] Unknown instruction: {instruction}");
}

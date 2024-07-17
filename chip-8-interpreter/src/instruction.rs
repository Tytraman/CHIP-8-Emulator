use rand::Rng;

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

pub(crate) fn clean_screen(_: u16, _: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], screen: &mut [u8], callbacks: &mut Chip8Callback) {
    (callbacks.clear_pixel)(&mut callbacks.callback_data);

    screen.fill(0);

    registers.pc += 2;
}

pub(crate) fn ret(_: u16, _: &Operands, _: &mut Memory, stack: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.sp -= 2;
    registers.pc = stack.read16(registers.sp as u16).unwrap() + 2;
}

pub(crate) fn jp_addr(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.pc = operands.nnn;
}

pub(crate) fn call_addr(_: u16, operands: &Operands, _: &mut Memory, stack: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    // Stock dans la pile l'adresse actuelle.
    if let Err(_err) = stack.write16(registers.sp as u16, registers.pc) {
        return (); // TODO: Err(err);
    }

    registers.sp += 2;

    registers.pc = operands.nnn;
}

pub(crate) fn se_reg_byte(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    if registers.v[operands.x as usize] == operands.kk {
        registers.pc += 4;
    } else {
        registers.pc += 2;
    }
}

pub(crate) fn sne_reg_byte(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    if registers.v[operands.x as usize] != operands.kk {
        registers.pc += 4;
    } else {
        registers.pc += 2;
    }
}

pub(crate) fn se_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    if registers.v[operands.x as usize] == registers.v[operands.y as usize] {
        registers.pc += 4;
    } else {
        registers.pc += 2;
    }
}

pub(crate) fn ld_reg_byte(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.v[operands.x as usize] = operands.kk;

    registers.pc += 2;
}

pub(crate) fn add_reg_byte(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.v[operands.x as usize] = registers.v[operands.x as usize].wrapping_add(operands.kk);

    registers.pc += 2;
}

pub(crate) fn ld_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.v[operands.x as usize] = registers.v[operands.y as usize];

    registers.pc += 2;
}

pub(crate) fn or_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.v[operands.x as usize] = registers.v[operands.x as usize] | registers.v[operands.y as usize];

    registers.pc += 2;
}

pub(crate) fn and_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.v[operands.x as usize] = registers.v[operands.x as usize] & registers.v[operands.y as usize];

    registers.pc += 2;
}

pub(crate) fn xor_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.v[operands.x as usize] = registers.v[operands.x as usize] ^ registers.v[operands.y as usize];

    registers.pc += 2;
}

pub(crate) fn add_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    let result = registers.v[operands.x as usize] as u16 + registers.v[operands.y as usize] as u16;

    registers.v[0xF] = (result > 255) as u8;
    registers.v[operands.x as usize] = (result & 0xFF) as u8;

    registers.pc += 2;
}

pub(crate) fn sub_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    // Si Vx > Vy, met la valeur de VF à 1.
    registers.v[0xF] = (registers.v[operands.x as usize] > registers.v[operands.y as usize]) as u8;

    registers.v[operands.x as usize] = registers.v[operands.x as usize].wrapping_sub(registers.v[operands.y as usize]);

    registers.pc += 2;
}

pub(crate) fn shr_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    // Récupère la valeur actuelle de Vx.
    let value = registers.v[operands.x as usize];

    // Si le bit de poids faible est à 1, met VF à 1.
    registers.v[0xF] = ((value & 0x1) > 0) as u8;

    // Décale de 1 bit vers la droite.

    registers.v[operands.x as usize] = value >> 1;

    registers.pc += 2;
}

pub(crate) fn subn_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    // Si Vy > Vx, met la valeur de VF à 1.
    registers.v[0xF] = (registers.v[operands.y as usize] > registers.v[operands.x as usize]) as u8;

    registers.v[operands.x as usize] = registers.v[operands.y as usize] - registers.v[operands.x as usize];

    registers.pc += 2;
}

pub(crate) fn shl_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    let value = registers.v[operands.x as usize];

    // Si le bit de poids fort est à 1, met VF à 1.
    registers.v[0xF] = ((value & 0x80) > 0) as u8;

    // Décale de Vy bits vers la gauche.
    registers.v[operands.x as usize] = value << 1;

    registers.pc += 2;
}

pub(crate) fn sne_reg_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    if registers.v[operands.x as usize] != registers.v[operands.y as usize] {
        registers.pc += 4;
    } else {
        registers.pc += 2;
    }
}

pub(crate) fn ld_i_addr(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.i = operands.nnn;

    registers.pc += 2;
}

pub(crate) fn jp_v0_addr(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.pc = operands.nnn + registers.v[0x0] as u16;
}

pub(crate) fn rnd_reg_byte(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(0..256) as u8;

    registers.v[operands.x as usize] = random_number & operands.kk;

    registers.pc += 2;
}

pub(crate) fn drw_reg_reg_nibble(_: u16, operands: &Operands, ram: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], screen: &mut [u8], callbacks: &mut Chip8Callback) {
    // Initialise le Carry Flag à 0.
    registers.v[0xF] = 0;

    if operands.x > 0xF || operands.y > 0xF {
        eprintln!("[CHIP-8] Error when drawing: x or y out of bound: {:02x} {:02x}", operands.x, operands.y);

        return ();
    }

    // Un sprite ne peut pas faire plus de 15 pixels de hauteur.
    if operands.nibble > 15 {
        eprintln!("[CHIP-8] Error when drawing: nibble is out of bound: {}", operands.nibble);

        return ();
    }

    // Pour chaque ligne du sprite à afficher.
    for row in 0..operands.nibble {
        // Si le pixel sort de l'écran vers le bas, le ramène en haut de l'écran.
        // 'wrapping_add' est une fonction qui permet d'ajouter un entier sans paniquer
        // s'il y a un overflow.
        let yy = (registers.v[operands.y as usize].wrapping_add(row)) % 32;

        let sprite = match ram.read8(registers.i + row as u16) {
            Ok(o) => o,
            Err(_err) => return (), // TODO: Err(err),
        };

        // Pour chaque bit de l'octet.
        for col in 0..8 {
            // Si le pixel sort de l'écran vers la droite, le ramène à gauche de
            // l'écran.
            let xx = (registers.v[operands.x as usize] + col) % 64;

            // Récupère l'état du pixel actuellement affiché à l'écran.
            let current_pixel =
                screen.get_mut(yy as usize * 64 + xx as usize).unwrap();

            // Le dernier décalement vers la droite permet de récupérer uniquement le
            // dernier bit.
            let sprite_bit = (sprite & (0x80 >> col)) >> (7 - col);

            // Si on veut allumer alors que c'est déjà allumé, on éteint.
            if (sprite_bit & *current_pixel) != 0 {
                // Le Carry Flag est mit à 1 lorsqu'un pixel est éteint car il y a une collision.
                registers.v[0xF] = 1;
            }

            // Les spécifications indiquent que le pixel actuel doit être XORed avec le
            // sprite.
            *current_pixel ^= sprite_bit;

            if *current_pixel != 0 {
                (callbacks.set_pixel)(&mut callbacks.callback_data, xx, yy);
            } else {
                (callbacks.unset_pixel)(&mut callbacks.callback_data, xx, yy);
            }
        }
    }

    registers.pc += 2;
}

pub(crate) fn skp_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, keys: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    if keys[registers.v[operands.x as usize] as usize] {
        registers.pc += 4;
    } else {
        registers.pc += 2;
    }
}

pub(crate) fn sknp_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, keys: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    if !keys[registers.v[operands.x as usize] as usize] {
        registers.pc += 4;
    } else {
        registers.pc += 2;
    }
}

pub(crate) fn ld_reg_dt(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.v[operands.x as usize] = registers.dt;

    registers.pc += 2;
}

pub(crate) fn ld_reg_k(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, keys: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    // Cela peut être n'importe quelle touche.
    if let Some(index) = keys.iter().position(|&pressed| pressed) {
        registers.v[operands.x as usize] = index as u8;

        registers.pc += 2;
    }
}

pub(crate) fn ld_dt_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.dt = registers.v[operands.x as usize];

    registers.pc += 2;
}

pub(crate) fn ld_st_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.st = registers.v[operands.x as usize];

    registers.pc += 2;
}

pub(crate) fn add_i_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    registers.i += registers.v[operands.x as usize] as u16;

    registers.pc += 2;
}

pub(crate) fn ld_i_reg(_: u16, operands: &Operands, _: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    // Comme les sprites sont stockées au tout début de la RAM, il n'y a pas besoin
    // de faire de calcul.
    registers.i = (registers.v[operands.x as usize] as u16) * 5;

    registers.pc += 2;
}

pub(crate) fn ld_b_reg(_: u16, operands: &Operands, ram: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    let mut value = registers.v[operands.x as usize];
                    
    if let Err(_err) = ram.write8(registers.i + 2, value % 10) {
        return (); // TODO: Err(err);
    }

    value /= 10;

    if let Err(_err) = ram.write8(registers.i + 1, value % 10) {
        return (); // TODO: Err(err);
    }

    value /= 10;
    
    if let Err(_err) = ram.write8(registers.i, value % 10) {
        return (); // TODO: Err(err);
    }

    registers.pc += 2;
}

pub(crate) fn ld_to_i_reg(_: u16, operands: &Operands, ram: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    for index in 0..operands.x + 1 {
        if let Err(err) = ram.write8(registers.i + index as u16, registers.v[index as usize]) {
            eprintln!("[CHIP-8 error] {err}");
            return (); // TODO: Err(err);
        }
    }

    registers.pc += 2;
}

pub(crate) fn ld_reg_from_i(_: u16, operands: &Operands, ram: &mut Memory, _: &mut Memory, registers: &mut Registers, _: &[bool], _: &mut [u8], _: &mut Chip8Callback) {
    for index in 0..operands.x + 1 {
        registers.v[index as usize] = match ram.read8(registers.i + index as u16) {
            Ok(o) => o,
            Err(err) => {
                eprintln!("[CHIP-8 error] {err}");
                return ();
             } // TODO: Err(err),
        }
    }

    registers.pc += 2;
}


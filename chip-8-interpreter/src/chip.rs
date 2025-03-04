use std::{
    any::Any,
    fs,
    time::{Duration, Instant},
};

use crate::{
    instruction::{self, Instruction},
    memory::Memory,
    register::Registers,
};

pub struct CallbackData {
    data: Option<Box<dyn Any>>,
}

impl CallbackData {
    pub fn new(value: Box<dyn Any>) -> Self {
        Self { data: Some(value) }
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        match &self.data {
            Some(t) => t.downcast_ref(),
            None => None,
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        // Le `mut` dans '&mut self.data' est nécessaire pour pouvoir récupérer le cast mutable
        // juste après.
        if let Some(t) = &mut self.data {
            return t.downcast_mut();
        }

        None
    }
}

pub struct Chip8Callback<'a> {
    pub(crate) clear_pixel: Box<dyn FnMut(&mut CallbackData) + 'a>,
    pub(crate) set_pixel: Box<dyn FnMut(&mut CallbackData, u8, u8) + 'a>,
    pub(crate) unset_pixel: Box<dyn FnMut(&mut CallbackData, u8, u8) + 'a>,
    pub(crate) callback_data: CallbackData,
}

impl<'a> Chip8Callback<'a> {
    pub fn set_callback_data(&mut self, callback_data: CallbackData) {
        self.callback_data = callback_data;
    }

    pub fn set_clear_pixel_callback(&mut self, c: impl FnMut(&mut CallbackData) + 'a) {
        self.clear_pixel = Box::new(c);
    }

    pub fn set_set_pixel_callback(&mut self, c: impl FnMut(&mut CallbackData, u8, u8) + 'a) {
        self.set_pixel = Box::new(c);
    }

    pub fn set_unset_pixel_callback(&mut self, c: impl FnMut(&mut CallbackData, u8, u8) + 'a) {
        self.unset_pixel = Box::new(c);
    }
}

pub struct Chip8<'a> {
    ram: Memory,
    stack: Memory,
    registers: Registers,
    screen: [u8; 64 * 32],
    keys: [bool; 0x10],
    paused: bool,
    callbacks: Chip8Callback<'a>,
    need_to_fetch: bool,
    next_instruction: Instruction<'a>,
    execution_instant: Instant,
}

fn add_hex_sprites(ram: &mut Memory) -> Result<(), String> {
    // Tableau qui contient les sprites des nombres hexadécimaux allant de 'O' à 'F'.
    let sprites: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
        0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80,
    ];

    ram.write8_range(0, sprites.len() as u16, &sprites)?;

    Ok(())
}

impl<'a> Chip8<'a> {
    pub fn build(program_name: &str) -> Result<Self, String> {
        // Lit le contenu du fichier et le stock dans un Vecteur u8.
        let content = match fs::read(program_name) {
            Ok(t) => t,
            Err(err) => return Err(err.to_string()),
        };

        println!("Program size: {}", content.len());

        let mut ram = Memory::new(0x1000);

        // Ajoute les sprites des nombres hexadécimaux.
        add_hex_sprites(&mut ram)?;

        // Copie le contenu du vecteur dans le buffer de la RAM.
        ram.write8_range(0x200, content.len() as u16 + 0x200, &content)?;

        Ok(Self {
            ram,
            stack: Memory::new(0x20),
            registers: Registers::new(),
            screen: [0; 64 * 32],
            keys: [false; 0x10],
            paused: true,
            callbacks: Chip8Callback {
                clear_pixel: Box::new(|_| {}),
                set_pixel: Box::new(|_, _, _| {}),
                unset_pixel: Box::new(|_, _, _| {}),
                callback_data: CallbackData { data: None },
            },
            need_to_fetch: true,
            next_instruction: Instruction::new(String::new(), 0x0000, 0x0000),
            execution_instant: Instant::now(),
        })
    }

    pub fn print_registers(&self) {
        println!(
            "[V0: {:02X}] [V1: {:02X}] [V2: {:02X}] [V3: {:02X}] [V4: {:02X}] [V5: {:02X}] [V6: {:02X}] [V7: {:02X}]\n[V8: {:02X}] [V9: {:02X}] [VA: {:02X}] [VB: {:02X}] [VC: {:02X}] [VD: {:02X}] [VE: {:02X}] [VF: {:02X}]\n[PC: {:04X}] [SP: {:02X}] [I: {:04X}] [DT: {:02X}] [ST: {:02X}]",
            self.registers.v[0],
            self.registers.v[1],
            self.registers.v[2],
            self.registers.v[3],
            self.registers.v[4],
            self.registers.v[5],
            self.registers.v[6],
            self.registers.v[7],
            self.registers.v[8],
            self.registers.v[9],
            self.registers.v[10],
            self.registers.v[11],
            self.registers.v[12],
            self.registers.v[13],
            self.registers.v[14],
            self.registers.v[15],
            self.registers.pc,
            self.registers.sp,
            self.registers.i,
            self.registers.dt,
            self.registers.st
        );
    }

    pub fn get_elapsed_time_since_last_instruction(&self) -> Duration {
        self.execution_instant.elapsed()
    }

    pub fn need_to_fetch(&self) -> bool {
        self.need_to_fetch
    }

    pub fn set_need_to_fetch(&mut self, value: bool) {
        self.need_to_fetch = value;
    }

    pub fn fetch_next_instruction(&self) -> Result<u16, String> {
        match self.ram.read16(self.registers.pc) {
            Ok(o) => Ok(o),
            Err(err) => return Err(err),
        }
    }

    pub fn decode_instruction(&mut self, instruction: u16) -> Result<&str, String> {
        let mut next_instruction = Instruction::new(String::new(), instruction, self.registers.pc);

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match instruction {
                    0x00E0 => {
                        // Nettoie l'écran.
                        next_instruction.set_disassembled("CLS".to_string());
                        next_instruction.set_callback(instruction::clean_screen);
                    }
                    0x00EE => {
                        // Retourne depuis une fonction.
                        next_instruction.set_disassembled("RET".to_string());
                        next_instruction.set_callback(instruction::ret);
                    }
                    _ => {
                        // Ignorée par les interpréteurs modernes.
                        next_instruction.set_disassembled(format!(
                            "SYS ${:04X}",
                            next_instruction.borrow_operands().nnn
                        ));
                    }
                }
            }
            0x1 => {
                // Met la valeur du registre PC à nnn.
                next_instruction.set_disassembled(format!(
                    "JP ${:04X}",
                    next_instruction.borrow_operands().nnn
                ));
                next_instruction.set_callback(instruction::jp_addr);
            }
            0x2 => {
                // Appelle la fonction située à l'adresse nnn.
                next_instruction.set_disassembled(format!(
                    "CALL ${:04X}",
                    next_instruction.borrow_operands().nnn
                ));
                next_instruction.set_callback(instruction::call_addr);
            }
            0x3 => {
                // Ignore la prochaine instruction si Vx == kk
                next_instruction.set_disassembled(format!(
                    "SE V{:01X}, {:02X}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().kk
                ));
                next_instruction.set_callback(instruction::se_reg_byte);
            }
            0x4 => {
                // Ignore la prochaine instruction si Vx != kk
                next_instruction.set_disassembled(format!(
                    "SNE V{:01X}, {:02X}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().kk
                ));
                next_instruction.set_callback(instruction::sne_reg_byte);
            }
            0x5 => {
                // Ignore la prochaine instruction si Vx == Vy
                next_instruction.set_disassembled(format!(
                    "SE V{:01X}, V{:01X}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().y
                ));
                next_instruction.set_callback(instruction::se_reg_reg);
            }
            0x6 => {
                // Met la valeur kk dans le registre Vx.
                next_instruction.set_disassembled(format!(
                    "LD V{:01X}, {:02X}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().kk
                ));
                next_instruction.set_callback(instruction::ld_reg_byte);
            }
            0x7 => {
                // Vx = Vx + kk
                next_instruction.set_disassembled(format!(
                    "ADD V{:01X}, {:02X}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().kk
                ));
                next_instruction.set_callback(instruction::add_reg_byte);
            }
            0x8 => match instruction & 0x000F {
                0x0 => {
                    // Vx = Vy
                    next_instruction.set_disassembled(format!(
                        "LD V{:01X}, V{:01X}",
                        next_instruction.borrow_operands().x,
                        next_instruction.borrow_operands().y
                    ));
                    next_instruction.set_callback(instruction::ld_reg_reg);
                }
                0x1 => {
                    // Vx = Vx | Vy
                    next_instruction.set_disassembled(format!(
                        "OR V{:01X}, V{:01X}",
                        next_instruction.borrow_operands().x,
                        next_instruction.borrow_operands().y
                    ));
                    next_instruction.set_callback(instruction::or_reg_reg);
                }
                0x2 => {
                    // Vx = Vx & Vy
                    next_instruction.set_disassembled(format!(
                        "AND V{:01X}, V{:01X}",
                        next_instruction.borrow_operands().x,
                        next_instruction.borrow_operands().y
                    ));
                    next_instruction.set_callback(instruction::and_reg_reg);
                }
                0x3 => {
                    // Vx = Vx ^ Vy
                    next_instruction.set_disassembled(format!(
                        "XOR V{:01X}, V{:01X}",
                        next_instruction.borrow_operands().x,
                        next_instruction.borrow_operands().y
                    ));
                    next_instruction.set_callback(instruction::xor_reg_reg);
                }
                0x4 => {
                    // Vx = Vx + Vy
                    next_instruction.set_disassembled(format!(
                        "ADD V{:01X}, V{:01X}",
                        next_instruction.borrow_operands().x,
                        next_instruction.borrow_operands().y
                    ));
                    next_instruction.set_callback(instruction::add_reg_reg);
                }
                0x5 => {
                    // Vx = Vx - Vy
                    next_instruction.set_disassembled(format!(
                        "SUB V{:01X}, V{:01X}",
                        next_instruction.borrow_operands().x,
                        next_instruction.borrow_operands().y
                    ));
                    next_instruction.set_callback(instruction::sub_reg_reg);
                }
                0x6 => {
                    // Vx = Vx >> Vy
                    next_instruction.set_disassembled(format!(
                        "SHR V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::shr_reg_reg);
                }
                0x7 => {
                    // Vx = Vy - Vx
                    next_instruction.set_disassembled(format!(
                        "SUBN V{:01X}, V{:01X}",
                        next_instruction.borrow_operands().x,
                        next_instruction.borrow_operands().y
                    ));
                    next_instruction.set_callback(instruction::subn_reg_reg);
                }
                0xE => {
                    // Vx = Vx << Vy
                    next_instruction.set_disassembled(format!(
                        "SHL V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::shl_reg_reg);
                }
                _ => (),
            },
            0x9 => {
                // Ignore la prochaine instruction si Vx != Vy
                next_instruction.set_disassembled(format!(
                    "SNE V{:01X}, V{:01X}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().y
                ));
                next_instruction.set_callback(instruction::sne_reg_reg);
            }
            0xA => {
                // Met la valeur du registre I à nnn.
                next_instruction.set_disassembled(format!(
                    "LD I, ${:04X}",
                    next_instruction.borrow_operands().nnn
                ));
                next_instruction.set_callback(instruction::ld_i_addr);
            }
            0xB => {
                // Saute à l'adresse nnn + V0
                next_instruction.set_disassembled(format!(
                    "JP V0, ${:04X}",
                    next_instruction.borrow_operands().nnn
                ));
                next_instruction.set_callback(instruction::jp_v0_addr);
            }
            0xC => {
                // Vx = random byte AND kk
                next_instruction.set_disassembled(format!(
                    "RND V{:01X}, {:02X}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().kk
                ));
                next_instruction.set_callback(instruction::rnd_reg_byte);
            }
            0xD => {
                next_instruction.set_disassembled(format!(
                    "DRW V{:01X}, V{:01X}, {}",
                    next_instruction.borrow_operands().x,
                    next_instruction.borrow_operands().y,
                    next_instruction.borrow_operands().nibble
                ));
                next_instruction.set_callback(instruction::drw_reg_reg_nibble);
            }
            0xE => match instruction & 0x00FF {
                0x9E => {
                    // Ignore l'instruction suivante si la touche Vx est appuyée.
                    next_instruction.set_disassembled(format!(
                        "SKP V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::skp_reg);
                }
                0xA1 => {
                    // Ignore l'instruction suivante si la touche Vx n'est pas appuyée.
                    next_instruction.set_disassembled(format!(
                        "SKNP V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::sknp_reg);
                }
                _ => {}
            },
            0xF => match instruction & 0x00FF {
                0x07 => {
                    // Vx = DT
                    next_instruction.set_disassembled(format!(
                        "LD V{:01X}, DT",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_reg_dt);
                }
                0x0A => {
                    // Attend qu'une touche soit pressée puis stock la valeur de la touche dans Vx.
                    // Instruction bloquante.
                    next_instruction.set_disassembled(format!(
                        "LD V{:01X}, K",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_reg_k);
                }
                0x15 => {
                    // DT = Vx
                    next_instruction.set_disassembled(format!(
                        "LD DT, V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_dt_reg);
                }
                0x18 => {
                    // ST = Vx
                    next_instruction.set_disassembled(format!(
                        "LD ST, V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_st_reg);
                }
                0x1E => {
                    // I = I + Vx
                    next_instruction.set_disassembled(format!(
                        "ADD I, V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::add_i_reg);
                }
                0x29 => {
                    // L'adresse vers le caractère Vx est stockée dans le registre I.
                    next_instruction.set_disassembled(format!(
                        "LD I, V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_i_reg);
                }
                0x33 => {
                    // Stock la représentation BCD de Vx dans les adresses à partir de I.
                    next_instruction.set_disassembled(format!(
                        "LD B, V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_b_reg);
                }
                0x55 => {
                    // Stock tous les registres à partir de V0 à Vx dans la mémoire à partir de l'adresse I.
                    next_instruction.set_disassembled(format!(
                        "LD [I], V{:01X}",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_to_i_reg);
                }
                0x65 => {
                    // Lit les registres de V0 à Vx depuis la mémoire à partir de l'adresse I.
                    next_instruction.set_disassembled(format!(
                        "LD V{:01X}, [I]",
                        next_instruction.borrow_operands().x
                    ));
                    next_instruction.set_callback(instruction::ld_reg_from_i);
                }
                _ => (),
            },
            _ => (),
        }

        self.next_instruction = next_instruction;

        Ok(self.next_instruction.get_disassembled())
    }

    pub fn execute_instruction(&mut self) {
        let period = 1.0_f64 / 60.0_f64;
        let nanos = period * 1_000_000_000.0_f64;

        self.next_instruction.execute(
            &mut self.ram,
            &mut self.stack,
            &mut self.registers,
            &self.keys,
            &mut self.screen,
            &mut self.callbacks,
        );

        // Décrémente le Delay Timer s'il a été défini.
        // Le timer a une fréquence de 60Hz.
        if self.registers.get_elapsed_time_since_last_dt() >= Duration::from_nanos(nanos as u64) {
            if self.registers.dt > 0 {
                self.registers.dt -= 1;
            }

            self.registers.reset_dt_time();
        }

        // Décrémente le Sound Timer s'il a été défini.
        // Le timer a une fréquence de 60Hz.
        if self.registers.get_elapsed_time_since_last_st() >= Duration::from_nanos(nanos as u64) {
            if self.registers.st > 0 {
                self.registers.st -= 1;
            }

            self.registers.reset_st_time();
        }

        self.execution_instant = Instant::now();
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_pause(&mut self, value: bool) {
        self.paused = value;
    }

    pub fn set_register_value(&mut self, register_number: u8, value: u8) -> Result<(), String> {
        let register = match self.registers.v.get_mut(register_number as usize) {
            Some(t) => t,
            None => return Err("Invalid register number".to_string()),
        };

        *register = value;

        Ok(())
    }

    pub fn set_key_pressed(&mut self, index: u8, value: bool) {
        if index > 0xF {
            return ();
        }

        self.keys[index as usize] = value;
    }

    pub fn borrow_mut_callbacks(&mut self) -> &mut Chip8Callback<'a> {
        &mut self.callbacks
    }

    pub fn borrow_next_instruction(&'a self) -> &Instruction {
        &self.next_instruction
    }
}

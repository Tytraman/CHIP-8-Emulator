use std::{any::Any, fs};

use rand::Rng;

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
    clear_pixel: Box<dyn FnMut(&mut CallbackData) + 'a>,
    set_pixel: Box<dyn FnMut(&mut CallbackData, u8, u8) + 'a>,
    unset_pixel: Box<dyn FnMut(&mut CallbackData, u8, u8) + 'a>,
    callback_data: CallbackData,
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
    ram: [u8; 0x1000],
    v: [u8; 0x10],
    stack: [u16; 0x10],
    pc: u16,
    sp: u8,
    i: u16,
    dt: u8,
    st: u8,
    screen: [u8; 64 * 32],
    keys: [bool; 0x10],
    paused: bool,
    callbacks: Chip8Callback<'a>,
}

fn add_hex_sprites(ram: &mut [u8; 0x1000]) {
    // Tableau qui contient les sprites des nombres hexadécimaux allant de 'O' à 'F'.
    let sprites: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
        0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80,
    ];

    // Copie les sprites dans la mémoire de l'interpréteur qui n'est pas utilisé par les
    // interpréteurs modernes.
    for (dest, from) in ram[..sprites.len()].iter_mut().zip(&sprites) {
        *dest = *from;
    }
}

impl<'a> Chip8<'a> {
    pub fn build(program_name: &str) -> Result<Self, String> {
        // Lit le contenu du fichier et le stock dans un Vecteur u8.
        let content = match fs::read(program_name) {
            Ok(t) => t,
            Err(err) => return Err(err.to_string()),
        };

        println!("Program size: {}", content.len());

        let mut ram: [u8; 0x1000] = [0; 0x1000];

        add_hex_sprites(&mut ram);

        // Copie le contenu du vecteur dans le buffer de la RAM.
        for (dest, from) in ram[0x200..content.len() + 0x200].iter_mut().zip(&content) {
            *dest = *from;
        }

        Ok(Self {
            ram,
            v: [0; 0x10],
            stack: [0; 0x10],
            pc: 0x200,
            sp: 0,
            i: 0,
            dt: 0,
            st: 0,
            screen: [0; 64 * 32],
            keys: [false; 0x10],
            paused: true,
            callbacks: Chip8Callback {
                clear_pixel: Box::new(|_| {}),
                set_pixel: Box::new(|_, _, _| {}),
                unset_pixel: Box::new(|_, _, _| {}),
                callback_data: CallbackData { data: None },
            },
        })
    }

    pub fn print_registers(&self) {
        println!(
            "[V0: {}] [V1: {}] [V2: {}] [V3: {}] [V4: {}] [V5: {}] [V6: {}] [V7: {}]\n[V8: {}] [V9: {}] [VA: {}] [VB: {}] [VC: {}] [VD: {}] [VE: {}] [VF: {}]\n[PC: {}] [SP: {}] [I: {}] [DT: {}] [ST: {}]",
            self.v[0],
            self.v[1],
            self.v[2],
            self.v[3],
            self.v[4],
            self.v[5],
            self.v[6],
            self.v[7],
            self.v[8],
            self.v[9],
            self.v[10],
            self.v[11],
            self.v[12],
            self.v[13],
            self.v[14],
            self.v[15],
            self.pc,
            self.sp,
            self.i,
            self.dt,
            self.st
        );
    }

    pub fn execute_next_instruction(&mut self) {
        let msb = self.ram[self.pc as usize];
        let lsb = self.ram[(self.pc + 1) as usize];

        // L'instruction à exécuter, elle est stockée sur 2 octets.
        let opcode = (lsb as u16) | ((msb as u16) << 8);

        // Les 12 bits de poids faible de l'instruction.
        let nnn = opcode & 0x0FFF;
        // Les 4 bits de poids faible de l'instruction.
        let nibble = (opcode & 0x000F) as u8;
        // Les 4 bits de poids faible sur l'octet de poids fort de l'instruction.
        let x = ((opcode & 0x0F00) >> 8) as u8;
        // Let 4 bits de poids fort sur l'octet de poids faible de l'instruction.
        let y = ((opcode & 0x00F0) >> 4) as u8;
        // Les 8 bits de poids faible de l'instruction.
        let kk = (opcode & 0x00FF) as u8;

        let mut message = format!("Unimplemented instruction: {opcode:04x}");

        match (opcode & 0xF000) >> 12 {
            0x0 => {
                match opcode {
                    0x00E0 => {
                        // Nettoie l'écran.
                        message = "CLS".to_string();

                        (self.callbacks.clear_pixel)(&mut self.callbacks.callback_data);

                        self.pc += 2;
                    }
                    0x00EE => {
                        // Retourne depuis une fonction.
                        message = "RET".to_string();

                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize] + 2;
                    }
                    _ => {
                        // Ignorée par les interpréteurs modernes.
                        message = format!("SYS {nnn:03x}");
                    }
                }
            }
            0x1 => {
                // Met la valeur du registre PC à nnn.
                message = format!("JP {nnn:03x}");

                self.pc = nnn;
            }
            0x2 => {
                // Appelle la fonction située à l'adresse nnn.
                message = format!("CALL {nnn:03x}");

                // Stock dans la pile l'adresse actuelle.
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;

                self.pc = nnn;
            }
            0x3 => {
                // Ignore la prochaine instruction si Vx == kk
                message = format!("SE V{x:01x}, {kk:02x}");

                if self.v[x as usize] == kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4 => {
                // Ignore la prochaine instruction si Vx != kk
                message = format!("SNE V{x:01x}, {kk:02x}");

                if self.v[x as usize] != kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5 => {
                // Ignore la prochaine instruction si Vx == Vy
                message = format!("SE V{x:01x}, V{y:01x}");

                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                // Met la valeur kk dans le registre Vx.
                message = format!("LD V{x:01x}, {kk:02x}");

                self.v[x as usize] = kk;

                self.pc += 2;
            }
            0x7 => {
                // Vx = Vx + kk
                message = format!("ADD V{x:01x}, {kk:02x}");

                self.v[x as usize] = self.v[x as usize].wrapping_add(kk);

                self.pc += 2;
            }
            0x8 => match opcode & 0x000F {
                0x0 => {
                    // Vx = Vy
                    message = format!("LD V{x:01x}, V{y:01x}");

                    self.v[x as usize] = self.v[y as usize];

                    self.pc += 2;
                }
                0x1 => {
                    // Vx = Vx | Vy
                    message = format!("OR V{x:01x}, V{y:01x}");

                    self.v[x as usize] = self.v[x as usize] | self.v[y as usize];

                    self.pc += 2;
                }
                0x2 => {
                    // Vx = Vx & Vy
                    message = format!("AND V{x:01x}, V{y:01x}");

                    self.v[x as usize] = self.v[x as usize] & self.v[y as usize];

                    self.pc += 2;
                }
                0x3 => {
                    // Vx = Vx ^ Vy
                    message = format!("XOR V{x:01x}, V{y:01x}");

                    self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];

                    self.pc += 2;
                }
                0x4 => {
                    // Vx = Vx + Vy
                    message = format!("ADD V{x:01x}, V{y:01x}");

                    let result = self.v[x as usize] as u16 + self.v[y as usize] as u16;

                    self.v[0xF] = (result > 255) as u8;
                    self.v[x as usize] = result as u8;

                    self.pc += 2;
                }
                0x5 => {
                    // Vx = Vx - Vy
                    message = format!("SUB V{x:01x}, V{y:01x}");

                    // Si Vx > Vy, met la valeur de VF à 1.
                    self.v[0xF] = (self.v[x as usize] > self.v[y as usize]) as u8;

                    self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);

                    self.pc += 2;
                }
                0x6 => {
                    // Vx = Vx >> Vy
                    message = format!("SHR V{x:01x}, V{y:01x}");

                    let mut value = self.v[x as usize];

                    // Si le bit de poids faible est à 1, met VF à 1.
                    self.v[0xF] = ((value & 0x1) > 0) as u8;

                    // Décale de Vy bits vers la droite.
                    value >>= self.v[y as usize];

                    self.v[x as usize] = value;

                    self.pc += 2;
                }
                0x7 => {
                    // Vx = Vy - Vx
                    message = format!("SUBN V{x:01x}, V{y:01x}");

                    // Si Vy > Vx, met la valeur de VF à 1.
                    self.v[0xF] = (self.v[y as usize] > self.v[x as usize]) as u8;

                    self.v[x as usize] = self.v[y as usize] - self.v[x as usize];

                    self.pc += 2;
                }
                0x8 => {
                    // Vx = Vx << Vy
                    message = format!("SHL V{x:01x}, V{y:01x}");

                    let mut value = self.v[x as usize];

                    // Si le bit de poids fort est à 1, met VF à 1.
                    self.v[0xF] = ((value & 0x80) > 0) as u8;

                    // Décale de Vy bits vers la gauche.
                    value <<= self.v[y as usize];

                    self.v[x as usize] = value;

                    self.pc += 2;
                }
                _ => (),
            },
            0x9 => {
                // Ignore la prochaine instruction si Vx != Vy
                message = format!("SNE V{x:01x}, V{y:01x}");

                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA => {
                // Met la valeur du registre I à nnn.
                message = format!("LD I, {nnn:03x}");

                self.i = nnn;

                self.pc += 2;
            }
            0xB => {
                // Saute à l'adresse nnn + V0
                message = format!("JP V0, {nnn:03x}");

                self.pc = nnn + self.v[0x0] as u16;
            }
            0xC => {
                // Vx = random byte AND kk
                message = format!("RND V{x:01x}, {kk:02x}");

                let mut rng = rand::thread_rng();

                let random_number = rng.gen_range(0..256) as u8;

                self.v[x as usize] = random_number & kk;

                self.pc += 2;
            }
            0xD => 'draw: {
                message = format!("DRW V{x:01x}, V{y:01x}, {nibble}");

                // Initialise le Carry Flag à 0.
                self.v[0xF] = 0;

                if x > 0xF || y > 0xF {
                    eprintln!("[CHIP-8] Error when drawing: x or y out of bound: {x:02x} {y:02x}");

                    break 'draw;
                }

                // Un sprite ne peut pas faire plus de 15 pixels de hauteur.
                if nibble > 15 {
                    eprintln!("[CHIP-8] Error when drawing: nibble is out of bound: {nibble}");

                    break 'draw;
                }

                // Pour chaque ligne du sprite à afficher.
                for row in 0..nibble {
                    // Si le pixel sort de l'écran vers le bas, le ramène en haut de l'écran.
                    // 'wrapping_add' est une fonction qui permet d'ajouter un entier sans paniquer
                    // s'il y a un overflow.
                    let yy = (self.v[y as usize].wrapping_add(row)) % 32;

                    let sprite = self.ram[(self.i + row as u16) as usize];

                    // Pour chaque bit de l'octet.
                    for col in 0..8 {
                        // Si le pixel sort de l'écran vers la droite, le ramène à gauche de
                        // l'écran.
                        let xx = (self.v[x as usize] + col) % 64;

                        // Récupère l'état du pixel actuellement affiché à l'écran.
                        let current_pixel =
                            self.screen.get_mut(yy as usize * 64 + xx as usize).unwrap();

                        // Le dernier décalement vers la droite permet de récupérer uniquement le
                        // dernier bit.
                        let sprite_bit = (sprite & (0x80 >> col)) >> (7 - col);

                        // Si on veut allumer alors que c'est déjà allumé, on éteint.
                        if (sprite_bit & *current_pixel) != 0 {
                            // Le Carry Flag est mit à 1 lorsqu'un pixel est éteint.
                            self.v[0xF] = 1;
                        }

                        // Les spécifications indiquent que le pixel actuel doit être XORed avec le
                        // sprite.
                        *current_pixel ^= sprite_bit;

                        if *current_pixel != 0 {
                            (self.callbacks.set_pixel)(&mut self.callbacks.callback_data, xx, yy);
                        } else {
                            (self.callbacks.unset_pixel)(&mut self.callbacks.callback_data, xx, yy);
                        }
                    }
                }

                self.pc += 2;
            }
            0xE => match opcode & 0x00FF {
                0x9E => {
                    // Ignore l'instruction suivante si la touche Vx est appuyée.
                    message = format!("SKP V{x:01x}");

                    if self.keys[self.v[x as usize] as usize] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    // Ignore l'instruction suivante si la touche Vx n'est pas appuyée.
                    message = format!("SKNP V{x:01x}");

                    if !self.keys[self.v[x as usize] as usize] {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => {}
            },
            0xF => match opcode & 0x00FF {
                0x07 => {
                    // Vx = DT
                    message = format!("LD V{x:01x}, DT");

                    self.v[x as usize] = self.dt;

                    self.pc += 2;
                }
                0x0A => {
                    // Attend qu'une touche soit pressée puis stock la valeur de la touche dans Vx.
                    // Instruction bloquante.
                    message = format!("LD V{x:01x}, K");

                    // Cela peut être n'importe quelle touche.

                    if let Some(index) = self.keys.iter().position(|&pressed| pressed) {
                        self.v[x as usize] = index as u8;

                        self.pc += 2;
                    }
                }
                0x15 => {
                    // DT = Vx
                    message = format!("LD DT, V{x:01x}");

                    self.dt = self.v[x as usize];

                    self.pc += 2;
                }
                0x18 => {
                    // ST = Vx
                    message = format!("LD ST, V{x:01x}");

                    self.st = self.v[x as usize];

                    self.pc += 2;
                }
                0x1E => {
                    // I = I + Vx
                    message = format!("ADD I, V{x:01x}");

                    self.i += self.v[x as usize] as u16;

                    self.pc += 2;
                }
                0x29 => {
                    // L'adresse vers le caractère Vx est sotckée dans le registre I.
                    message = format!("LD F, V{x:01x}");

                    // Comme les sprites sont stockées au tout début de la RAM, il n'y a pas besoin
                    // de faire de calcul.
                    self.i = self.v[x as usize] as u16;

                    self.pc += 2;
                }
                0x33 => {
                    // Stock la représentation BCD de Vx dans les adresses à partir de I.
                    message = format!("LD B, V{x:01x}");

                    let mut value = self.v[x as usize];

                    self.ram[self.i as usize] = value % 10;

                    value /= 10;

                    self.ram[(self.i + 1) as usize] = value % 10;

                    value /= 10;

                    self.ram[(self.i + 2) as usize] = value % 10;

                    self.pc += 2;
                }
                0x55 => {
                    // Stock tous les registres à partir de V0 à Vx dans la mémoire à partir de l'adresse I.
                    message = format!("LD [I], V{x:01x}");

                    for index in 0..x + 1 {
                        self.ram[(self.i + index as u16) as usize] = self.v[index as usize];
                    }

                    self.pc += 2;
                }
                0x65 => {
                    // Lit les registres de V0 à Vx depuis la mémoire à partir de l'adresse I.
                    message = format!("LD V{x:01x}, [I]");

                    for index in 0..x + 1 {
                        self.v[index as usize] = self.ram[(self.i + index as u16) as usize];
                    }

                    self.pc += 2;
                }
                _ => (),
            },
            _ => (),
        }

        // Décrémente le Delay Timer s'il a été défini.
        if self.dt > 0 {
            self.dt -= 1;
        }

        // Décrémente le Sound Timer s'il a été défini.
        if self.st > 0 {
            self.st -= 1;
        }

        println!("[CHIP-8] {message}");
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_pause(&mut self, value: bool) {
        self.paused = value;
    }

    pub fn set_register_value(&mut self, register_number: u8, value: u8) -> Result<(), String> {
        let register = match self.v.get_mut(register_number as usize) {
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
}

use std::time::Duration;

use chip_8_interpreter::chip::Chip8;
use graph_punk::{
    types::UserData,
    window::user_input::{KeyStatus, Keys},
};

use crate::Config;

fn check_key_state<'a>(keys: &Keys, key: &str, mut c: impl FnMut(KeyStatus, KeyStatus) + 'a) {
    if let Some((pressed, last_state)) = keys.get_key_status(key) {
        (c)(pressed, last_state);
    }
}

pub fn update_callback(keys: &Keys, user_data: &mut UserData) {
    let (config, chip8) = match user_data.get_mut::<(Config, Chip8)>() {
        Some(t) => t,
        None => {
            eprintln!("Cannot get CHIP-8 in update callback.");
            return ();
        }
    };

    // Vérifie si l'utilisateur appuie sur l'une des touches du CHIP-8.
    check_key_state(keys, "1", |pressed, _| {
        chip8.set_key_pressed(0x1, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "2", |pressed, _| {
        chip8.set_key_pressed(0x2, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "3", |pressed, _| {
        chip8.set_key_pressed(0x3, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "4", |pressed, _| {
        chip8.set_key_pressed(0xC, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "a", |pressed, _| {
        chip8.set_key_pressed(0x4, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "z", |pressed, _| {
        chip8.set_key_pressed(0x5, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "e", |pressed, _| {
        chip8.set_key_pressed(0x6, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "r", |pressed, _| {
        chip8.set_key_pressed(0xD, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "q", |pressed, _| {
        chip8.set_key_pressed(0x7, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "s", |pressed, _| {
        chip8.set_key_pressed(0x8, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "d", |pressed, _| {
        chip8.set_key_pressed(0x9, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "f", |pressed, _| {
        chip8.set_key_pressed(0xE, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "w", |pressed, _| {
        chip8.set_key_pressed(0xA, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "x", |pressed, _| {
        chip8.set_key_pressed(0x0, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "c", |pressed, _| {
        chip8.set_key_pressed(0xB, matches!(pressed, KeyStatus::Pressed))
    });
    check_key_state(keys, "v", |pressed, _| {
        chip8.set_key_pressed(0xF, matches!(pressed, KeyStatus::Pressed))
    });

    // Vérifie si l'utilisateur switch entre le mode "instruction par instruction" et "instructions
    // automatiques".
    check_key_state(keys, "p", |pressed, last_state| {
        if matches!(pressed, KeyStatus::Pressed) && matches!(last_state, KeyStatus::Released) {
            config.auto_next_instruction = !config.auto_next_instruction;

            if config.auto_next_instruction {
                println!("[CHIP-8] Running instructions automatically.");
            } else {
                println!("[CHIP-8] Running instruction per instruction.");
            }
        }
    });

    // Si l'émulateur est en mode "instruction par instruction".
    if !config.auto_next_instruction {
        check_key_state(keys, "n", |pressed, last_state| {
            if matches!(pressed, KeyStatus::Pressed) && matches!(last_state, KeyStatus::Released) {
                chip8.set_pause(false);
            } else {
                chip8.set_pause(true);
            }
        });
    }

    // Vérifie si l'utilisateur met pause à l'interpréteur.
    check_key_state(keys, " ", |pressed, last_state| {
        if matches!(pressed, KeyStatus::Pressed) && matches!(last_state, KeyStatus::Released) {
            // L'utilisateur ne peut mettre pause que si le mode est "instructions automatiques".
            if config.auto_next_instruction {
                chip8.set_pause(!chip8.is_paused());

                if chip8.is_paused() {
                    println!("[CHIP-8] Interpreter paused.");
                } else {
                    println!("[CHIP-8] Interpreter resumed.");
                }
            }
        }
    });

    if chip8.need_to_fetch() {
        // Récupère l'instruction suivante.
        let ins = match chip8.fetch_next_instruction() {
            Ok(o) => o,
            Err(err) => {
                eprintln!("[CHIP-8 error] {err}");

                return ();
            }
        };

        {
            // Décode l'instruction à exécuter.
            let disassembly = match chip8.decode_instruction(ins) {
                Ok(o) => o,
                Err(err) => {
                    eprintln!("[CHIP-8 error] Decode instruction: {err}");

                    return ();
                }
            };

            println!("[CHIP-8] {disassembly}");
        }

        chip8.set_need_to_fetch(false);
    }

    if chip8.is_paused() {
        // Il n'est possible d'afficher la valeur des registres que si l'interpréteur est en pause.
        check_key_state(keys, "o", |pressed, last_state| {
            if matches!(pressed, KeyStatus::Pressed) && matches!(last_state, KeyStatus::Released) {
                println!("[CHIP-8] Printing registers:");

                chip8.print_registers();
            }
        });
    } else {
        let period = 1.0_f64 / 500.0_f64;
        let nanos = period * 1_000_000_000.0_f64;

        if chip8.get_elapsed_time_since_last_instruction() >= Duration::from_nanos(nanos as u64) {
            // Exécute l'instruction.
            chip8.execute_instruction();
            chip8.set_need_to_fetch(true);
        }
    }
}

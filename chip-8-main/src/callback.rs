use std::{cell::RefCell, rc::Rc};

use chip_8_interpreter::chip::Chip8;
use chip_8_rendering::{
    renderer::{KeyStatus, RendererParams},
    window::UserData,
};

use crate::Config;

fn check_key_state<'a>(renderer_params: &Rc<RefCell<RendererParams>>, key: &str, mut c: impl FnMut(KeyStatus, KeyStatus) + 'a) {
    if let Some((pressed, last_state)) = renderer_params
        .borrow_mut()
        .renderer
        .get_key_status(key)
    {
        (c)(pressed, last_state);
    }
}

pub fn update_callback(renderer_params: Rc<RefCell<RendererParams>>, user_data: &mut UserData) {
    let (config, chip8) = match user_data.get_mut::<(Config, Chip8)>() {
        Some(t) => t,
        None => {
            eprintln!("Cannot get CHIP-8 in update callback.");
            return ();
        }
    };

    // Vérifie si l'utilisateur appuie sur l'une des touches du CHIP-8.
    check_key_state(&renderer_params, "1", |pressed, _| chip8.set_key_pressed(0x1, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "2", |pressed, _| chip8.set_key_pressed(0x2, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "3", |pressed, _| chip8.set_key_pressed(0x3, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "4", |pressed, _| chip8.set_key_pressed(0xC, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "a", |pressed, _| chip8.set_key_pressed(0x4, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "z", |pressed, _| chip8.set_key_pressed(0x5, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "e", |pressed, _| chip8.set_key_pressed(0x6, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "r", |pressed, _| chip8.set_key_pressed(0xD, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "q", |pressed, _| chip8.set_key_pressed(0x7, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "s", |pressed, _| chip8.set_key_pressed(0x8, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "d", |pressed, _| chip8.set_key_pressed(0x9, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "f", |pressed, _| chip8.set_key_pressed(0xE, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "w", |pressed, _| chip8.set_key_pressed(0xA, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "x", |pressed, _| chip8.set_key_pressed(0x0, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "c", |pressed, _| chip8.set_key_pressed(0xB, matches!(pressed, KeyStatus::Pressed)));
    check_key_state(&renderer_params, "v", |pressed, _| chip8.set_key_pressed(0xF, matches!(pressed, KeyStatus::Pressed)));

    // Vérifie si l'utilisateur switch entre le mode "instruction par instruction" et "instructions
    // automatiques".
    check_key_state(&renderer_params, "p", |pressed, last_state| {
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
        check_key_state(&renderer_params, "n", |pressed, last_state| {
            if matches!(pressed, KeyStatus::Pressed) && matches!(last_state, KeyStatus::Released) {
                chip8.set_pause(false);
            } else {
                chip8.set_pause(true);
            }
        });
    }

    // Vérifie si l'utilisateur met pause à l'interpréteur.
    check_key_state(&renderer_params, " ", |pressed, last_state| {
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

    if !chip8.is_paused() {
        chip8.execute_next_instruction();
    } else {
        // Il n'est possible d'afficher la valeur des registres que si l'interpréteur est en pause.
        check_key_state(&renderer_params, "o", |pressed, last_state| {
            if matches!(pressed, KeyStatus::Pressed) && matches!(last_state, KeyStatus::Released) {
                println!("[CHIP-8] Printing registers:");

                chip8.print_registers();
            }
        });
    }
}

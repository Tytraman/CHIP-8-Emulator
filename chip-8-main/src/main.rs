mod callback;

use std::{cell::RefCell, env, rc::Rc};

use callback::update_callback;
use chip_8_interpreter::chip::{CallbackData, Chip8};
use chip_8_rendering::{renderer::RendererManager, types::UserData, window::window::Window};

pub struct Config {
    pub auto_next_instruction: bool,
    pub program_name: String,
}

fn main() -> Result<(), String> {
    println!("=====[ CHIP-8 emulator ]=====");

    let config = match process_args(env::args()) {
        Ok(t) => t,
        Err(err) => return Err(format!("Parsing arguments: {err}")),
    };

    println!("Loading program \"{}\"...", config.program_name);

    let renderer_manager = RendererManager::new();

    let mut chip8 = Chip8::build(&format!("Builtin/Programs/{}", config.program_name))?;

    let mut window = match Window::new("CHIP-8 emulator", 700, 400, &mut renderer_manager.borrow_mut()) {
        Ok(w) => w,
        Err(e) => return Err(e),
    };

    if let Some(renderer) = renderer_manager.borrow_mut().borrow_mut_renderer(window.get_renderer_name()) {
        match renderer.init_resources() {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }
    } else {
        return Err(format!("Cannot find renderer named \"{}\"", window.get_renderer_name()));
    }

    

    let callbacks = chip8.borrow_mut_callbacks();

    callbacks.set_callback_data(CallbackData::new(Box::new(Rc::clone(&renderer_manager))));

    callbacks.set_clear_pixel_callback(|callback_data| {
        if let Some(renderer_manager) = callback_data.get::<Rc<RefCell<RendererManager>>>() {
            if let Some(renderer) = renderer_manager.borrow_mut().borrow_mut_renderer("0") {
                let _ = renderer.clear_grid_pixel();
            }
        }
    });

    callbacks.set_set_pixel_callback(|callback_data, x, y| {
        if let Some(renderer_manager) = callback_data.get::<Rc<RefCell<RendererManager>>>() {
            if let Some(renderer) = renderer_manager.borrow_mut().borrow_mut_renderer("0") {
                let _ = renderer.set_grid_pixel(x as usize, y as usize, true);
            }
        }
    });

    callbacks.set_unset_pixel_callback(|callback_data, x, y| {
        if let Some(renderer_manager) = callback_data.get::<Rc<RefCell<RendererManager>>>() {
            if let Some(renderer) = renderer_manager.borrow_mut().borrow_mut_renderer("0") {
                let _ = renderer.set_grid_pixel(x as usize, y as usize, false);
            }
        }
    });

    let user_data = UserData::new(Box::new((config, chip8)));

    window.set_update_callback(update_callback, user_data);

    println!("Emulator is ready!");
    match window.run(renderer_manager) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    println!("Good-bye!");

    Ok(())
}

fn process_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    // Saute le 1er argument car c'est normalement le nom du programme.
    args.next();

    let mut program_name = String::new();

    // Boucle tant qu'il y a reste des arguments.
    while let Some(arg) = args.next() {
        // 'match' ne peut pas opérer sur des 'String' directement, il faut donc récupérer le
        // 'str'.
        match &arg[..] {
            // Lit le nom du jeu à ouvrir.
            "--program" | "-p" => {
                // Il faut qu'il y ai un argument après celui-ci qui réfère au nom.
                if let Some(name) = args.next() {
                    program_name = name;

                    if let Some(index) = program_name.chars().rev().position(|c| c == '.') {
                        program_name = program_name[..program_name.len() - index - 1].to_string();
                    }
                } else {
                    return Err("no program name specified after --program argument".to_string());
                }
            }
            _ => (),
        }
    }

    if program_name.is_empty() {
        return Err("no program name provided".to_string());
    }

    program_name.push_str(".ch8");

    Ok(Config {
        auto_next_instruction: false,
        program_name,
    })
}

mod callback;

use std::{cell::RefCell, env, rc::Rc};

use callback::update_callback;
use chip_8_interpreter::chip::{CallbackData, Chip8};
use graph_punk::{maths::vec::Vec2, message::MessageCaller, types::UserData, GraphPunk};

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

    let mut graph_punk = GraphPunk::new();

    graph_punk.create_window("chip8_window", "CHIP-8 emulator", 700, 400)?;
    graph_punk.init_basic_resources()?;

    let mut chip8 = Chip8::build(&format!("Builtin/Programs/{}", config.program_name))?;

    graph_punk.window_set_display_size("chip8_window", Vec2 { x: 64, y: 32 })?;

    let callbacks = chip8.borrow_mut_callbacks();

    let mut message_caller = MessageCaller::default();
    message_caller.register_message("clear_pixel", |renderer, _, drawing_objects, _| {
        let _ = renderer.clear_grid_pixel(drawing_objects);
    });

    message_caller.register_message("set_pixel", |renderer, _, drawing_objects, user_data| {
        if let Some((x, y)) = user_data.get::<(usize, usize)>() {
            let _ = renderer.set_grid_pixel(drawing_objects, *x, *y, true);
        }
    });

    message_caller.register_message("unset_pixel", |renderer, _, drawing_objects, user_data| {
        if let Some((x, y)) = user_data.get::<(usize, usize)>() {
            let _ = renderer.set_grid_pixel(drawing_objects, *x, *y, false);
        }
    });

    let message_caller = Rc::new(RefCell::new(message_caller));

    callbacks.set_callback_data(CallbackData::new(Box::new(Rc::clone(&message_caller))));

    callbacks.set_clear_pixel_callback(|callback_data| {
        if let Some(rc_message_caller) = callback_data.get::<Rc<RefCell<MessageCaller>>>() {
            let mut borrowed_message_caller = rc_message_caller.borrow_mut();

            let _ = borrowed_message_caller.add_message("clear_pixel", UserData::default());
        }
    });

    callbacks.set_set_pixel_callback(|callback_data, x, y| {
        if let Some(rc_message_caller) = callback_data.get::<Rc<RefCell<MessageCaller>>>() {
            let mut borrowed_message_caller = rc_message_caller.borrow_mut();

            let _ = borrowed_message_caller.add_message(
                "set_pixel",
                UserData::new(Box::new((x as usize, y as usize))),
            );
        }
    });

    callbacks.set_unset_pixel_callback(|callback_data, x, y| {
        if let Some(rc_message_caller) = callback_data.get::<Rc<RefCell<MessageCaller>>>() {
            let mut borrowed_message_caller = rc_message_caller.borrow_mut();

            let _ = borrowed_message_caller.add_message(
                "unset_pixel",
                UserData::new(Box::new((x as usize, y as usize))),
            );
        }
    });

    let user_data = UserData::new(Box::new((config, chip8)));

    graph_punk.window_set_update_callback("chip8_window", update_callback, user_data)?;

    println!("Emulator is ready!");
    graph_punk.run_window("chip8_window", message_caller)?;

    graph_punk.benchmark_print_results();

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

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    EventPump, Sdl,
};

use crate::{
    gl_exec,
    renderer::{check_errors, clear_errors, Renderer},
    types::RGB,
};

pub struct Window<'a> {
    sdl: Sdl,
    window: sdl2::video::Window,
    renderer: Renderer,
    event_pump: EventPump,
    background_color: RGB,
    update_callback: Box<dyn FnMut(&mut Renderer) + 'a>,
}

impl<'a> Window<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        // Défini les options globales d'OpenGL, nécessaire avant de se servir de la moindre
        // fonction OpenGL.
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        // Crée la fenêtre.
        let window = match video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .position_centered()
            .build()
        {
            Ok(t) => t,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        // Crée le contexte OpenGL nécessaire pour la fenêtre afin de dessiner dessus.
        let gl_context = window.gl_create_context()?;
        // Charge toutes les fonctions OpenGL grâce à une recherche customisée.
        let gl = gl::load_with(|proc_name| {
            video_subsystem.gl_get_proc_address(proc_name) as *const std::os::raw::c_void
        });

        // Permet de récupérer les évènements liés à la fenêtre, comme les entrées utilisateur.
        let event_pump = sdl.event_pump()?;

        let renderer = Renderer::new(gl_context, gl);
        if let Err(err) = renderer.set_viewport_size(width as i32, height as i32) {
            return Err(err);
        }

        Ok(Window {
            sdl,
            window,
            renderer,
            event_pump,
            background_color: RGB::new(0, 0, 0),
            update_callback: Box::new(|_| {}),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        // Boucle infinie de la fenêtre.
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    // Détecte lorsque la fenêtre est redimensionnée.
                    Event::Window {
                        win_event: WindowEvent::Resized(width, height),
                        ..
                    } => {
                        println!("Window resized: {width}x{height}");
                        if let Err(err) = self.renderer.set_viewport_size(width, height) {
                            eprintln!("{err}");
                        }
                    }
                    _ => {}
                }
            }

            // Appelle la fonction de callback pour mettre à jour l'état du moteur et du programme.
            (self.update_callback)(&mut self.renderer);

            // Défini la couleur qu'OpenGL va utiliser pour nettoyer l'écran.
            if let Err(err) = gl_exec!(|| gl::ClearColor(
                self.background_color.r as f32 / 255.0 as f32,
                self.background_color.g as f32 / 255.0 as f32,
                self.background_color.b as f32 / 255.0 as f32,
                1.0 as f32,
            )) {
                return Err(err);
            }

            // Nettoie l'écran.
            if let Err(err) = gl_exec!(|| gl::Clear(gl::COLOR_BUFFER_BIT)) {
                return Err(err);
            }

            // Dessine tous les objets.
            for drawing_object in self.renderer.borrow_drawing_objects().iter() {
                if drawing_object.is_visible() {
                    if let Err(err) = drawing_object.draw() {
                        eprintln!("{err}");

                        continue;
                    }
                }
            }

            // Met à jour le contenu dessiné sur la fenêtre.
            self.window.gl_swap_window();
        }

        Ok(())
    }

    pub fn borrow_sdl(&self) -> &Sdl {
        &self.sdl
    }

    pub fn borrow_renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn borrow_mut_renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn set_update_callback(&mut self, c: impl FnMut(&mut Renderer) + 'a) {
        self.update_callback = Box::new(c);
    }
}

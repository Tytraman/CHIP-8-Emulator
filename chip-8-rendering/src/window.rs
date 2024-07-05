use std::{any::Any, cell::RefCell, ops::Deref, rc::Rc};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    EventPump, Sdl, VideoSubsystem,
};

use crate::{
    gl_exec,
    renderer::{check_errors, clear_errors, KeyStatus, Renderer, RendererParams},
    types::RGB,
};

pub struct UserData {
    data: Option<Box<dyn Any>>,
}

impl UserData {
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

pub struct Window<'a> {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: sdl2::video::Window,
    event_pump: EventPump,
    background_color: RGB,
    update_callback: Box<dyn FnMut(Rc<RefCell<RendererParams>>, &mut UserData) + 'a>,
    user_data: UserData,
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

        // Permet de récupérer les évènements liés à la fenêtre, comme les entrées utilisateur.
        let event_pump = sdl.event_pump()?;

        Ok(Window {
            sdl,
            video_subsystem,
            window,
            event_pump,
            background_color: RGB::new(0, 0, 0),
            update_callback: Box::new(|_, _| {}),
            user_data: UserData { data: None },
        })
    }

    pub fn create_renderer(&self) -> Result<RendererParams, String> {
        // Crée le contexte OpenGL nécessaire pour la fenêtre afin de dessiner dessus.
        let gl_context = self.window.gl_create_context()?;

        // Charge toutes les fonctions OpenGL grâce à une recherche customisée.
        let gl = gl::load_with(|proc_name| {
            self.video_subsystem.gl_get_proc_address(proc_name) as *const std::os::raw::c_void
        });

        let renderer = Renderer::new(gl_context, gl);
        if let Err(err) =
            renderer.set_viewport_size(self.get_width() as i32, self.get_height() as i32)
        {
            return Err(err);
        }

        Ok(RendererParams { renderer })
    }

    pub fn run(&mut self, renderer_params: Rc<RefCell<RendererParams>>) -> Result<(), String> {
        // Boucle infinie de la fenêtre.
        'running: loop {
            renderer_params
                .deref()
                .borrow_mut()
                .renderer
                .update_last_key_states();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num1),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("1", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num2),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("2", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num3),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("3", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Num4),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("4", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("a", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Z),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("z", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::E),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("e", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("r", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("q", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("s", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("d", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::F),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("f", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("w", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::X),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("x", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::C),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("c", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::V),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("v", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::N),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("n", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::O),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("o", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::P),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("p", KeyStatus::Pressed);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state(" ", KeyStatus::Pressed);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num1),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("1", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num2),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("2", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num3),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("3", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Num4),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("4", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::A),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("a", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Z),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("z", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::E),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("e", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::R),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("r", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Q),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("q", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::S),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("s", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::D),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("d", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::F),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("f", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::W),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("w", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::X),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("x", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::C),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("c", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::V),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("v", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::N),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("n", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::O),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("o", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::P),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state("p", KeyStatus::Released);
                    }
                    Event::KeyUp {
                        keycode: Some(Keycode::Space),
                        repeat: false,
                        ..
                    } => {
                        renderer_params
                            .deref()
                            .borrow_mut()
                            .renderer
                            .set_key_state(" ", KeyStatus::Released);
                    }
                    // Détecte lorsque la fenêtre est redimensionnée.
                    Event::Window {
                        win_event: WindowEvent::Resized(width, height),
                        ..
                    } => {
                        println!("Window resized: {width}x{height}");
                        if let Err(err) = renderer_params
                            .borrow_mut()
                            .renderer
                            .set_viewport_size(width, height)
                        {
                            eprintln!("{err}");
                        }
                    }
                    _ => {}
                }
            }

            // Appelle la fonction de callback pour mettre à jour l'état du moteur et du programme.
            (self.update_callback)(Rc::clone(&renderer_params), &mut self.user_data);

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
            for drawing_object in renderer_params
                .borrow_mut()
                .renderer
                .borrow_drawing_objects()
                .iter()
            {
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

    pub fn get_width(&self) -> u32 {
        self.window.size().0
    }

    pub fn get_height(&self) -> u32 {
        self.window.size().1
    }

    pub fn borrow_sdl(&self) -> &Sdl {
        &self.sdl
    }

    pub fn set_update_callback(
        &mut self,
        c: impl FnMut(Rc<RefCell<RendererParams>>, &mut UserData) + 'a,
        user_data: UserData,
    ) {
        self.update_callback = Box::new(c);
        self.user_data = user_data;
    }
}

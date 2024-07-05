use std::{collections::HashMap, fs};

use gl::types::GLint;
use sdl2::video::GLContext;
use vbo::VBO;

use crate::{
    maths::vec::Vec3,
    shader::{Shader, ShaderProgram, ShaderType},
    shapes::rectangle::Rectangle,
};

pub mod data_object;
pub mod uniform;
pub mod vao;
pub mod vbo;

#[macro_export]
macro_rules! gl_exec {
    ( $closure:expr ) => {{
        unsafe {
            clear_errors();

            $closure();

            check_errors(stringify!($closure))
        }
    }};
}

pub trait Draw {
    fn draw(&self) -> Result<(), String>;

    fn get_position(&self) -> Vec3<f32>;
    fn set_position(&mut self, position: Vec3<f32>);

    fn get_scale(&self) -> Vec3<f32>;
    fn set_scale(&mut self, scale: Vec3<f32>);

    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, value: bool);
}

#[derive(Clone, Debug)]
pub enum KeyStatus {
    Pressed,
    Released,
}

pub struct Keys {
    pub one: (KeyStatus, KeyStatus),
    pub two: (KeyStatus, KeyStatus),
    pub three: (KeyStatus, KeyStatus),
    pub four: (KeyStatus, KeyStatus),
    pub a: (KeyStatus, KeyStatus),
    pub c: (KeyStatus, KeyStatus),
    pub d: (KeyStatus, KeyStatus),
    pub e: (KeyStatus, KeyStatus),
    pub f: (KeyStatus, KeyStatus),
    pub n: (KeyStatus, KeyStatus),
    pub o: (KeyStatus, KeyStatus),
    pub p: (KeyStatus, KeyStatus),
    pub q: (KeyStatus, KeyStatus),
    pub r: (KeyStatus, KeyStatus),
    pub s: (KeyStatus, KeyStatus),
    pub v: (KeyStatus, KeyStatus),
    pub w: (KeyStatus, KeyStatus),
    pub x: (KeyStatus, KeyStatus),
    pub z: (KeyStatus, KeyStatus),
    pub space: (KeyStatus, KeyStatus),
}

impl Keys {
    pub fn new() -> Self {
        Self {
            one: (KeyStatus::Released, KeyStatus::Released),
            two: (KeyStatus::Released, KeyStatus::Released),
            three: (KeyStatus::Released, KeyStatus::Released),
            four: (KeyStatus::Released, KeyStatus::Released),
            a: (KeyStatus::Released, KeyStatus::Released),
            c: (KeyStatus::Released, KeyStatus::Released),
            d: (KeyStatus::Released, KeyStatus::Released),
            e: (KeyStatus::Released, KeyStatus::Released),
            f: (KeyStatus::Released, KeyStatus::Released),
            n: (KeyStatus::Released, KeyStatus::Released),
            o: (KeyStatus::Released, KeyStatus::Released),
            p: (KeyStatus::Released, KeyStatus::Released),
            q: (KeyStatus::Released, KeyStatus::Released),
            r: (KeyStatus::Released, KeyStatus::Released),
            s: (KeyStatus::Released, KeyStatus::Released),
            v: (KeyStatus::Released, KeyStatus::Released),
            w: (KeyStatus::Released, KeyStatus::Released),
            x: (KeyStatus::Released, KeyStatus::Released),
            z: (KeyStatus::Released, KeyStatus::Released),
            space: (KeyStatus::Released, KeyStatus::Released),
        }
    }
}

pub struct Renderer {
    context: GLContext,
    gl: (),
    vbos: HashMap<usize, VBO>,
    drawing_objects: Vec<Box<dyn Draw>>,
    keys: Keys,
}

pub struct RendererParams {
    pub renderer: Renderer,
}

impl Renderer {
    pub fn new(context: GLContext, gl: ()) -> Self {
        Self {
            context,
            gl,
            vbos: HashMap::new(),
            drawing_objects: Vec::new(),
            keys: Keys::new(),
        }
    }

    pub fn init_resources(&mut self) -> Result<(), String> {
        let shader_filename = "Builtin/Shaders/chip8_vertex_shader.glsl";
        println!("Trying to open shader file \"{shader_filename}\"...");
        let vertex_shader = match fs::read_to_string(shader_filename) {
            Ok(t) => t,
            Err(e) => return Err(e.to_string()),
        };

        let shader_filename = "Builtin/Shaders/chip8_fragment_shader.glsl";
        println!("Trying to open shader file \"{shader_filename}\"...");
        let fragment_shader = match fs::read_to_string(shader_filename) {
            Ok(t) => t,
            Err(e) => return Err(e.to_string()),
        };

        let mut vert_shader = Shader::new(
            ShaderType::Vertex,
            "chip8_vertex_shader".to_string(),
            vertex_shader,
        );

        match vert_shader.create() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        match vert_shader.source() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        match vert_shader.compile() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        let mut frag_shader = Shader::new(
            ShaderType::Fragment,
            "chip8_fragment_shader".to_string(),
            fragment_shader,
        );

        match frag_shader.create() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        match frag_shader.source() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        match frag_shader.compile() {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        let program = match ShaderProgram::build(&vert_shader, &frag_shader) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        if let Err(err) = program.link() {
            return Err(err);
        }

        let chip_width = 64.0_f32;
        let chip_height = 32.0_f32;

        let mut x = -1.0_f32 + ((2.0_f32 / chip_width) / 2.0_f32);
        let mut y = 1.0_f32 - ((2.0_f32 / chip_height) / 2.0_f32);

        let x_origin = x;

        let rect = match Rectangle::build(
            self,
            program,
            Vec3 { x, y, z: 0.0_f32 },
            Vec3 {
                x: 2.0_f32 / chip_width,
                y: 2.0_f32 / chip_height,
                z: 1.0_f32,
            },
        ) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        for _ in 0..32 {
            for _ in 0..64 {
                let mut rect_clone = rect.clone();
                rect_clone.set_position(Vec3 { x, y, z: 0.0_f32 });
                rect_clone.set_visible(false);

                self.drawing_objects.push(Box::new(rect_clone));
                x += 2.0_f32 / chip_width;
            }
            x = x_origin;
            y -= 2.0_f32 / chip_height;
        }

        Ok(())
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Result<&mut Box<dyn Draw>, String> {
        if x > 63 || y > 31 {
            return Err("Indexes are out of bound".to_string());
        }

        let pixel = match self.drawing_objects.get_mut(y * 64 + x) {
            Some(t) => t,
            None => return Err(format!("Cannot find drawing object at {x} {y}")),
        };

        Ok(pixel)
    }

    pub fn clear_grid_pixel(&mut self) -> Result<(), String> {
        for x in 0..64 {
            for y in 0..32 {
                self.set_grid_pixel(x, y, false)?;
            }
        }

        Ok(())
    }

    pub fn toggle_grid_pixel(&mut self, x: usize, y: usize) -> Result<(), String> {
        let pixel = match self.get_pixel(x, y) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        let visible = pixel.is_visible();

        pixel.set_visible(!visible);

        Ok(())
    }

    pub fn set_grid_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        let pixel = match self.get_pixel(x, y) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        pixel.set_visible(value);

        Ok(())
    }

    pub fn set_viewport_size(&self, width: i32, height: i32) -> Result<(), String> {
        gl_exec!(|| gl::Viewport(0, 0, width as GLint, height as GLint))
    }

    pub fn borrow_context(&self) -> &GLContext {
        &self.context
    }

    pub fn borrow_gl(&self) -> &() {
        &self.gl
    }

    pub fn borrow_drawing_objects(&self) -> &Vec<Box<dyn Draw>> {
        &self.drawing_objects
    }

    pub fn get_key_status(&self, key: &str) -> Option<(KeyStatus, KeyStatus)> {
        match key {
            "1" => Some(self.keys.one.clone()),
            "2" => Some(self.keys.two.clone()),
            "3" => Some(self.keys.three.clone()),
            "4" => Some(self.keys.four.clone()),
            "a" => Some(self.keys.a.clone()),
            "c" => Some(self.keys.c.clone()),
            "d" => Some(self.keys.d.clone()),
            "e" => Some(self.keys.e.clone()),
            "f" => Some(self.keys.f.clone()),
            "n" => Some(self.keys.n.clone()),
            "o" => Some(self.keys.o.clone()),
            "p" => Some(self.keys.p.clone()),
            "q" => Some(self.keys.q.clone()),
            "r" => Some(self.keys.r.clone()),
            "s" => Some(self.keys.s.clone()),
            "v" => Some(self.keys.v.clone()),
            "w" => Some(self.keys.w.clone()),
            "x" => Some(self.keys.x.clone()),
            "z" => Some(self.keys.z.clone()),
            " " => Some(self.keys.space.clone()),
            _ => None,
        }
    }

    pub fn update_last_key_states(&mut self) {
        self.keys.one.1 = self.keys.one.0.clone();
        self.keys.two.1 = self.keys.two.0.clone();
        self.keys.three.1 = self.keys.three.0.clone();
        self.keys.four.1 = self.keys.four.0.clone();
        self.keys.a.1 = self.keys.a.0.clone();
        self.keys.c.1 = self.keys.c.0.clone();
        self.keys.d.1 = self.keys.d.0.clone();
        self.keys.e.1 = self.keys.e.0.clone();
        self.keys.f.1 = self.keys.f.0.clone();
        self.keys.n.1 = self.keys.n.0.clone();
        self.keys.o.1 = self.keys.o.0.clone();
        self.keys.p.1 = self.keys.p.0.clone();
        self.keys.q.1 = self.keys.q.0.clone();
        self.keys.r.1 = self.keys.r.0.clone();
        self.keys.s.1 = self.keys.s.0.clone();
        self.keys.v.1 = self.keys.v.0.clone();
        self.keys.w.1 = self.keys.w.0.clone();
        self.keys.x.1 = self.keys.x.0.clone();
        self.keys.z.1 = self.keys.z.0.clone();
        self.keys.space.1 = self.keys.space.0.clone();
    }

    pub fn set_key_state(&mut self, key: &str, state: KeyStatus) {
        match key {
            "1" => self.keys.one.0 = state,
            "2" => self.keys.two.0 = state,
            "3" => self.keys.three.0 = state,
            "4" => self.keys.four.0 = state,
            "a" => self.keys.a.0 = state,
            "c" => self.keys.c.0 = state,
            "d" => self.keys.d.0 = state,
            "e" => self.keys.e.0 = state,
            "f" => self.keys.f.0 = state,
            "n" => self.keys.n.0 = state,
            "o" => self.keys.o.0 = state,
            "p" => self.keys.p.0 = state,
            "q" => self.keys.q.0 = state,
            "r" => self.keys.r.0 = state,
            "s" => self.keys.s.0 = state,
            "v" => self.keys.v.0 = state,
            "w" => self.keys.w.0 = state,
            "x" => self.keys.x.0 = state,
            "z" => self.keys.z.0 = state,
            " " => self.keys.space.0 = state,
            _ => (),
        }
    }
}

pub fn clear_errors() {
    unsafe {
        loop {
            if gl::GetError() == gl::NO_ERROR {
                break;
            }
        }
    }
}

pub fn check_errors(function_name: &str) -> Result<(), String> {
    let mut message = String::new();
    let mut code;

    unsafe {
        loop {
            let error = gl::GetError();

            if error == gl::NO_ERROR {
                break;
            }

            match error {
                gl::INVALID_ENUM => code = "INVALID_ENUM".to_string(),
                gl::INVALID_VALUE => code = "INVALID_VALUE".to_string(),
                gl::INVALID_OPERATION => code = "INVALID_OPERATION".to_string(),
                gl::STACK_OVERFLOW => code = "STACK_OVERFLOW".to_string(),
                gl::STACK_UNDERFLOW => code = "STACK_UNDERFLOW".to_string(),
                gl::OUT_OF_MEMORY => code = "OUT_OF_MEMORY".to_string(),
                gl::INVALID_FRAMEBUFFER_OPERATION => {
                    code = "INVALID_FRAMEBUFFER_OPERATION".to_string()
                }
                gl::CONTEXT_LOST => code = "CONTEXT_LOST".to_string(),

                _ => code = error.to_string(),
            }

            message.push_str(&format!("[OpenGL error] {function_name}: {code}"));
        }
    }

    if message.len() > 0 {
        return Err(message);
    }

    Ok(())
}

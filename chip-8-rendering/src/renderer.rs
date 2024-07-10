use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

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

pub struct RendererManager {
    renderers: HashMap<String, Renderer>,
}

impl RendererManager {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            renderers: HashMap::new(),
        }))
    }

    pub fn borrow_mut_renderer(&mut self, name: &str) -> Option<&mut Renderer> {
        self.renderers.get_mut(name)
    }

    pub fn add_renderer(&mut self, name: &str, renderer: Renderer) {
        let _ = self.renderers.insert(name.to_string(), renderer);
    }

    pub fn get_number_of_renderers(&self) -> usize {
        self.renderers.len()
    }
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

pub struct Renderer {
    context: GLContext,
    vbos: HashMap<usize, VBO>,
    drawing_objects: Vec<Box<dyn Draw>>,
}

impl Renderer {
    pub fn new(context: GLContext) -> Self {
        Self {
            context,
            vbos: HashMap::new(),
            drawing_objects: Vec::new(),
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

    pub fn borrow_drawing_objects(&self) -> &Vec<Box<dyn Draw>> {
        &self.drawing_objects
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

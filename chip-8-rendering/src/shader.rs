use std::{ffi::CString, ptr};

use gl::types::GLuint;

use crate::common::ChipString;
use crate::gl_exec;
use crate::renderer::{check_errors, clear_errors};

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    shader_type: ShaderType,
    name: String,
    source: String,
    id: u32,
}

#[derive(Clone)]
pub struct ShaderProgram {
    id: GLuint,
}

impl Shader {
    pub fn new(shader_type: ShaderType, name: String, source: String) -> Self {
        Self {
            shader_type,
            name,
            source,
            id: 0,
        }
    }

    pub fn create(&mut self) -> Result<(), String> {
        let t = match self.shader_type {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        };

        gl_exec!(|| self.id = gl::CreateShader(t))
    }

    pub fn source(&self) -> Result<(), String> {
        let c_str = match CString::new(self.source.as_bytes()) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };

        gl_exec!(|| gl::ShaderSource(self.id, 1, &c_str.as_ptr(), ptr::null()))
    }

    pub fn compile(&self) -> Result<(), String> {
        if let Err(err) = gl_exec!(|| gl::CompileShader(self.id)) {
            return Err(err);
        }

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        // S'il y a une erreur, 'success' vaudra 0.
        if let Err(err) = gl_exec!(|| gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success)) {
            return Err(err);
        }

        if success == 0 {
            if let Err(err) = gl_exec!(|| gl::GetShaderInfoLog(
                self.id,
                info_log.len() as i32,
                ptr::null_mut(),
                info_log.as_mut_ptr(),
            )) {
                return Err(err);
            }

            return Err(ChipString::i8_array_to_string(&info_log));
        }

        Ok(())
    }

    pub fn destroy(&self) -> Result<(), String> {
        gl_exec!(|| gl::DeleteShader(self.id))
    }

    pub fn is_vertex_shader(&self) -> bool {
        matches!(self.shader_type, ShaderType::Vertex)
    }

    pub fn is_fragment_shader(&self) -> bool {
        matches!(self.shader_type, ShaderType::Fragment)
    }

    pub fn borrow_name(&self) -> &String {
        &self.name
    }
}

impl ShaderProgram {
    pub fn none() -> Self {
        Self { id: 0 }
    }

    pub fn build(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self, String> {
        if !vertex_shader.is_vertex_shader() {
            return Err("'vertex_shader' is not a Vertex Shader".to_string());
        }

        if !fragment_shader.is_fragment_shader() {
            return Err("'fragment_shader' is not a Fragment Shader".to_string());
        }

        let mut id = 0;

        if let Err(err) = gl_exec!(|| id = gl::CreateProgram()) {
            return Err(err);
        }

        if let Err(err) = gl_exec!(|| gl::AttachShader(id, vertex_shader.id)) {
            return Err(err);
        }

        if let Err(err) = gl_exec!(|| gl::AttachShader(id, fragment_shader.id)) {
            return Err(err);
        }

        Ok(ShaderProgram { id })
    }

    pub fn link(&self) -> Result<(), String> {
        if let Err(err) = gl_exec!(|| gl::LinkProgram(self.id)) {
            return Err(err);
        }

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        if let Err(err) = gl_exec!(|| gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success)) {
            return Err(err);
        }

        if success == 0 {
            if let Err(err) = gl_exec!(|| gl::GetProgramInfoLog(
                self.id,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr()
            )) {
                return Err(err);
            }

            return Err(ChipString::i8_array_to_string(&info_log));
        }

        Ok(())
    }

    pub fn use_it(&self) -> Result<(), String> {
        gl_exec!(|| gl::UseProgram(self.id))
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

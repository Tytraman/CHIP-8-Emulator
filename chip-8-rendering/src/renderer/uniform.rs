use std::ffi::CString;

use gl::types::GLint;

use crate::{
    gl_exec,
    maths::{
        mat::Mat4,
        vec::{Vec3, Vec4},
    },
    shader::ShaderProgram,
};

use super::{check_errors, clear_errors};

#[derive(Clone)]
pub struct Uniform {
    location: GLint,
}

impl Uniform {
    pub fn new() -> Self {
        Self { location: -1 }
    }

    pub fn search(&mut self, shader_program: &ShaderProgram, name: &str) -> Result<(), String> {
        let c_str = match CString::new(name.as_bytes()) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };

        gl_exec!(|| self.location = gl::GetUniformLocation(shader_program.get_id(), c_str.as_ptr()))
    }

    pub fn send_vec3(&self, v: &Vec3<f32>) -> Result<(), String> {
        gl_exec!(|| gl::Uniform3f(self.location, v.x, v.y, v.z))
    }

    pub fn send_vec4(&self, v: &Vec4<f32>) -> Result<(), String> {
        gl_exec!(|| gl::Uniform4f(self.location, v.x, v.y, v.z, v.w))
    }

    pub fn send_mat4(&self, m: &Mat4<f32>) -> Result<(), String> {
        gl_exec!(|| gl::UniformMatrix4fv(self.location, 1, gl::TRUE, m.borrow_data().as_ptr()))
    }
}

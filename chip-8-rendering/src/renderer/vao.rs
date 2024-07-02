use std::os;

use gl::types::GLuint;

use crate::gl_exec;

use super::{check_errors, clear_errors};

#[derive(Clone)]
pub struct VAO {
    id: GLuint,
}

impl VAO {
    pub fn build() -> Result<Self, String> {
        let mut id = 0;

        if let Err(err) = gl_exec!(|| gl::GenVertexArrays(1, &mut id)) {
            return Err(err);
        }

        Ok(Self { id })
    }

    pub fn bind(&self) -> Result<(), String> {
        gl_exec!(|| gl::BindVertexArray(self.id))
    }

    pub fn attrib_pointer(
        &self,
        index: u32,
        size: i32,
        stride: i32,
        offset: usize,
    ) -> Result<(), String> {
        gl_exec!(|| gl::VertexAttribPointer(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            stride,
            offset as *const os::raw::c_void,
        ))
    }

    pub fn enable_attrib(&self, index: u32) -> Result<(), String> {
        gl_exec!(|| gl::EnableVertexAttribArray(index))
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

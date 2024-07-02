use std::mem::size_of;

use gl::types::GLuint;

use crate::gl_exec;

use super::{check_errors, clear_errors};

pub struct VBO {
    vertices: Vec<f32>,
    id: GLuint,
}

impl VBO {
    pub fn build(mut vertices: Vec<f32>) -> Result<Self, String> {
        let mut id = 0;

        // Génère un nouveau VBO.
        if let Err(err) = gl_exec!(|| gl::GenBuffers(1, &mut id)) {
            return Err(err);
        }

        // Rend le nouveau VBO actif.
        if let Err(err) = gl_exec!(|| gl::BindBuffer(gl::ARRAY_BUFFER, id)) {
            return Err(err);
        }

        // Copie les données dans le VBO nouvellement créé.
        if let Err(err) = gl_exec!(|| gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_mut_ptr() as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        )) {
            return Err(err);
        }

        Ok(Self { vertices, id })
    }

    pub fn bind(&self) -> Result<(), String> {
        gl_exec!(|| gl::BindBuffer(gl::ARRAY_BUFFER, self.id))
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn borrow_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }
}

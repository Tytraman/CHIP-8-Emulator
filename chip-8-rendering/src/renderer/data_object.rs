use std::collections::HashMap;

use gl::types::GLsizei;

use crate::{
    gl_exec,
    maths::{mat::Mat4, vec::Vec3},
    shader::ShaderProgram,
};

use super::{check_errors, clear_errors, uniform::Uniform, vao::VAO, vbo::VBO, Renderer};

#[derive(Clone)]
pub struct DataObject {
    vertices_number: usize,
    vao: VAO,
    shader_program: ShaderProgram,
    uniforms: HashMap<String, Uniform>,
    position: Vec3<f32>,
    scale: Vec3<f32>,
    visible: bool,
}

pub struct AttribPointer {
    pub index: u32,
    pub size: i32,
    pub stride: i32,
    pub offset: usize,
}

pub type AttribPointers = Vec<AttribPointer>;

impl DataObject {
    pub fn build(
        renderer: &mut Renderer,
        vertices: Vec<f32>,
        attrib_pointers: &AttribPointers,
        position: Vec3<f32>,
        scale: Vec3<f32>,
    ) -> Result<Self, String> {
        let vao = match VAO::build() {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        if let Err(err) = vao.bind() {
            return Err(err);
        }

        let vbo = match VBO::build(vertices) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        let vertices_number = vbo.borrow_vertices().len();

        renderer.vbos.insert(vbo.get_id() as usize, vbo);

        let mut index = 0;
        for attrib in attrib_pointers.iter() {
            if let Err(err) =
                vao.attrib_pointer(attrib.index, attrib.size, attrib.stride, attrib.offset)
            {
                return Err(err);
            };

            if let Err(err) = vao.enable_attrib(index) {
                return Err(err);
            }

            index += 1;
        }

        Ok(Self {
            vertices_number,
            vao,
            shader_program: ShaderProgram::none(),
            uniforms: HashMap::new(),
            position,
            scale,
            visible: true,
        })
    }

    pub fn draw(&self) -> Result<(), String> {
        let transform_uniform = match self.uniforms.get("chip_transform") {
            Some(t) => t,
            None => return Err("chip_transform uniform not found".to_string()),
        };

        if let Err(err) = self.shader_program.use_it() {
            return Err(err);
        }

        let mut trans = Mat4::translate(&Mat4::new(), &self.position);
        trans = Mat4::scale(&trans, &self.scale);

        if let Err(err) = transform_uniform.send_mat4(&trans) {
            return Err(err);
        }

        if let Err(err) = self.vao.bind() {
            return Err(err);
        }

        gl_exec!(|| gl::DrawArrays(gl::TRIANGLES, 0, self.get_vertices_number() as GLsizei))
    }

    pub fn get_vertices_number(&self) -> usize {
        self.vertices_number
    }

    pub fn get_position(&self) -> Vec3<f32> {
        self.position.clone()
    }

    pub fn get_scale(&self) -> Vec3<f32> {
        self.scale.clone()
    }

    pub fn set_position(&mut self, position: Vec3<f32>) {
        self.position = position;
    }

    pub fn set_scale(&mut self, scale: Vec3<f32>) {
        self.scale = scale;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    pub fn borrow_vao(&self) -> &VAO {
        &self.vao
    }

    pub fn borrow_shader_program(&self) -> &ShaderProgram {
        &self.shader_program
    }

    pub fn borrow_uniforms(&self) -> &HashMap<String, Uniform> {
        &self.uniforms
    }

    pub fn borrow_mut_uniforms(&mut self) -> &mut HashMap<String, Uniform> {
        &mut self.uniforms
    }

    pub fn set_shader_program(&mut self, shader_program: ShaderProgram) {
        self.shader_program = shader_program;
    }
}

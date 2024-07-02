use crate::{
    maths::vec::{Vec3, Vec4},
    renderer::{
        data_object::{AttribPointer, DataObject},
        uniform::Uniform,
        Draw, Renderer,
    },
    shader::ShaderProgram,
};

use std::mem;

#[derive(Clone)]
pub struct Rectangle {
    data_object: DataObject,
}

impl Rectangle {
    pub fn build(
        renderer: &mut Renderer,
        shader_program: ShaderProgram,
        position: Vec3<f32>,
        size: Vec3<f32>,
    ) -> Result<Self, String> {
        let x = 0.5_f32;
        let y = 0.5_f32;

        let pixel = vec![
            -x, -y, 0.0_f32, x, -y, 0.0_f32, -x, y, 0.0_f32, x, -y, 0.0_f32, -x, y, 0.0_f32, x, y,
            0.0_f32,
        ];

        let attrib_pointer = AttribPointer {
            index: 0,
            size: 3,
            stride: (3 * mem::size_of::<f32>()) as i32,
            offset: 0,
        };

        let mut rect =
            match DataObject::build(renderer, pixel, &vec![attrib_pointer], position, size) {
                Ok(t) => t,
                Err(err) => return Err(err),
            };

        let uniforms = rect.borrow_mut_uniforms();

        if let Err(err) = shader_program.use_it() {
            return Err(err);
        }

        let color_name = "chip_color";
        let mut color_uniform = Uniform::new();
        let mut transform_uniform = Uniform::new();

        if let Err(err) = color_uniform.search(&shader_program, color_name) {
            return Err(err);
        }

        let transform_name = "chip_transform";
        if let Err(err) = transform_uniform.search(&shader_program, transform_name) {
            return Err(err);
        }

        if let Err(err) = color_uniform.send_vec4(&Vec4 {
            x: 1.0_f32,
            y: 1.0_f32,
            z: 1.0_f32,
            w: 1.0_f32,
        }) {
            return Err(err);
        }

        uniforms.insert(color_name.to_string(), color_uniform);
        uniforms.insert(transform_name.to_string(), transform_uniform);

        rect.set_shader_program(shader_program);

        Ok(Self { data_object: rect })
    }
}

impl Draw for Rectangle {
    fn draw(&self) -> Result<(), String> {
        self.data_object.draw()
    }

    fn get_position(&self) -> Vec3<f32> {
        self.data_object.get_position()
    }

    fn get_scale(&self) -> Vec3<f32> {
        self.data_object.get_scale()
    }

    fn set_position(&mut self, position: Vec3<f32>) {
        self.data_object.set_position(position);
    }

    fn set_scale(&mut self, scale: Vec3<f32>) {
        self.data_object.set_scale(scale);
    }

    fn is_visible(&self) -> bool {
        self.data_object.is_visible()
    }

    fn set_visible(&mut self, value: bool) {
        self.data_object.set_visible(value);
    }
}

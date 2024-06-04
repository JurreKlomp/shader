mod clean;

mod objects;
mod vbo;

use clean::*;
use objects::*;

use serde::{Deserialize, Serialize};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;
const TITLE: &str = "Shader";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    camera: Camera,
    sphere: Sphere,
}

fn main() {
    println!("Hello, world!");

    let config_file = std::fs::read_to_string("./config.json").unwrap();

    let config: Config = serde_json::from_str(&config_file).unwrap();
    let Config { camera, sphere } = config;

    // println!("{:?}", config);

    let mut window = window::Window::new(WIDTH, HEIGHT, TITLE);

    // [----------OPENGL----------]
    create_square();

    let mut shader_program = shader_program::ShaderProgram::new();
    shader_program.add_shader("./shaders/vertex_shader.vert", gl::VERTEX_SHADER);
    shader_program.add_shader("./shaders/fragment_shader.frag", gl::FRAGMENT_SHADER);
    let shader_program = shader_program.compile();
    shader_program.bind();

    let camera_uniforms = Camera::get_uniforms(shader_program.id);
    let sphere_uniforms = Sphere::get_uniforms(shader_program.id);
    let material_uniforms = Material::get_uniforms(shader_program.id);

    let (_, width_uniform) = objects::create_uniform(shader_program.id, "width");
    let (_, height_uniform) = objects::create_uniform(shader_program.id, "height");
    // [----------OPENGL----------]

    while !window.should_close() {
        unsafe {
            camera.update_uniforms(&camera_uniforms);
            sphere.update_uniforms(&sphere_uniforms);
            sphere.material.update_uniforms(&material_uniforms);

            let (width, height) = window.get_size();
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Uniform1f(width_uniform, width as f32);
            gl::Uniform1f(height_uniform, height as f32);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        window.update();
    }
    shader_program.unbind();
}

fn create_square() {
    let data: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0];

    let mut vbo = 0;
    let attribute = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            &data[0] as *const f32 as *const std::os::raw::c_void,
            gl::DYNAMIC_DRAW,
        );
        gl::VertexAttribPointer(
            attribute,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(attribute);
    }
}

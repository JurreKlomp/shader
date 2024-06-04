mod clean;
use clean::*;

mod objects;
use objects::*;
mod vbo;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 540;
const TITLE: &str = "Shader";

fn main() {
    println!("Hello, world!");

    let mut window = window::Window::new(WIDTH, HEIGHT, TITLE);

    let camera = Camera {
        position: vec3::Vec3::new(0.0, 0.0, -5.0),
        fov: 90.0,
    };
    let sphere = Sphere {
        position: vec3::Vec3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        material: Material {
            albedo: vec3::Vec3::new(0.5, 0.7, 1.0),
            metallic: 0.0,
            roughness: 1.0,
        },
    };

    // [----------OPENGL----------]
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

    let mut shader_program = shader_program::ShaderProgram::new();
    shader_program.add_shader("./vertex_shader.vert", gl::VERTEX_SHADER);
    shader_program.add_shader("./fragment_shader.frag", gl::FRAGMENT_SHADER);
    let shader_program = shader_program.compile();
    shader_program.bind();

    let camera_uniforms = Camera::get_uniforms(shader_program.id);
    let sphere_uniforms = Sphere::get_uniforms(shader_program.id);
    let material_uniforms = Material::get_uniforms(shader_program.id);
    // [----------OPENGL----------]

    while !window.should_close() {
        unsafe {
            camera.update_uniforms(&camera_uniforms);
            sphere.update_uniforms(&sphere_uniforms);
            sphere.material.update_uniforms(&material_uniforms);

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        window.update();
    }
    shader_program.unbind();
}

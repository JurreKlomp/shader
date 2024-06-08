mod clean;
mod vbo;

use clean::*;

use serde::{Deserialize, Serialize};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const TITLE: &str = "Shader";

#[derive(Serialize, Deserialize, Debug)]
struct Scene {
    camera: objects::Camera,
    spheres: Vec<objects::Sphere>,
}

fn main() {
    println!("Hello, world!");

    let config_file = std::fs::read_to_string("./scene.json").unwrap();
    let Scene {
        mut camera,
        mut spheres,
    } = serde_json::from_str(&config_file).unwrap();
    spheres.sort_by(|a, b| {
        (camera.position - a.position)
            .partial_cmp(&(camera.position - b.position))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut window = window::Window::new(WIDTH, HEIGHT, TITLE);

    // [----------OPENGL----------]
    create_viewport();

    let mut program = shader_program::ShaderProgram::new();
    program.add_shader("./shaders/vertex_shader.vert", gl::VERTEX_SHADER);
    program.add_shader("./shaders/fragment_shader.frag", gl::FRAGMENT_SHADER);
    let program = program.compile();
    program.bind();

    let (_, width_uniform) = create_uniform(program.id, "width".to_string());
    let (_, height_uniform) = create_uniform(program.id, "height".to_string());

    let (_, camera_position_uniform) = create_uniform(program.id, "camera.position".to_string());
    let (_, camera_angles_uniform) = create_uniform(program.id, "camera.angles".to_string());
    let (_, camera_fov_uniform) = create_uniform(program.id, "camera.fov".to_string());

    let mut sphere_uniforms = Vec::new();

    for i in 0..spheres.len() {
        let (_, position) = create_uniform(program.id, format!("spheres[{i}].position"));
        let (_, radius) = create_uniform(program.id, format!("spheres[{i}].radius"));
        let (_, albedo) = create_uniform(program.id, format!("spheres[{i}].material.albedo"));
        let (_, roughness) = create_uniform(program.id, format!("spheres[{i}].material.roughness"));
        let (_, metallic) = create_uniform(program.id, format!("spheres[{i}].material.metallic"));

        sphere_uniforms.push((position, radius, albedo, roughness, metallic));
    }

    // [----------OPENGL----------]

    while !window.should_close() {
        const DELTA: f32 = 0.0005;
        if window.is_pressed(glfw::Key::W) {
            camera.position.z += DELTA;
        }
        if window.is_pressed(glfw::Key::A) {
            camera.position.x -= DELTA;
        }
        if window.is_pressed(glfw::Key::S) {
            camera.position.z -= DELTA;
        }
        if window.is_pressed(glfw::Key::D) {
            camera.position.x += DELTA;
        }
        if window.is_pressed(glfw::Key::Space) {
            camera.position.y += DELTA;
        }
        if window.is_pressed(glfw::Key::LeftShift) {
            camera.position.y -= DELTA;
        }

        unsafe {
            let (width, height) = window.get_size();
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Uniform1f(width_uniform, width as f32);
            gl::Uniform1f(height_uniform, height as f32);

            camera.position.update_uniform(camera_position_uniform);
            camera.angles.update_uniform(camera_angles_uniform);
            gl::Uniform1f(camera_fov_uniform, camera.fov);

            for i in 0..spheres.len() {
                let sphere = &spheres[i];
                let (
                    position_uniform,
                    radius_uniform,
                    albedo_uniform,
                    roughness_uniform,
                    metallic_uniform,
                ) = sphere_uniforms[i];

                sphere.position.update_uniform(position_uniform);
                gl::Uniform1f(radius_uniform, sphere.radius);
                sphere.material.albedo.update_uniform(albedo_uniform);
                gl::Uniform1f(roughness_uniform, sphere.material.roughness);
                gl::Uniform1f(metallic_uniform, sphere.material.metallic);
            }

            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        window.update();
    }
    program.unbind();
}

pub fn create_uniform(shader_program: u32, uniform_name: String) -> (String, i32) {
    let uniform_location = unsafe {
        let cstring = std::ffi::CString::new(uniform_name.as_bytes()).unwrap();
        gl::GetUniformLocation(shader_program, cstring.as_ptr())
    };

    if uniform_location < 0 {
        panic!("Cannot create uniform {}", uniform_name);
    }
    (uniform_name, uniform_location)
}

fn create_viewport() {
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

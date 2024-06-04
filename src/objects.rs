use super::vec3::Vec3;

use serde::{Serialize, Deserialize};

pub trait Uniforms<const N: usize> {
    fn get_uniforms<'a>(shader_program: u32) -> [(&'a str, i32); N];
    fn update_uniforms(&self, uniforms: &[(&str, i32); N]);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub fov: f32,
}

impl Uniforms<2> for Camera {
    fn get_uniforms<'a>(shader_program: u32) -> [(&'a str, i32); 2] {
        [
            create_uniform(shader_program, "camera.position"),
            create_uniform(shader_program, "camera.fov"),
        ]
    }

    fn update_uniforms(&self, uniforms: &[(&str, i32); 2]) {
        let [(_, position_attribute_location), (_, fov_attribute_location)] = *uniforms;

        unsafe {
            gl::Uniform3f(
                position_attribute_location,
                self.position.x,
                self.position.y,
                self.position.z,
            );
            gl::Uniform1f(fov_attribute_location, self.fov);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub albedo: Vec3,
    pub metallic: f32,
    pub roughness: f32,
}

impl Uniforms<3> for Material {
    fn get_uniforms<'a>(shader_program: u32) -> [(&'a str, i32); 3] {
        [
            create_uniform(shader_program, "sphere.material.albedo"),
            create_uniform(shader_program, "sphere.material.metallic"),
            create_uniform(shader_program, "sphere.material.roughness"),
        ]
    }

    fn update_uniforms(&self, uniforms: &[(&str, i32); 3]) {
        let [(_, albedo_attribute_location), (_, metallic_attribute_location), (_, roughness_attribute_location)] =
            *uniforms;

        unsafe {
            gl::Uniform3f(
                albedo_attribute_location,
                self.albedo.x,
                self.albedo.y,
                self.albedo.z,
            );
            gl::Uniform1f(metallic_attribute_location, self.metallic);
            gl::Uniform1f(roughness_attribute_location, self.roughness);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Uniforms<2> for Sphere {
    fn get_uniforms<'a>(shader_program: u32) -> [(&'a str, i32); 2] {
        [
            create_uniform(shader_program, "sphere.position"),
            create_uniform(shader_program, "sphere.radius"),
        ]
    }

    fn update_uniforms(&self, uniforms: &[(&str, i32); 2]) {
        let [(_, position_attribute_location), (_, radius_attribute_location)] = *uniforms;

        unsafe {
            gl::Uniform3f(
                position_attribute_location,
                self.position.x,
                self.position.y,
                self.position.z,
            );
            gl::Uniform1f(radius_attribute_location, self.radius);
        }
    }
}

pub fn create_uniform(shader_program: u32, uniform_name: &str) -> (&str, i32) {
    let uniform_location = unsafe {
        let cstring = std::ffi::CString::new(uniform_name).unwrap();
        gl::GetUniformLocation(shader_program, cstring.as_ptr())
    };

    if uniform_location < 0 {
        panic!("Cannot create uniform {}", uniform_name);
    }
    (uniform_name, uniform_location)
}

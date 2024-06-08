use super::vec3::Vec3;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub angles: Vec3,
    pub fov: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub albedo: Vec3,
    pub metallic: f32,
    pub roughness: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material: Material,
}

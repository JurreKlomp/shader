use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(self) -> f32 {
        f32::sqrt(Self::dot(self, self))
    }

    pub fn normalize(v: Vec3) -> Self {
        if v.magnitude() == 0.0 {
            return v;
        }
        v / v.magnitude()
    }

    pub fn dot(v: Vec3, w: Vec3) -> f32 {
        v.x * w.x + v.y * w.y + v.z * w.z
    }

    pub fn cross(v: Vec3, w: Vec3) -> Self {
        Self::new(
            v.y * w.z - v.z * w.y,
            v.z * w.x - v.x * w.z,
            v.x * w.y - v.y * w.x,
        )
    }

    pub fn rotate(&self, angles: Vec3) -> Self {
        let cos = Vec3::new(f32::cos(angles.x), f32::cos(angles.y), f32::cos(angles.z));
        let sin = Vec3::new(f32::sin(angles.x), f32::sin(angles.y), f32::sin(angles.z));
        Self::new(
            (cos.x * cos.y) * self.x
                + (cos.x * sin.y * sin.z - sin.x * cos.y) * self.y
                + (cos.x * sin.y * cos.z + sin.x * sin.z) * self.z,
            (sin.x * cos.y) * self.x
                + (sin.x * sin.y * sin.z + cos.x * cos.z) * self.y
                + (sin.x * sin.y * cos.z - cos.x * sin.z) * self.z,
            (-sin.y) * self.x + (cos.y * sin.z) * self.y + (cos.y * cos.z) * self.z,
        )
    }

    pub fn update_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.x, self.y, self.z);
        }
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Vec3::dot(*other, *other).partial_cmp(&Vec3::dot(*self, *self))
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

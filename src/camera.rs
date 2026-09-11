use crate::{ray::Ray, vec3::Vec3};
pub struct Camera {
    origin: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    scale: f32,
}
impl Camera {
    pub fn look_at(origin: Vec3, target: Vec3, world_up: Vec3, fov: f32) -> Self {
        let forward = (target - origin).normalize();
        let right = forward.cross(world_up).normalize();
        let up = right.cross(forward).normalize();
        Self {
            origin,
            forward,
            right,
            up,
            scale: (fov.to_radians() / 2.0).tan(),
        }
    }
    pub fn ray(&self, u: f32, v: f32, aspect: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.forward
                + self.right * (u * aspect * self.scale)
                + self.up * (v * self.scale))
                .normalize(),
        }
    }
}

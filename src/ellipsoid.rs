use std::f32::consts::PI;

use crate::{cube::Hit, material::Material, ray::Ray, vec3::Vec3};

/// Elipsoide escalado por eje. Permite carrocerías y ruedas redondeadas sin
/// incorporar una biblioteca de modelos 3D.
pub struct Ellipsoid {
    pub center: Vec3,
    pub radii: Vec3,
    pub uv_offset: f32,
    pub material: Material,
}

impl Ellipsoid {
    pub fn intersect(&self, ray: Ray) -> Option<Hit<'_>> {
        let origin = ray.origin - self.center;
        let o = Vec3::new(
            origin.x / self.radii.x,
            origin.y / self.radii.y,
            origin.z / self.radii.z,
        );
        let d = Vec3::new(
            ray.direction.x / self.radii.x,
            ray.direction.y / self.radii.y,
            ray.direction.z / self.radii.z,
        );
        let a = d.dot(d);
        let half_b = o.dot(d);
        let discriminant = half_b * half_b - a * (o.dot(o) - 1.0);
        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let mut distance = (-half_b - root) / a;
        if distance < 0.001 {
            distance = (-half_b + root) / a;
        }
        if distance < 0.001 {
            return None;
        }

        let point = ray.at(distance);
        let local = point - self.center;
        let normal = Vec3::new(
            local.x / (self.radii.x * self.radii.x),
            local.y / (self.radii.y * self.radii.y),
            local.z / (self.radii.z * self.radii.z),
        )
        .normalize();
        let unit = Vec3::new(
            local.x / self.radii.x,
            local.y / self.radii.y,
            local.z / self.radii.z,
        );
        let u = 0.5 + unit.z.atan2(unit.x) / (2.0 * PI);
        let v = 0.5 - unit.y.clamp(-1.0, 1.0).asin() / PI;

        Some(Hit {
            distance,
            point,
            normal,
            u: u + self.uv_offset,
            v,
            material: &self.material,
        })
    }
}

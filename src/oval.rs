use crate::{cube::Hit, material::Material, ray::Ray, vec3::Vec3};

/// Superficie plana con forma de anillo elíptico. Es la base de la pista ovalada.
pub struct OvalRing {
    pub center: Vec3,
    pub outer_radii: (f32, f32),
    pub inner_radii: (f32, f32),
    pub material: Material,
}

impl OvalRing {
    pub fn intersect(&self, ray: Ray) -> Option<Hit<'_>> {
        if ray.direction.y.abs() < 0.00001 {
            return None;
        }
        let distance = (self.center.y - ray.origin.y) / ray.direction.y;
        if distance < 0.001 {
            return None;
        }
        let point = ray.at(distance);
        let x = point.x - self.center.x;
        let z = point.z - self.center.z;
        let outer = (x / self.outer_radii.0).powi(2) + (z / self.outer_radii.1).powi(2);
        let inner = (x / self.inner_radii.0).powi(2) + (z / self.inner_radii.1).powi(2);
        if outer > 1.0 || inner < 1.0 {
            return None;
        }
        let u = x;
        let v = z;
        Some(Hit {
            distance,
            point,
            normal: if ray.direction.y < 0.0 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(0.0, -1.0, 0.0)
            },
            u,
            v,
            material: &self.material,
        })
    }
}

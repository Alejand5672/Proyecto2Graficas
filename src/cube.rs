use crate::{material::Material, ray::Ray, vec3::Vec3};
pub struct Hit<'a> {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub u: f32,
    pub v: f32,
    pub material: &'a Material,
}
pub struct Cube {
    pub center: Vec3,
    /// Permite escalar un bloque en cada eje; sigue siendo una pieza cúbica
    /// base, útil para formar la silueta inicial del automóvil y el escenario.
    pub half_size: Vec3,
    pub material: Material,
}
impl Cube {
    pub fn intersect(&self, ray: Ray) -> Option<Hit<'_>> {
        let o = ray.origin - self.center;
        let (mut near, mut far) = (-f32::INFINITY, f32::INFINITY);
        for (a, b, half) in [
            (o.x, ray.direction.x, self.half_size.x),
            (o.y, ray.direction.y, self.half_size.y),
            (o.z, ray.direction.z, self.half_size.z),
        ] {
            if b.abs() < 0.00001 {
                if a.abs() > half {
                    return None;
                }
            } else {
                let t0 = (-half - a) / b;
                let t1 = (half - a) / b;
                near = near.max(t0.min(t1));
                far = far.min(t0.max(t1));
            }
        }
        if near > far || far < 0.001 {
            return None;
        }
        let d = if near > 0.001 { near } else { far };
        let p = o + ray.direction * d;
        let n = if p.x.abs() >= p.y.abs() && p.x.abs() >= p.z.abs() {
            Vec3::new(p.x.signum(), 0.0, 0.0)
        } else if p.y.abs() >= p.z.abs() {
            Vec3::new(0.0, p.y.signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, p.z.signum())
        };
        let (u, v) = if n.x.abs() > 0.5 {
            (
                p.z / (2.0 * self.half_size.z) + 0.5,
                p.y / (2.0 * self.half_size.y) + 0.5,
            )
        } else if n.y.abs() > 0.5 {
            (
                p.x / (2.0 * self.half_size.x) + 0.5,
                p.z / (2.0 * self.half_size.z) + 0.5,
            )
        } else {
            (
                p.x / (2.0 * self.half_size.x) + 0.5,
                p.y / (2.0 * self.half_size.y) + 0.5,
            )
        };
        Some(Hit {
            distance: d,
            point: ray.at(d),
            normal: n,
            u,
            v,
            material: &self.material,
        })
    }
}

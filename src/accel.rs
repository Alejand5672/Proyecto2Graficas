//! BVH de cajas y elipsoides. Los anillos planos se consultan por separado.
use crate::{
    cube::{Cube, Hit},
    ellipsoid::Ellipsoid,
    ray::Ray,
    vec3::Vec3,
};
pub struct Bvh {
    min: Vec3,
    max: Vec3,
    ids: Vec<usize>,
    children: Option<Box<(Bvh, Bvh)>>,
}
fn component(v: Vec3, a: usize) -> f32 {
    match a {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    }
}
impl Bvh {
    pub fn build(c: &[Cube], e: &[Ellipsoid]) -> Self {
        Self::node((0..c.len() + e.len()).collect(), c, e)
    }
    fn bounds(id: usize, c: &[Cube], e: &[Ellipsoid]) -> (Vec3, Vec3) {
        if id < c.len() {
            (c[id].center, c[id].half_size)
        } else {
            let o = &e[id - c.len()];
            (o.center, o.radii)
        }
    }
    fn node(mut ids: Vec<usize>, c: &[Cube], e: &[Ellipsoid]) -> Self {
        let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = -min;
        for &id in &ids {
            let (p, h) = Self::bounds(id, c, e);
            let a = p - h;
            let b = p + h;
            min = Vec3::new(min.x.min(a.x), min.y.min(a.y), min.z.min(a.z));
            max = Vec3::new(max.x.max(b.x), max.y.max(b.y), max.z.max(b.z));
        }
        if ids.len() <= 6 {
            return Self {
                min,
                max,
                ids,
                children: None,
            };
        }
        let span = max - min;
        let axis = if span.x > span.y && span.x > span.z {
            0
        } else if span.y > span.z {
            1
        } else {
            2
        };
        ids.sort_by(|a, b| {
            component(Self::bounds(*a, c, e).0, axis)
                .total_cmp(&component(Self::bounds(*b, c, e).0, axis))
        });
        let right = ids.split_off(ids.len() / 2);
        let children = Some(Box::new((Self::node(ids, c, e), Self::node(right, c, e))));
        Self {
            min,
            max,
            ids: Vec::new(),
            children,
        }
    }
    pub fn hit<'a>(
        &self,
        ray: Ray,
        c: &'a [Cube],
        e: &'a [Ellipsoid],
        limit: f32,
    ) -> Option<Hit<'a>> {
        let mut lo = 0.001f32;
        let mut hi = limit;
        for a in 0..3 {
            let o = component(ray.origin, a);
            let d = component(ray.direction, a);
            let min = component(self.min, a);
            let max = component(self.max, a);
            if d.abs() < 1e-8 {
                if o < min || o > max {
                    return None;
                }
            } else {
                let t0 = (min - o) / d;
                let t1 = (max - o) / d;
                lo = lo.max(t0.min(t1));
                hi = hi.min(t0.max(t1));
                if lo > hi {
                    return None;
                }
            }
        }
        if let Some(children) = &self.children {
            let left = children.0.hit(ray, c, e, limit);
            let bound = left.as_ref().map_or(limit, |h| h.distance);
            children.1.hit(ray, c, e, bound).or(left)
        } else {
            self.ids
                .iter()
                .filter_map(|&id| {
                    if id < c.len() {
                        c[id].intersect(ray)
                    } else {
                        e[id - c.len()].intersect(ray)
                    }
                })
                .filter(|h| h.distance < limit)
                .min_by(|a, b| a.distance.total_cmp(&b.distance))
        }
    }
}

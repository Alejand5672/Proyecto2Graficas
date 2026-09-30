use crate::{
    camera::Camera,
    color::{Color, Image},
    cube::Hit,
    ray::Ray,
    scene::Scene,
};
#[derive(Clone, Copy)]
pub struct RenderConfig {
    pub width: usize,
    pub height: usize,
    pub max_bounces: u8,
}
pub fn render(scene: &Scene, camera: &Camera, c: RenderConfig) -> Image {
    let mut image = Image::new(c.width, c.height);
    let aspect = c.width as f32 / c.height as f32;
    let workers = std::thread::available_parallelism()
        .map_or(1, |count| count.get())
        .min(c.height.max(1));
    let rows_per_worker = c.height.div_ceil(workers);

    std::thread::scope(|scope| {
        let mut jobs = Vec::with_capacity(workers);
        for worker in 0..workers {
            let start_y = worker * rows_per_worker;
            let end_y = (start_y + rows_per_worker).min(c.height);
            if start_y >= end_y {
                continue;
            }
            jobs.push(scope.spawn(move || {
                let mut pixels = Vec::with_capacity((end_y - start_y) * c.width);
                for y in start_y..end_y {
                    for x in 0..c.width {
                        let u = 2.0 * ((x as f32 + 0.5) / c.width as f32) - 1.0;
                        let v = 1.0 - 2.0 * ((y as f32 + 0.5) / c.height as f32);
                        let value = if c.max_bounces >= 4 {
                            let mut sum = Color::BLACK;
                            for (dx, dy) in
                                [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)]
                            {
                                sum = sum
                                    + trace(
                                        scene,
                                        camera.ray(
                                            u + dx * 2.0 / c.width as f32,
                                            v + dy * 2.0 / c.height as f32,
                                            aspect,
                                        ),
                                        c.max_bounces,
                                    );
                            }
                            sum * 0.25
                        } else {
                            trace(scene, camera.ray(u, v, aspect), c.max_bounces)
                        };
                        pixels.push(value);
                    }
                }
                (start_y, pixels)
            }));
        }
        for job in jobs {
            let (start_y, pixels) = job.join().expect("falló un hilo de render");
            let offset = start_y * c.width;
            image.pixels[offset..offset + pixels.len()].copy_from_slice(&pixels);
        }
    });
    image
}
fn trace(scene: &Scene, ray: Ray, depth: u8) -> Color {
    match scene.intersect(ray) {
        Some(hit) => shade(scene, ray, hit, depth),
        None => scene.sky_color(ray.direction),
    }
}
fn shade(scene: &Scene, ray: Ray, hit: Hit<'_>, depth: u8) -> Color {
    let m = hit.material;
    let entering = ray.direction.dot(hit.normal) < 0.0;
    let normal = if entering { hit.normal } else { -hit.normal };
    let mut color = m.albedo(hit.u, hit.v) * (m.ambient + 0.10 * (normal.y + 1.0));
    for light in &scene.lights {
        let toward = light.position - hit.point;
        let distance = toward.length();
        let direction = toward / distance;
        let shadow = Ray {
            origin: hit.point + normal * 0.002,
            direction,
        };
        let mut visibility = 0.0;
        for offset in [-0.7f32, 0.7] {
            let sample = Ray {
                origin: shadow.origin,
                direction: (toward + crate::vec3::Vec3::new(offset, 0.0, -offset)).normalize(),
            };
            visibility += match scene.intersect(sample) {
                Some(h) if h.distance < distance => h.material.transparency * 0.6,
                _ => 1.0,
            } * 0.5;
        }
        if visibility > 0.0 {
            let diffuse = normal.dot(direction).max(0.0) * m.diffuse;
            let spec = (-direction)
                .reflect(normal)
                .dot(-ray.direction)
                .max(0.0)
                .powf(m.shininess)
                * m.specular;
            color = color
                + m.albedo(hit.u, hit.v) * light.color * (diffuse * light.intensity * visibility)
                + light.color * (spec * light.intensity * visibility);
        }
    }
    if depth > 0 && m.reflectivity > 0.0 {
        color = color * (1.0 - m.reflectivity)
            + trace(
                scene,
                Ray {
                    origin: hit.point + normal * 0.002,
                    direction: ray.direction.reflect(normal).normalize(),
                },
                depth - 1,
            ) * m.reflectivity
    }
    if depth > 0 && m.transparency > 0.0 {
        let eta = if entering { 1.0 / m.ior } else { m.ior };
        if let Some(direction) = ray.direction.refract(normal, eta) {
            color = color * (1.0 - m.transparency)
                + trace(
                    scene,
                    Ray {
                        origin: hit.point - normal * 0.002,
                        direction: direction.normalize(),
                    },
                    depth - 1,
                ) * m.transparency
        } else {
            color = trace(
                scene,
                Ray {
                    origin: hit.point + normal * 0.002,
                    direction: ray.direction.reflect(normal),
                },
                depth - 1,
            );
        }
    }
    color
}

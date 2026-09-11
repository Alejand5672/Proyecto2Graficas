use crate::{
    color::Color,
    cube::{Cube, Hit},
    material::Material,
    ray::Ray,
    texture::Texture,
    vec3::Vec3,
};
pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}
pub struct Skybox {
    pub horizon: Color,
    pub zenith: Color,
}
pub struct Scene {
    pub cubes: Vec<Cube>,
    pub lights: Vec<Light>,
    pub skybox: Skybox,
}
impl Scene {
    pub fn base() -> Self {
        let red = Material::matte(Texture::Solid(Color::new(0.82, 0.015, 0.05)));
        let red_dark = Material::matte(Texture::Solid(Color::new(0.42, 0.008, 0.02)));
        let tire = Material::matte(Texture::Solid(Color::new(0.015, 0.018, 0.022)));
        let glass = Material::glass(Texture::Solid(Color::new(0.38, 0.76, 0.88)));
        let asphalt = Material::matte(Texture::Checker {
            first: Color::new(0.12, 0.14, 0.16),
            second: Color::new(0.08, 0.09, 0.10),
            scale: 3.0,
        });
        let sand = Material::matte(Texture::Solid(Color::new(0.55, 0.22, 0.07)));
        let mesa = Material::matte(Texture::Solid(Color::new(0.31, 0.075, 0.025)));
        Self {
            // Boceto estático: pista, desierto y silueta por bloques de un auto saltando.
            cubes: vec![
                Cube {
                    center: Vec3::new(0.0, -1.25, -5.0),
                    half_size: Vec3::new(8.0, 0.18, 12.0),
                    material: asphalt,
                },
                Cube {
                    center: Vec3::new(-5.4, -0.15, -12.0),
                    half_size: Vec3::new(2.2, 1.0, 1.1),
                    material: mesa.clone(),
                },
                Cube {
                    center: Vec3::new(4.7, -0.35, -13.0),
                    half_size: Vec3::new(2.8, 0.8, 1.0),
                    material: mesa,
                },
                Cube {
                    center: Vec3::new(0.0, -0.95, -11.0),
                    half_size: Vec3::new(8.0, 0.25, 2.0),
                    material: sand,
                },
                // Carro: bloque principal, cofre, cabina/ventana y cuatro ruedas cuadradas provisionales.
                Cube {
                    center: Vec3::new(0.0, 0.20, -4.4),
                    half_size: Vec3::new(1.35, 0.48, 0.70),
                    material: red,
                },
                Cube {
                    center: Vec3::new(0.0, 0.68, -4.72),
                    half_size: Vec3::new(0.74, 0.43, 0.44),
                    material: glass,
                },
                Cube {
                    center: Vec3::new(0.0, 0.32, -5.02),
                    half_size: Vec3::new(1.06, 0.30, 0.28),
                    material: red_dark,
                },
                Cube {
                    center: Vec3::new(-1.14, -0.34, -4.02),
                    half_size: Vec3::new(0.27, 0.48, 0.22),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(1.14, -0.34, -4.02),
                    half_size: Vec3::new(0.27, 0.48, 0.22),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(-1.14, -0.34, -4.83),
                    half_size: Vec3::new(0.27, 0.48, 0.22),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(1.14, -0.34, -4.83),
                    half_size: Vec3::new(0.27, 0.48, 0.22),
                    material: tire,
                },
            ],
            lights: vec![Light {
                position: Vec3::new(-4.0, 7.0, 2.0),
                color: Color::new(1.0, 0.93, 0.82),
                intensity: 1.0,
            }],
            skybox: Skybox {
                horizon: Color::new(0.72, 0.28, 0.08),
                zenith: Color::new(0.11, 0.28, 0.52),
            },
        }
    }
    pub fn intersect(&self, ray: Ray) -> Option<Hit<'_>> {
        self.cubes
            .iter()
            .filter_map(|c| c.intersect(ray))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }
    pub fn sky_color(&self, d: Vec3) -> Color {
        self.skybox
            .horizon
            .mix(self.skybox.zenith, ((d.y + 1.0) * 0.5).powf(0.8))
    }
}

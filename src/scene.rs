use crate::{
    color::Color,
    cube::{Cube, Hit},
    ellipsoid::Ellipsoid,
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
    pub ellipsoids: Vec<Ellipsoid>,
    pub lights: Vec<Light>,
    pub skybox: Skybox,
}
impl Scene {
    pub fn base() -> Self {
        // Cinco materiales con textura y parámetros propios: asfalto, carro,
        // llanta, cristal y desierto. El cristal aporta refracción y reflexión.
        let mut car = Material::matte(Texture::Stripes {
            first: Color::new(0.86, 0.012, 0.035),
            second: Color::new(0.48, 0.005, 0.015),
            scale: 7.0,
        });
        car.specular = 0.62;
        car.shininess = 80.0;
        car.reflectivity = 0.18;
        let mut tire = Material::matte(Texture::Checker {
            first: Color::new(0.018, 0.022, 0.028),
            second: Color::new(0.08, 0.085, 0.09),
            scale: 8.0,
        });
        tire.specular = 0.12;
        let glass = Material::glass(Texture::Solid(Color::new(0.78, 0.91, 0.96)));
        let asphalt = Material::matte(Texture::Checker {
            first: Color::new(0.12, 0.14, 0.16),
            second: Color::new(0.08, 0.09, 0.10),
            scale: 3.0,
        });
        let sand = Material::matte(Texture::Stripes {
            first: Color::new(0.12, 0.34, 0.08),
            second: Color::new(0.32, 0.48, 0.12),
            scale: 9.0,
        });
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
                    material: sand.clone(),
                },
                Cube {
                    center: Vec3::new(4.7, -0.35, -13.0),
                    half_size: Vec3::new(2.8, 0.8, 1.0),
                    material: sand.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, -0.95, -11.0),
                    half_size: Vec3::new(8.0, 0.25, 2.0),
                    material: sand.clone(),
                },
                // Rayo McQueen estilizado: los bloques escalonados forman un
                // auto bajo, con cofre, cabina, cara, ojos y ruedas con rines.
                // El frente queda hacia la cámara inicial (eje Z positivo).
                Cube {
                    center: Vec3::new(0.0, 0.02, -4.35),
                    half_size: Vec3::new(1.38, 0.44, 1.02),
                    material: car.clone(),
                },
                // Cofre en dos alturas: crea una nariz inclinada en vez de un bloque único.
                Cube {
                    center: Vec3::new(0.0, 0.38, -3.64),
                    half_size: Vec3::new(1.16, 0.20, 0.38),
                    material: car.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, 0.64, -4.42),
                    half_size: Vec3::new(0.82, 0.34, 0.48),
                    material: car.clone(),
                },
                // Parabrisas y ojos, colocados sobre la parte frontal de la cabina.
                Cube {
                    center: Vec3::new(-0.39, 0.74, -3.76),
                    half_size: Vec3::new(0.30, 0.24, 0.035),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(0.39, 0.74, -3.76),
                    half_size: Vec3::new(0.30, 0.24, 0.035),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(-0.39, 0.74, -3.71),
                    half_size: Vec3::new(0.075, 0.105, 0.025),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(0.39, 0.74, -3.71),
                    half_size: Vec3::new(0.075, 0.105, 0.025),
                    material: tire.clone(),
                },
                // Faros, sonrisa oscura y defensa roja para reconocer el personaje.
                Cube {
                    center: Vec3::new(-0.86, 0.14, -3.16),
                    half_size: Vec3::new(0.20, 0.12, 0.035),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(0.86, 0.14, -3.16),
                    half_size: Vec3::new(0.20, 0.12, 0.035),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, -0.13, -3.15),
                    half_size: Vec3::new(0.48, 0.075, 0.04),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, -0.34, -3.17),
                    half_size: Vec3::new(1.05, 0.10, 0.12),
                    material: car.clone(),
                },
                // Cuatro ruedas cuadradas con un rin claro superpuesto.
                Cube {
                    center: Vec3::new(-1.22, -0.38, -3.72),
                    half_size: Vec3::new(0.25, 0.42, 0.25),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(1.22, -0.38, -3.72),
                    half_size: Vec3::new(0.25, 0.42, 0.25),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(-1.22, -0.38, -4.95),
                    half_size: Vec3::new(0.25, 0.42, 0.25),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(1.22, -0.38, -4.95),
                    half_size: Vec3::new(0.25, 0.42, 0.25),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(-1.48, -0.38, -3.72),
                    half_size: Vec3::new(0.025, 0.18, 0.11),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(1.48, -0.38, -3.72),
                    half_size: Vec3::new(0.025, 0.18, 0.11),
                    material: glass.clone(),
                },
                // Estadio: barreras interiores y gradas escalonadas a ambos lados.
                Cube {
                    center: Vec3::new(-3.75, -0.72, -6.0),
                    half_size: Vec3::new(0.14, 0.42, 9.0),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(3.75, -0.72, -6.0),
                    half_size: Vec3::new(0.14, 0.42, 9.0),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(-4.65, -0.55, -7.0),
                    half_size: Vec3::new(0.72, 0.24, 8.0),
                    material: sand.clone(),
                },
                Cube {
                    center: Vec3::new(4.65, -0.55, -7.0),
                    half_size: Vec3::new(0.72, 0.24, 8.0),
                    material: sand.clone(),
                },
                Cube {
                    center: Vec3::new(-5.15, -0.12, -7.0),
                    half_size: Vec3::new(0.72, 0.20, 8.0),
                    material: car.clone(),
                },
                Cube {
                    center: Vec3::new(5.15, -0.12, -7.0),
                    half_size: Vec3::new(0.72, 0.20, 8.0),
                    material: car.clone(),
                },
                Cube {
                    center: Vec3::new(-5.65, 0.29, -7.0),
                    half_size: Vec3::new(0.72, 0.20, 8.0),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(5.65, 0.29, -7.0),
                    half_size: Vec3::new(0.72, 0.20, 8.0),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(-6.15, 0.70, -7.0),
                    half_size: Vec3::new(0.72, 0.20, 8.0),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(6.15, 0.70, -7.0),
                    half_size: Vec3::new(0.72, 0.20, 8.0),
                    material: tire.clone(),
                },
                // Torres de iluminación que enmarcan la recta principal.
                Cube {
                    center: Vec3::new(-6.7, 2.1, -2.0),
                    half_size: Vec3::new(0.12, 1.4, 0.12),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(6.7, 2.1, -2.0),
                    half_size: Vec3::new(0.12, 1.4, 0.12),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(-6.7, 3.45, -2.0),
                    half_size: Vec3::new(0.55, 0.18, 0.18),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(6.7, 3.45, -2.0),
                    half_size: Vec3::new(0.55, 0.18, 0.18),
                    material: glass.clone(),
                },
                // Zona interior de pits: garajes, toldos y vehículos de apoyo.
                Cube {
                    center: Vec3::new(-1.65, -0.62, -9.1),
                    half_size: Vec3::new(1.15, 0.34, 0.72),
                    material: car.clone(),
                },
                Cube {
                    center: Vec3::new(1.35, -0.62, -9.1),
                    half_size: Vec3::new(1.15, 0.34, 0.72),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(-1.65, -0.18, -9.1),
                    half_size: Vec3::new(1.30, 0.10, 0.86),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(1.35, -0.18, -9.1),
                    half_size: Vec3::new(1.30, 0.10, 0.86),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(-2.0, -0.73, -11.2),
                    half_size: Vec3::new(0.72, 0.22, 0.30),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, -0.73, -11.5),
                    half_size: Vec3::new(0.72, 0.22, 0.30),
                    material: car.clone(),
                },
                Cube {
                    center: Vec3::new(2.0, -0.73, -11.2),
                    half_size: Vec3::new(0.72, 0.22, 0.30),
                    material: glass.clone(),
                },
                // Torre central y pantalla gigante, inspiradas en el Motor Speedway.
                Cube {
                    center: Vec3::new(0.0, 1.25, -12.4),
                    half_size: Vec3::new(0.34, 2.20, 0.34),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, 3.20, -12.1),
                    half_size: Vec3::new(1.35, 0.82, 0.18),
                    material: car.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, 3.20, -11.90),
                    half_size: Vec3::new(1.12, 0.62, 0.035),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(-1.28, 4.08, -12.1),
                    half_size: Vec3::new(0.12, 0.22, 0.30),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(1.28, 4.08, -12.1),
                    half_size: Vec3::new(0.12, 0.22, 0.30),
                    material: tire.clone(),
                },
                // Línea de salida a cuadros sobre la recta principal.
                Cube {
                    center: Vec3::new(-2.45, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(-1.75, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(-1.05, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(-0.35, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(0.35, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(1.05, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(1.75, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(2.45, -1.045, -5.75),
                    half_size: Vec3::new(0.34, 0.018, 0.34),
                    material: tire.clone(),
                },
                // Arco de meta construido con cubos: aporta escala a la pista.
                Cube {
                    center: Vec3::new(-3.3, 0.1, -6.8),
                    half_size: Vec3::new(0.16, 1.35, 0.16),
                    material: sand.clone(),
                },
                Cube {
                    center: Vec3::new(3.3, 0.1, -6.8),
                    half_size: Vec3::new(0.16, 1.35, 0.16),
                    material: sand.clone(),
                },
                Cube {
                    center: Vec3::new(0.0, 1.28, -6.8),
                    half_size: Vec3::new(3.45, 0.16, 0.16),
                    material: tire.clone(),
                },
                Cube {
                    center: Vec3::new(-0.55, 1.28, -6.60),
                    half_size: Vec3::new(0.28, 0.15, 0.03),
                    material: glass.clone(),
                },
                Cube {
                    center: Vec3::new(0.55, 1.28, -6.60),
                    half_size: Vec3::new(0.28, 0.15, 0.03),
                    material: car.clone(),
                },
            ],
            ellipsoids: vec![
                // Isla verde aplanada: deja visible el asfalto alrededor y sugiere el óvalo.
                Ellipsoid {
                    center: Vec3::new(0.0, -1.00, -10.4),
                    radii: Vec3::new(3.05, 0.13, 4.35),
                    material: sand.clone(),
                },
                // Volúmenes principales redondeados de Rayo McQueen.
                Ellipsoid {
                    center: Vec3::new(0.0, 0.02, -4.35),
                    radii: Vec3::new(1.46, 0.50, 1.18),
                    material: car.clone(),
                },
                Ellipsoid {
                    center: Vec3::new(0.0, 0.34, -3.66),
                    radii: Vec3::new(1.25, 0.28, 0.60),
                    material: car.clone(),
                },
                Ellipsoid {
                    center: Vec3::new(0.0, 0.66, -4.45),
                    radii: Vec3::new(0.88, 0.48, 0.66),
                    material: car.clone(),
                },
                // Ruedas elipsoidales: aplanadas en X para conservar su orientación lateral.
                Ellipsoid {
                    center: Vec3::new(-1.32, -0.38, -3.72),
                    radii: Vec3::new(0.22, 0.45, 0.45),
                    material: tire.clone(),
                },
                Ellipsoid {
                    center: Vec3::new(1.32, -0.38, -3.72),
                    radii: Vec3::new(0.22, 0.45, 0.45),
                    material: tire.clone(),
                },
                Ellipsoid {
                    center: Vec3::new(-1.32, -0.38, -4.95),
                    radii: Vec3::new(0.22, 0.45, 0.45),
                    material: tire.clone(),
                },
                Ellipsoid {
                    center: Vec3::new(1.32, -0.38, -4.95),
                    radii: Vec3::new(0.22, 0.45, 0.45),
                    material: tire,
                },
            ],
            lights: vec![
                Light {
                    position: Vec3::new(-4.0, 7.0, 2.0),
                    color: Color::new(1.0, 0.93, 0.82),
                    intensity: 0.95,
                },
                Light {
                    position: Vec3::new(6.7, 5.0, -9.0),
                    color: Color::new(0.72, 0.84, 1.0),
                    intensity: 0.45,
                },
            ],
            skybox: Skybox {
                horizon: Color::new(0.58, 0.72, 0.84),
                zenith: Color::new(0.08, 0.24, 0.48),
            },
        }
    }
    pub fn intersect(&self, ray: Ray) -> Option<Hit<'_>> {
        self.cubes
            .iter()
            .filter_map(|c| c.intersect(ray))
            .chain(self.ellipsoids.iter().filter_map(|e| e.intersect(ray)))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }
    pub fn sky_color(&self, d: Vec3) -> Color {
        self.skybox
            .horizon
            .mix(self.skybox.zenith, ((d.y + 1.0) * 0.5).powf(0.8))
    }
}

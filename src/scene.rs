use crate::{
    color::Color,
    cube::{Cube, Hit},
    ellipsoid::Ellipsoid,
    material::Material,
    oval::OvalRing,
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
    pub oval_rings: Vec<OvalRing>,
    pub lights: Vec<Light>,
    pub skybox: Skybox,
}

fn add_cube(cubes: &mut Vec<Cube>, center: Vec3, half_size: Vec3, material: &Material) {
    cubes.push(Cube {
        center,
        half_size,
        material: material.clone(),
    });
}

fn add_race_car(
    ellipsoids: &mut Vec<Ellipsoid>,
    x: f32,
    z: f32,
    scale: f32,
    paint_slot: f32,
    paint: &Material,
    glass: &Material,
    tire: &Material,
) {
    for (dx, dy, dz, rx, ry, rz, material, uv_offset) in [
        (0.0, -0.67, 0.0, 0.86, 0.40, 1.23, paint, paint_slot),
        (0.0, -0.46, -0.60, 0.76, 0.27, 0.63, paint, paint_slot),
        (0.0, -0.27, 0.29, 0.55, 0.32, 0.52, glass, 0.0),
        (-0.72, -0.74, -0.56, 0.22, 0.34, 0.34, tire, 0.0),
        (0.72, -0.74, -0.56, 0.22, 0.34, 0.34, tire, 0.0),
        (-0.72, -0.74, 0.59, 0.22, 0.34, 0.34, tire, 0.0),
        (0.72, -0.74, 0.59, 0.22, 0.34, 0.34, tire, 0.0),
    ] {
        ellipsoids.push(Ellipsoid {
            center: Vec3::new(x + dx * scale, dy, z + dz * scale),
            radii: Vec3::new(rx * scale, ry * scale, rz * scale),
            uv_offset,
            material: material.clone(),
        });
    }
}

impl Scene {
    pub fn base() -> Self {
        // Cinco materiales con texturas y parámetros independientes.
        let mut asphalt = Material::matte(Texture::Checker {
            first: Color::new(0.105, 0.115, 0.125),
            second: Color::new(0.075, 0.080, 0.090),
            scale: 18.0,
        });
        asphalt.specular = 0.28;
        asphalt.shininess = 48.0;
        asphalt.reflectivity = 0.06;
        let mut grass = Material::matte(Texture::Stripes {
            first: Color::new(0.075, 0.27, 0.055),
            second: Color::new(0.16, 0.38, 0.075),
            scale: 26.0,
        });
        grass.specular = 0.06;
        grass.shininess = 8.0;
        grass.reflectivity = 0.01;
        let mut car = Material::matte(Texture::RacePaint);
        car.specular = 0.68;
        car.shininess = 92.0;
        car.reflectivity = 0.20;
        let mut dark = Material::matte(Texture::Solid(Color::new(0.035, 0.045, 0.060)));
        dark.specular = 0.14;
        dark.shininess = 24.0;
        dark.reflectivity = 0.025;
        let glass = Material::glass(Texture::Stripes {
            first: Color::new(0.30, 0.62, 0.82),
            second: Color::new(0.72, 0.88, 0.95),
            scale: 6.0,
        });

        let mut cubes = Vec::new();
        // Terreno, pit lane y plataforma central.
        add_cube(
            &mut cubes,
            Vec3::new(0.0, -1.38, -8.0),
            Vec3::new(18.0, 0.28, 26.0),
            &grass,
        );
        add_cube(
            &mut cubes,
            Vec3::new(5.65, -1.075, -8.0),
            Vec3::new(0.82, 0.025, 11.8),
            &asphalt,
        );
        add_cube(
            &mut cubes,
            Vec3::new(2.25, -1.09, -8.0),
            Vec3::new(2.45, 0.035, 9.2),
            &glass,
        );

        // Muros y tribunas laterales escalonadas.
        for side in [-1.0_f32, 1.0] {
            add_cube(
                &mut cubes,
                Vec3::new(side * 10.75, -0.78, -8.0),
                Vec3::new(0.12, 0.36, 18.6),
                &glass,
            );
            for tier in 0..7 {
                let t = tier as f32;
                add_cube(
                    &mut cubes,
                    Vec3::new(side * (11.45 + t * 0.52), -0.72 + t * 0.43, -8.0),
                    Vec3::new(0.62, 0.18, 18.2),
                    if tier % 2 == 0 { &dark } else { &glass },
                );
            }
        }
        // Tribuna de la curva norte.
        for tier in 0..7 {
            let t = tier as f32;
            add_cube(
                &mut cubes,
                Vec3::new(0.0, -0.68 + t * 0.43, -27.35 - t * 0.52),
                Vec3::new(10.9 + t * 0.55, 0.18, 0.62),
                if tier % 2 == 0 { &dark } else { &glass },
            );
        }
        // Tribuna de la curva sur: completa el anillo del estadio.
        for tier in 0..7 {
            let t = tier as f32;
            add_cube(
                &mut cubes,
                Vec3::new(0.0, -0.68 + t * 0.43, 11.35 + t * 0.52),
                Vec3::new(10.9 + t * 0.55, 0.18, 0.62),
                if tier % 2 == 0 { &dark } else { &glass },
            );
        }
        // Público en tres niveles: suficiente densidad sin usar modelos externos.
        for row in 0..23 {
            let z = -25.0 + row as f32 * 1.5;
            for side in [-1.0_f32, 1.0] {
                for seat_tier in 0..3 {
                    let material = match (row + seat_tier) % 3 {
                        0 => &glass,
                        1 => &car,
                        _ => &dark,
                    };
                    add_cube(
                        &mut cubes,
                        Vec3::new(
                            side * (11.72 + seat_tier as f32 * 0.54),
                            0.04 + seat_tier as f32 * 0.43,
                            z,
                        ),
                        Vec3::new(0.24, 0.14, 0.48),
                        material,
                    );
                }
            }
        }

        // Boxes y edificios de pits.
        for bay in 0..7 {
            let z = -16.0 + bay as f32 * 2.55;
            add_cube(
                &mut cubes,
                Vec3::new(3.10, -0.63, z),
                Vec3::new(1.18, 0.43, 0.92),
                if bay % 2 == 0 { &dark } else { &glass },
            );
            add_cube(
                &mut cubes,
                Vec3::new(3.10, -0.12, z),
                Vec3::new(1.34, 0.09, 1.05),
                &glass,
            );
        }
        // Trailers y motorhomes del paddock.
        for row in 0..4 {
            for column in 0..5 {
                let x = -3.8 + column as f32 * 1.55;
                let z = -15.2 + row as f32 * 2.15;
                add_cube(
                    &mut cubes,
                    Vec3::new(x, -0.78, z),
                    Vec3::new(0.58, 0.23, 0.78),
                    if (row + column) % 2 == 0 {
                        &glass
                    } else {
                        &glass
                    },
                );
                add_cube(
                    &mut cubes,
                    Vec3::new(x, -0.48, z),
                    Vec3::new(0.66, 0.07, 0.86),
                    &dark,
                );
            }
        }

        // Torre de resultados y pantalla gigante.
        add_cube(
            &mut cubes,
            Vec3::new(-1.0, 1.25, -4.7),
            Vec3::new(0.34, 2.35, 0.34),
            &dark,
        );
        add_cube(
            &mut cubes,
            Vec3::new(-1.0, 3.42, -4.7),
            Vec3::new(1.42, 0.92, 0.28),
            &glass,
        );
        add_cube(
            &mut cubes,
            Vec3::new(-1.0, 3.42, -4.39),
            Vec3::new(1.18, 0.70, 0.035),
            &glass,
        );

        // Línea de salida a cuadros.
        for square in 0..8 {
            add_cube(
                &mut cubes,
                Vec3::new(7.05 + square as f32 * 0.43, -1.015, -5.3),
                Vec3::new(0.20, 0.018, 0.38),
                if square % 2 == 0 { &glass } else { &dark },
            );
        }
        // Detalles de Rayo McQueen junto a la meta. El volumen principal se
        // construye con elipsoides; estos cubos finos aportan acabado de carrera.
        add_cube(
            &mut cubes,
            Vec3::new(8.50, -0.35, -2.89),
            Vec3::new(0.82, 0.075, 0.11),
            &car,
        );
        add_cube(
            &mut cubes,
            Vec3::new(7.98, -0.48, -3.02),
            Vec3::new(0.07, 0.18, 0.07),
            &dark,
        );
        add_cube(
            &mut cubes,
            Vec3::new(9.02, -0.48, -3.02),
            Vec3::new(0.07, 0.18, 0.07),
            &dark,
        );
        // Franja del cofre, faros y pupilas visibles durante la inspección.
        add_cube(
            &mut cubes,
            Vec3::new(8.50, -0.245, -4.66),
            Vec3::new(0.11, 0.025, 0.38),
            &glass,
        );
        for x in [8.12_f32, 8.88] {
            add_cube(
                &mut cubes,
                Vec3::new(x, -0.46, -5.12),
                Vec3::new(0.16, 0.08, 0.035),
                &glass,
            );
        }
        for x in [8.27_f32, 8.73] {
            add_cube(
                &mut cubes,
                Vec3::new(x, 0.045, -3.78),
                Vec3::new(0.055, 0.035, 0.075),
                &dark,
            );
        }
        // Alerones oscuros y franjas claras de los rivales azul y verde.
        for (x, z) in [(9.72_f32, -3.30_f32), (7.18_f32, -3.45_f32)] {
            add_cube(
                &mut cubes,
                Vec3::new(x, -0.39, z + 0.92),
                Vec3::new(0.60, 0.06, 0.09),
                &dark,
            );
            add_cube(
                &mut cubes,
                Vec3::new(x, -0.27, z - 0.52),
                Vec3::new(0.075, 0.02, 0.30),
                &glass,
            );
        }
        // Torres de iluminación.
        for (x, z) in [(-10.8, -20.0), (10.8, -20.0), (-10.8, 4.0), (10.8, 4.0)] {
            add_cube(
                &mut cubes,
                Vec3::new(x, 2.2, z),
                Vec3::new(0.13, 2.8, 0.13),
                &dark,
            );
            add_cube(
                &mut cubes,
                Vec3::new(x, 4.85, z),
                Vec3::new(0.62, 0.19, 0.25),
                &glass,
            );
        }

        let mut ellipsoids = Vec::new();
        add_race_car(&mut ellipsoids, 8.50, -4.02, 1.0, 0.0, &car, &glass, &dark);
        add_race_car(&mut ellipsoids, 9.72, -3.30, 0.80, 1.0, &car, &glass, &dark);
        add_race_car(&mut ellipsoids, 7.18, -3.45, 0.80, 2.0, &car, &glass, &dark);

        Self {
            cubes,
            ellipsoids,
            oval_rings: vec![
                OvalRing {
                    center: Vec3::new(0.0, -1.055, -8.0),
                    outer_radii: (10.45, 18.45),
                    inner_radii: (6.72, 14.72),
                    material: asphalt.clone(),
                },
                OvalRing {
                    center: Vec3::new(0.0, -1.025, -8.0),
                    outer_radii: (10.28, 18.28),
                    inner_radii: (10.05, 18.05),
                    material: glass.clone(),
                },
                OvalRing {
                    center: Vec3::new(0.0, -1.024, -8.0),
                    outer_radii: (7.02, 15.02),
                    inner_radii: (6.76, 14.76),
                    material: glass.clone(),
                },
                OvalRing {
                    center: Vec3::new(0.0, -1.045, -8.0),
                    outer_radii: (5.92, 13.10),
                    inner_radii: (5.20, 12.35),
                    material: asphalt,
                },
            ],
            lights: vec![
                Light {
                    position: Vec3::new(-9.0, 15.0, 8.0),
                    color: Color::new(1.0, 0.95, 0.86),
                    intensity: 0.95,
                },
                Light {
                    position: Vec3::new(10.0, 11.0, -18.0),
                    color: Color::new(0.68, 0.82, 1.0),
                    intensity: 0.42,
                },
            ],
            skybox: Skybox {
                horizon: Color::new(0.58, 0.72, 0.84),
                zenith: Color::new(0.07, 0.20, 0.43),
            },
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Hit<'_>> {
        self.cubes
            .iter()
            .filter_map(|c| c.intersect(ray))
            .chain(self.ellipsoids.iter().filter_map(|e| e.intersect(ray)))
            .chain(self.oval_rings.iter().filter_map(|o| o.intersect(ray)))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }

    pub fn sky_color(&self, direction: Vec3) -> Color {
        self.skybox
            .horizon
            .mix(self.skybox.zenith, ((direction.y + 1.0) * 0.5).powf(0.8))
    }
}

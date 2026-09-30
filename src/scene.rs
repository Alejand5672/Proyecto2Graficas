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
    pub accel: crate::accel::Bvh,
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

impl Scene {
    pub fn base() -> Self {
        // Cinco materiales con texturas y parámetros independientes.
        let mut asphalt = Material::matte(Texture::Asphalt);
        asphalt.specular = 0.28;
        asphalt.shininess = 48.0;
        asphalt.reflectivity = 0.015;
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
        let glass = Material::glass(Texture::Solid(Color::new(0.32, 0.58, 0.72)));
        let concrete = Material::matte(Texture::Checker {
            first: Color::new(0.64, 0.66, 0.67),
            second: Color::new(0.60, 0.62, 0.63),
            scale: 75.0,
        });
        let white = Material::matte(Texture::Solid(Color::new(0.94, 0.91, 0.78)));
        let mut metal = Material::matte(Texture::Stripes {
            first: Color::new(0.29, 0.34, 0.40),
            second: Color::new(0.36, 0.41, 0.46),
            scale: 45.0,
        });
        metal.specular = 0.65;
        metal.reflectivity = 0.28;
        let crowd = Material::matte(Texture::Crowd);

        let mut cubes = Vec::new();
        // Terreno, pit lane y plataforma central.
        add_cube(
            &mut cubes,
            Vec3::new(0.0, -1.38, -8.0),
            Vec3::new(65.0, 0.28, 75.0),
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
            &concrete,
        );

        // Muros y tribunas laterales escalonadas.
        for side in [-1.0_f32, 1.0] {
            add_cube(
                &mut cubes,
                Vec3::new(side * 10.75, -0.78, -8.0),
                Vec3::new(0.12, 0.36, 18.6),
                &concrete,
            );
            for tier in 0..7 {
                let t = tier as f32;
                add_cube(
                    &mut cubes,
                    Vec3::new(side * (11.45 + t * 0.52), -0.89 + t * 0.215, -8.0),
                    Vec3::new(0.62, 0.21 + t * 0.215, 18.2),
                    &crowd,
                );
            }
        }
        // Tribuna de la curva norte.
        for tier in 0..7 {
            let t = tier as f32;
            add_cube(
                &mut cubes,
                Vec3::new(0.0, -0.89 + t * 0.215, -27.35 - t * 0.52),
                Vec3::new(10.9 + t * 0.55, 0.21 + t * 0.215, 0.62),
                &crowd,
            );
        }
        // Tribuna de la curva sur: completa el anillo del estadio.
        for tier in 0..7 {
            let t = tier as f32;
            add_cube(
                &mut cubes,
                Vec3::new(0.0, -0.89 + t * 0.215, 11.35 + t * 0.52),
                Vec3::new(10.9 + t * 0.55, 0.21 + t * 0.215, 0.62),
                &crowd,
            );
        }
        // Boxes y edificios de pits.
        for bay in 0..7 {
            let z = -16.0 + bay as f32 * 2.55;
            add_cube(
                &mut cubes,
                Vec3::new(3.10, -0.63, z),
                Vec3::new(1.18, 0.43, 0.92),
                if bay % 2 == 0 { &dark } else { &concrete },
            );
            add_cube(
                &mut cubes,
                Vec3::new(3.10, -0.12, z),
                Vec3::new(1.34, 0.09, 1.05),
                &concrete,
            );
        }
        // Trailers y motorhomes del paddock.
        for row in 0..4 {
            for column in 0..3 {
                let x = -3.8 + column as f32 * 1.55;
                let z = -15.2 + row as f32 * 2.15;
                add_cube(
                    &mut cubes,
                    Vec3::new(x, -0.87, z),
                    Vec3::new(0.58, 0.23, 0.78),
                    if (row + column) % 2 == 0 {
                        &concrete
                    } else {
                        &concrete
                    },
                );
                add_cube(
                    &mut cubes,
                    Vec3::new(x, -0.57, z),
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
            &concrete,
        );
        add_cube(
            &mut cubes,
            Vec3::new(-1.0, 3.42, -4.39),
            Vec3::new(1.18, 0.70, 0.035),
            &concrete,
        );

        // Meta en dos filas, pintura opaca al ras del asfalto.
        for row in 0..2 {
            for square in 0..12 {
                add_cube(
                    &mut cubes,
                    Vec3::new(
                        7.05 + square as f32 * 0.25,
                        -1.043,
                        -5.6 + row as f32 * 0.25,
                    ),
                    Vec3::new(0.125, 0.006, 0.125),
                    if (row + square) % 2 == 0 {
                        &white
                    } else {
                        &dark
                    },
                );
            }
        }
        // Valla de seguridad con postes y cables tensados.
        for side in [-1.0, 1.0] {
            for i in 0..25 {
                add_cube(
                    &mut cubes,
                    Vec3::new(side * 10.8, 0.2, -26.0 + i as f32 * 1.5),
                    Vec3::new(0.025, 1.30, 0.025),
                    &metal,
                );
            }
            for h in 0..5 {
                add_cube(
                    &mut cubes,
                    Vec3::new(side * 10.8, -0.30 + h as f32 * 0.35, -8.0),
                    Vec3::new(0.015, 0.015, 18.5),
                    &metal,
                );
            }
        }
        // Panel de meta suspendido sobre soportes.
        let yellow = Material::matte(Texture::Solid(Color::new(0.86, 0.60, 0.045)));
        // Delimitación del pit lane y marcas de goma en la frenada.
        for i in 0..12 {
            add_cube(
                &mut cubes,
                Vec3::new(6.5, -1.035, -13.0 + i as f32 * 0.8),
                Vec3::new(0.025, 0.004, 0.25),
                &yellow,
            );
        }
        for x in [7.1, 7.65, 8.2, 8.7, 9.3] {
            add_cube(
                &mut cubes,
                Vec3::new(x, -1.048, -7.5),
                Vec3::new(0.025, 0.002, 1.25),
                &dark,
            );
        }
        for x in [6.8, 10.45] {
            add_cube(
                &mut cubes,
                Vec3::new(x, 0.55, -1.8),
                Vec3::new(0.05, 1.65, 0.05),
                &metal,
            );
        }
        add_cube(
            &mut cubes,
            Vec3::new(8.6, 2.15, -1.8),
            Vec3::new(1.85, 0.28, 0.08),
            &dark,
        );
        crate::cars::label(
            &mut cubes,
            "PISTON CUP",
            Vec3::new(9.8, 2.32, -1.90),
            0.065,
            &white,
        );
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
                &concrete,
            );
        }

        let mut ellipsoids = Vec::new();
        crate::cars::build(
            &mut cubes,
            &mut ellipsoids,
            8.45,
            -4.35,
            0.64,
            0.0,
            &car,
            &glass,
            &dark,
            &white,
            &metal,
        );
        crate::cars::build(
            &mut cubes,
            &mut ellipsoids,
            9.65,
            -3.45,
            0.58,
            1.0,
            &car,
            &glass,
            &dark,
            &white,
            &metal,
        );
        crate::cars::build(
            &mut cubes,
            &mut ellipsoids,
            7.30,
            -4.05,
            0.59,
            2.0,
            &car,
            &glass,
            &dark,
            &white,
            &metal,
        );

        Self {
            accel: crate::accel::Bvh::build(&cubes, &ellipsoids),
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
                    material: white.clone(),
                },
                OvalRing {
                    center: Vec3::new(0.0, -1.024, -8.0),
                    outer_radii: (7.02, 15.02),
                    inner_radii: (6.76, 14.76),
                    material: white.clone(),
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
                    position: Vec3::new(-180.0, 240.0, -120.0),
                    color: Color::new(1.0, 0.83, 0.64),
                    intensity: 0.95,
                },
                Light {
                    position: Vec3::new(10.0, 11.0, -18.0),
                    color: Color::new(0.68, 0.82, 1.0),
                    intensity: 0.24,
                },
            ],
            skybox: Skybox {
                horizon: Color::new(0.58, 0.72, 0.84),
                zenith: Color::new(0.07, 0.20, 0.43),
            },
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Hit<'_>> {
        self.accel
            .hit(ray, &self.cubes, &self.ellipsoids, f32::INFINITY)
            .into_iter()
            .chain(self.oval_rings.iter().filter_map(|o| o.intersect(ray)))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }

    pub fn sky_color(&self, direction: Vec3) -> Color {
        let base = self
            .skybox
            .horizon
            .mix(self.skybox.zenith, direction.y.max(0.0).powf(0.5));
        let sun = direction
            .dot(Vec3::new(-180.0, 240.0, -120.0).normalize())
            .max(0.0);
        let cloud = ((direction.x * 22.0 + direction.z * 13.0).sin()
            + (direction.z * 31.0 - direction.x * 7.0).sin())
            * 0.25
            + 0.5;
        base.mix(
            Color::new(0.88, 0.85, 0.79),
            ((cloud - 0.58) * 2.0).clamp(0.0, 0.6) * direction.y.max(0.0),
        ) + Color::new(1.0, 0.80, 0.50) * (sun.powf(900.0) * 3.0 + sun.powf(24.0) * 0.18)
    }
}

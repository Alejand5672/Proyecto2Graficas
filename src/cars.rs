//! Detalles analíticos sobre los volúmenes existentes; sin modelos importados.
use crate::{cube::Cube, ellipsoid::Ellipsoid, material::Material, vec3::Vec3};

fn block(c: &mut Vec<Cube>, p: Vec3, h: Vec3, m: &Material) {
    c.push(Cube {
        center: p,
        half_size: h,
        material: m.clone(),
    });
}
pub fn label(c: &mut Vec<Cube>, text: &str, p: Vec3, size: f32, m: &Material) {
    for (i, ch) in text.chars().enumerate() {
        let rows: [u8; 5] = match ch {
            '9' => [7, 5, 7, 1, 7],
            '5' => [7, 4, 7, 1, 7],
            '4' => [5, 5, 7, 1, 1],
            '3' => [7, 1, 7, 1, 7],
            '8' => [7, 5, 7, 5, 7],
            '6' => [7, 4, 7, 5, 7],
            'P' => [7, 5, 7, 4, 4],
            'I' => [7, 2, 2, 2, 7],
            'S' => [7, 4, 7, 1, 7],
            'T' => [7, 2, 2, 2, 2],
            'O' => [7, 5, 5, 5, 7],
            'N' => [5, 7, 7, 7, 5],
            'C' => [7, 4, 4, 4, 7],
            'U' => [5, 5, 5, 5, 7],
            _ => [0; 5],
        };
        for (r, bits) in rows.iter().enumerate() {
            for col in 0..3 {
                if bits & (1 << (2 - col)) != 0 {
                    block(
                        c,
                        p + Vec3::new(
                            -(i as f32 * 4.0 + col as f32) * size,
                            -(r as f32) * size,
                            0.0,
                        ),
                        Vec3::new(size * 0.48, size * 0.48, 0.003),
                        m,
                    );
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn build(
    c: &mut Vec<Cube>,
    e: &mut Vec<Ellipsoid>,
    x: f32,
    z: f32,
    s: f32,
    slot: f32,
    paint: &Material,
    glass: &Material,
    rubber: &Material,
    white: &Material,
    metal: &Material,
) {
    let pos = |dx: f32, y: f32, dz: f32| Vec3::new(x + dx * s, -1.055 + y * s, z + dz * s);
    let mut coating = paint.clone();
    coating.reflectivity = [0.19, 0.24, 0.12][slot as usize];
    for (dx, y, dz, rx, ry, rz, m, uv) in [
        (0.0, 0.41, 0.0, 0.86, 0.32, 1.23, &coating, slot),
        (0.0, 0.62, -0.60, 0.76, 0.23, 0.63, &coating, slot),
        (0.0, 0.79, 0.29, 0.60, 0.24, 0.57, glass, 0.0),
        (-0.78, 0.34, -0.56, 0.19, 0.34, 0.34, rubber, 0.0),
        (0.78, 0.34, -0.56, 0.19, 0.34, 0.34, rubber, 0.0),
        (-0.78, 0.34, 0.59, 0.19, 0.34, 0.34, rubber, 0.0),
        (0.78, 0.34, 0.59, 0.19, 0.34, 0.34, rubber, 0.0),
    ] {
        e.push(Ellipsoid {
            center: pos(dx, y, dz),
            radii: Vec3::new(rx * s, ry * s, rz * s),
            uv_offset: uv,
            material: m.clone(),
        });
    }
    // Ojos sobre el frente del parabrisas, iris y pupilas opacas.
    for dx in [-0.24, 0.24] {
        for (y, dz, rx, ry, rz, m) in [
            (0.86, -0.20, 0.24, 0.10, 0.05, white),
            (0.86, -0.247, 0.07, 0.08, 0.017, metal),
            (0.86, -0.263, 0.036, 0.06, 0.012, rubber),
        ] {
            e.push(Ellipsoid {
                center: pos(dx, y, dz),
                radii: Vec3::new(rx * s, ry * s, rz * s),
                uv_offset: 0.0,
                material: m.clone(),
            });
        }
    }
    // Sonrisa, faros y rines.
    block(
        c,
        pos(0.0, 0.39, -1.23),
        Vec3::new(0.33 * s, 0.045 * s, 0.025 * s),
        rubber,
    );
    for dx in [-0.52, 0.52] {
        block(
            c,
            pos(dx, 0.53, -1.08),
            Vec3::new(0.14 * s, 0.06 * s, 0.045 * s),
            white,
        );
    }
    for dx in [-0.97, 0.97] {
        for dz in [-0.56, 0.59] {
            e.push(Ellipsoid {
                center: pos(dx, 0.34, dz),
                radii: Vec3::new(0.012 * s, 0.19 * s, 0.19 * s),
                uv_offset: 0.0,
                material: metal.clone(),
            });
        }
    }
    let wing_y = if slot == 1.0 { 1.18 } else { 0.78 };
    for dx in [-0.54, 0.54] {
        block(
            c,
            pos(dx, (0.55 + wing_y) * 0.5, 0.99),
            Vec3::new(0.04 * s, (wing_y - 0.55) * 0.5 * s, 0.04 * s),
            metal,
        );
    }
    let mut wing = coating.clone();
    wing.texture = crate::texture::Texture::Solid(match slot as usize {
        1 => crate::color::Color::new(0.04, 0.48, 0.82),
        2 => crate::color::Color::new(0.18, 0.68, 0.10),
        _ => crate::color::Color::new(0.92, 0.018, 0.028),
    });
    block(
        c,
        pos(0.0, wing_y, 0.99),
        Vec3::new(0.80 * s, 0.045 * s, 0.16 * s),
        &wing,
    );
    label(
        c,
        ["95", "43", "86"][slot as usize],
        pos(0.14, 0.66, -1.19),
        0.043 * s,
        white,
    );
    // Techo de pintura y números horizontales, visibles en la vista de referencia.
    block(
        c,
        pos(0.0, 1.0, 0.38),
        Vec3::new(0.40 * s, 0.035 * s, 0.32 * s),
        &wing,
    );
    let mut digits = Vec::new();
    label(
        &mut digits,
        ["95", "43", "86"][slot as usize],
        Vec3::new(0.24, 0.0, 0.0),
        0.08,
        white,
    );
    for d in digits {
        block(
            c,
            pos(d.center.x, 1.041, 0.54 + d.center.y),
            Vec3::new(d.half_size.x * s, 0.004 * s, d.half_size.y * s),
            white,
        );
    }
    // Décoration de capot spécifique, sans modèle ni image externe.
    let mut badge = white.clone();
    badge.texture = crate::texture::Texture::Solid(if slot == 0.0 {
        crate::color::Color::new(0.95, 0.62, 0.06)
    } else {
        crate::color::Color::new(0.87, 0.91, 0.90)
    });
    block(
        c,
        pos(0.0, 0.85, -0.62),
        Vec3::new(0.25 * s, 0.018 * s, 0.16 * s),
        &badge,
    );
    // La langue de McQueen rappelle le franchissement de ligne de la référence.
    if slot == 0.0 {
        let mut tongue = rubber.clone();
        tongue.texture = crate::texture::Texture::Solid(crate::color::Color::new(0.48, 0.11, 0.15));
        // Elipsoide aplanado: raíz dentro de la boca y punta continua redondeada.
        e.push(Ellipsoid {
            center: pos(0.0, 0.37, -1.40),
            radii: Vec3::new(0.16 * s, 0.025 * s, 0.28 * s),
            uv_offset: 0.0,
            material: tongue,
        });
    }
}

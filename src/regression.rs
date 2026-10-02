use crate::{
    color::Color, cube::Cube, material::Material, ray::Ray, scene::Scene, texture::Texture,
    vec3::Vec3,
};
#[test]
fn scaled_floor_normal_is_up() {
    let cube = Cube {
        center: Vec3::new(0.0, 0.0, 0.0),
        half_size: Vec3::new(18.0, 0.28, 26.0),
        material: Material::matte(Texture::Solid(Color::BLACK)),
    };
    let hit = cube
        .intersect(Ray {
            origin: Vec3::new(10.0, 3.0, 8.0),
            direction: Vec3::new(0.0, -1.0, 0.0),
        })
        .unwrap();
    assert_eq!(hit.normal, Vec3::new(0.0, 1.0, 0.0));
}
#[test]
fn acceleration_matches_linear_intersections() {
    let s = Scene::base();
    for x in -12..13 {
        for z in -28..14 {
            let ray = Ray {
                origin: Vec3::new(x as f32 + 0.13, 12.0, z as f32 + 0.17),
                direction: Vec3::new(0.03, -1.0, 0.02).normalize(),
            };
            let reference = s
                .cubes
                .iter()
                .filter_map(|c| c.intersect(ray))
                .chain(s.ellipsoids.iter().filter_map(|e| e.intersect(ray)))
                .chain(s.oval_rings.iter().filter_map(|o| o.intersect(ray)))
                .min_by(|a, b| a.distance.total_cmp(&b.distance));
            let actual = s.intersect(ray);
            assert_eq!(reference.is_some(), actual.is_some());
            if let (Some(a), Some(b)) = (reference, actual) {
                assert!((a.distance - b.distance).abs() < 0.0001);
            }
        }
    }
}
#[test]
fn tires_touch_asphalt_for_all_scales() {
    let s = Scene::base();
    // McQueen añade un elipsoide para la lengua después de sus 17 piezas.
    for start in [0, 18, 35] {
        for wheel in 3..7 {
            let e = &s.ellipsoids[start + wheel];
            assert!((e.center.y - e.radii.y + 1.055).abs() < 0.0001);
        }
    }
}
#[test]
fn glass_total_internal_reflection() {
    assert!(
        Vec3::new(0.9, -0.43589, 0.0)
            .normalize()
            .refract(Vec3::new(0.0, 1.0, 0.0), 1.5)
            .is_none()
    );
}

#[test]
fn suite_facade_is_reflective_refractive_glass() {
    let scene = Scene::base();
    let hit = scene
        .intersect(Ray {
            origin: Vec3::new(6.0, 1.10, -5.0),
            direction: Vec3::new(-1.0, 0.0, 0.0),
        })
        .unwrap();
    assert!(hit.material.reflectivity >= 0.4);
    assert!(hit.material.transparency > 0.0);
    assert_eq!(hit.material.ior, 1.5);
}

use crate::{
    camera::Camera,
    color::Image,
    renderer::{RenderConfig, render},
    scene::Scene,
    vec3::Vec3,
};
use raylib::prelude::*;

/// Visor interactivo. El raytracer se ejecuta solo cuando cambia la cámara.
pub fn show(scene: Scene, config: RenderConfig) {
    let mut yaw = 0.0_f32;
    let mut pitch = 0.13_f32;
    let mut distance = 9.4_f32;
    let target = Vec3::new(0.0, -0.15, -5.1);
    let mut image: Image = render(
        &scene,
        &Camera::orbit(target, distance, yaw, pitch, 55.0),
        config,
    );
    let (mut window, thread) = raylib::init()
        .size(image.width as i32, image.height as i32)
        .title("Proyecto 2 — Diorama Cars (raytracing)")
        .build();
    window.set_target_fps(60);

    while !window.window_should_close() {
        let mut changed = false;
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            yaw -= 0.045;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            yaw += 0.045;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            pitch += 0.025;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            pitch -= 0.025;
            changed = true;
        }
        let wheel = window.get_mouse_wheel_move();
        if wheel != 0.0 {
            distance = (distance - wheel * 0.55).clamp(4.0, 16.0);
            changed = true;
        }
        if changed {
            image = render(
                &scene,
                &Camera::orbit(target, distance, yaw, pitch, 55.0),
                config,
            );
        }
        let mut draw = window.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        for y in 0..image.height {
            for x in 0..image.width {
                let pixel = image.pixels[y * image.width + x];
                draw.draw_pixel(
                    x as i32,
                    y as i32,
                    Color::new(
                        (pixel.r.clamp(0.0, 1.0).sqrt() * 255.0) as u8,
                        (pixel.g.clamp(0.0, 1.0).sqrt() * 255.0) as u8,
                        (pixel.b.clamp(0.0, 1.0).sqrt() * 255.0) as u8,
                        255,
                    ),
                );
            }
        }
        draw.draw_text(
            "Flechas: orbitar | Rueda: zoom | Raytracing: reflejo y refraccion",
            12,
            12,
            20,
            Color::RAYWHITE,
        );
    }
}

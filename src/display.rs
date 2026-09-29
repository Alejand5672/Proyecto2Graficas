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
    let mut pitch = 1.03_f32;
    let mut distance = 42.0_f32;
    let mut target = Vec3::new(0.0, -0.55, -8.0);
    let mut image: Image = render(
        &scene,
        &Camera::orbit(target, distance, yaw, pitch, 55.0),
        config,
    );
    let (mut window, thread) = raylib::init()
        .size(image.width as i32, image.height as i32)
        .title("Motor Speedway — Diorama con raytracing")
        .resizable()
        .build();
    window.set_target_fps(60);

    while !window.window_should_close() {
        let mut changed = false;
        if window.is_key_pressed(KeyboardKey::KEY_F11) {
            window.toggle_fullscreen();
        }
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
        let pan_speed = if window.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            0.48
        } else {
            0.22
        };
        let forward = Vec3::new(yaw.sin(), 0.0, yaw.cos());
        let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin());
        if window.is_key_down(KeyboardKey::KEY_W) {
            target = target - forward * pan_speed;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            target = target + forward * pan_speed;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            target = target - right * pan_speed;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            target = target + right * pan_speed;
            changed = true;
        }
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            yaw = 0.0;
            pitch = 1.03;
            distance = 42.0;
            target = Vec3::new(0.0, -0.55, -8.0);
            changed = true;
        }
        if window.is_key_pressed(KeyboardKey::KEY_M) {
            yaw = 0.16;
            pitch = 0.82;
            distance = 8.6;
            target = Vec3::new(8.50, -0.55, -3.65);
            changed = true;
        }
        let wheel = window.get_mouse_wheel_move();
        if wheel != 0.0 {
            distance = (distance - wheel * 1.55).clamp(5.5, 58.0);
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
            "Flechas: orbitar | WASD: mover | Rueda: zoom | M: meta/auto | R: estadio | F11",
            12,
            12,
            20,
            Color::RAYWHITE,
        );
    }
}

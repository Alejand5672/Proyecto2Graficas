use crate::{
    renderer::{RenderConfig, render},
    scene::Scene,
    vec3::Vec3,
    views::View,
};
use raylib::prelude::*;
use std::{
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};

/// CPU tracing runs on a worker; the window only uploads completed frames.
pub fn show(scene: Scene, config: RenderConfig) {
    let smoke = std::env::args().any(|a| a == "--smoke");
    let scene = Arc::new(scene);
    let (mut window, thread) = raylib::init()
        .size(1280, 800)
        .title("PISTON CUP | Final")
        .resizable()
        .build();
    window.set_target_fps(60);
    let (tx, rx) = mpsc::channel();
    let mut texture = window
        .load_texture_from_image(&thread, &Image::gen_image_color(1, 1, Color::BLACK))
        .expect("textura");
    let mut view = View::preset(0);
    let mut revision = 0u64;
    let mut pending = true;
    let mut busy = false;
    let mut refined = false;
    let mut last_move = Instant::now();
    let mut frame_ms = 0u128;
    let mut touch_last: Option<Vector2> = None;
    let mut pinch_last: Option<f32> = None;
    while !window.window_should_close() {
        let dt = window.get_frame_time().min(0.05);
        let mut changed = false;
        let width = window.get_screen_width().max(1);
        let height = window.get_screen_height().max(1);
        if window.is_key_pressed(KeyboardKey::KEY_F11) {
            window.toggle_fullscreen();
        }
        for (key, n) in [
            (KeyboardKey::KEY_ONE, 0),
            (KeyboardKey::KEY_TWO, 1),
            (KeyboardKey::KEY_THREE, 2),
            (KeyboardKey::KEY_FOUR, 3),
            (KeyboardKey::KEY_M, 0),
            (KeyboardKey::KEY_R, 3),
        ] {
            if window.is_key_pressed(key) {
                view = View::preset(n);
                changed = true;
            }
        }
        let mouse = window.get_mouse_position();
        if window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && mouse.y < 42.0 {
            let n = ((mouse.x - 14.0) / 135.0) as i32;
            if (0..4).contains(&n) {
                view = View::preset(n as usize);
                changed = true;
            }
        }
        let delta = window.get_mouse_delta();
        let touches = window.get_touch_point_count();
        if touches == 1 {
            let p = window.get_touch_position(0);
            if let Some(last) = touch_last {
                if p.y > 46.0 {
                    view.yaw -= (p.x - last.x) * 0.006;
                    view.pitch += (p.y - last.y) * 0.005;
                    changed |= p.x != last.x || p.y != last.y;
                }
            }
            touch_last = Some(p);
        } else {
            touch_last = None;
        }
        if touches >= 2 {
            let a = window.get_touch_position(0);
            let b = window.get_touch_position(1);
            let distance = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();
            if let Some(last) = pinch_last {
                if distance > 1.0 {
                    view.distance *= last / distance;
                    changed = true;
                }
            }
            pinch_last = Some(distance);
        } else {
            pinch_last = None;
        }
        if touches == 0
            && window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
            && mouse.y > 46.0
            && (delta.x != 0.0 || delta.y != 0.0)
        {
            view.yaw -= delta.x * 0.006;
            view.pitch += delta.y * 0.005;
            changed = true;
        }
        let forward = Vec3::new(view.yaw.sin(), 0.0, view.yaw.cos());
        let right = Vec3::new(view.yaw.cos(), 0.0, -view.yaw.sin());
        if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT)
            && (delta.x != 0.0 || delta.y != 0.0)
        {
            view.target = view.target - right * (delta.x * view.distance * 0.001)
                + forward * (delta.y * view.distance * 0.001);
            changed = true;
        }
        for (key, a, b) in [
            (KeyboardKey::KEY_LEFT, -1.0, 0.0),
            (KeyboardKey::KEY_RIGHT, 1.0, 0.0),
            (KeyboardKey::KEY_UP, 0.0, 1.0),
            (KeyboardKey::KEY_DOWN, 0.0, -1.0),
        ] {
            if window.is_key_down(key) {
                view.yaw += a * dt;
                view.pitch += b * dt;
                changed = true;
            }
        }
        let speed = dt
            * view.distance
            * if window.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                0.8
            } else {
                0.3
            };
        for (key, offset) in [
            (KeyboardKey::KEY_W, -forward),
            (KeyboardKey::KEY_S, forward),
            (KeyboardKey::KEY_A, -right),
            (KeyboardKey::KEY_D, right),
        ] {
            if window.is_key_down(key) {
                view.target = view.target + offset * speed;
                changed = true;
            }
        }
        let wheel = window.get_mouse_wheel_move();
        if wheel != 0.0 {
            view.distance *= (-wheel * 0.1).exp();
            changed = true;
        }
        if window.is_window_resized() {
            changed = true;
        }
        view.constrain();
        view.avoid_solids(&scene);
        if changed {
            revision += 1;
            pending = true;
            refined = false;
            last_move = Instant::now();
        }
        if let Ok((rev, hi, img, ms)) = rx.try_recv() {
            busy = false;
            let img: crate::color::Image = img;
            if rev == revision {
                if texture.width() != img.width as i32 || texture.height() != img.height as i32 {
                    texture = window
                        .load_texture_from_image(
                            &thread,
                            &Image::gen_image_color(
                                img.width as i32,
                                img.height as i32,
                                Color::BLACK,
                            ),
                        )
                        .expect("textura");
                }
                texture
                    .update_texture(&img.rgba())
                    .expect("actualizar imagen");
                refined = hi;
                frame_ms = ms;
            }
        }
        if !busy && (pending || (!refined && last_move.elapsed() > Duration::from_millis(250))) {
            let hi = last_move.elapsed() > Duration::from_millis(250);
            let w = if hi { config.width } else { 320 };
            let h = ((w as f32 * height as f32 / width as f32) as usize).max(1);
            let c = RenderConfig {
                width: w,
                height: h,
                max_bounces: if hi { config.max_bounces } else { 2 },
            };
            let camera = view.camera();
            let scene = Arc::clone(&scene);
            let tx = tx.clone();
            let rev = revision;
            std::thread::spawn(move || {
                let start = Instant::now();
                let img = render(&scene, &camera, c);
                let _ = tx.send((rev, hi, img, start.elapsed().as_millis()));
            });
            busy = true;
            pending = false;
        }
        let mut draw = window.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        draw.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
            Rectangle::new(0.0, 0.0, width as f32, height as f32),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
        draw.draw_rectangle(0, 0, width, 44, Color::new(10, 17, 26, 235));
        for (n, name) in ["1  FINAL", "2  FRONTAL", "3  LATERAL", "4  ESTADIO"]
            .iter()
            .enumerate()
        {
            draw.draw_text(name, 20 + n as i32 * 135, 15, 16, Color::RAYWHITE);
        }
        draw.draw_rectangle(0, height - 34, width, 34, Color::new(10, 17, 26, 230));
        draw.draw_text(
            "Arrastrar: orbitar | Derecho/WASD: mover | Rueda: zoom | R: estadio | F11",
            14,
            height - 24,
            16,
            Color::RAYWHITE,
        );
        if busy {
            draw.draw_text("RENDERIZANDO...", width - 190, 15, 16, Color::GOLD);
        } else {
            draw.draw_text(
                &format!("{} ms", frame_ms),
                width - 130,
                15,
                16,
                Color::LIGHTGRAY,
            );
        }
        if smoke && refined {
            break;
        }
    }
}

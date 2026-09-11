use crate::color::Image;
use raylib::prelude::*;

/// Muestra una imagen ya calculada. No vuelve a ejecutar el raytracer dentro
/// del ciclo de la ventana: la escena permanece estática hasta cerrarla.
pub fn show(image: Image) {
    let (mut window, thread) = raylib::init()
        .size(image.width as i32, image.height as i32)
        .title("Proyecto 2 — Boceto estático de Cars")
        .build();
    window.set_target_fps(60);

    while !window.window_should_close() {
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
            "primer boceto",
            12,
            12,
            20,
            Color::RAYWHITE,
        );
    }
}

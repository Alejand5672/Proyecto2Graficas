mod camera;
mod color;
mod cube;
mod display;
mod material;
mod ray;
mod renderer;
mod scene;
mod texture;
mod vec3;

use camera::Camera;
use renderer::{RenderConfig, render};
use scene::Scene;
use vec3::Vec3;

fn main() {
    // Primer boceto: entorno simple y silueta estática; el salto y movimiento
    // se añadirán cuando se defina la animación.
    let scene = Scene::base();
    let camera = Camera::look_at(
        Vec3::new(0.0, 1.6, 4.6),
        Vec3::new(0.0, 0.0, -4.8),
        Vec3::new(0.0, 1.0, 0.0),
        55.0,
    );
    let image = render(
        &scene,
        &camera,
        RenderConfig {
            width: 640,
            height: 420,
            max_bounces: 3,
        },
    );
    display::show(image);
}

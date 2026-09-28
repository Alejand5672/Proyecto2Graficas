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

use renderer::RenderConfig;
use scene::Scene;

fn main() {
    let scene = Scene::base();
    display::show(
        scene,
        RenderConfig {
            width: 640,
            height: 420,
            max_bounces: 3,
        },
    );
}

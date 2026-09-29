mod camera;
mod color;
mod cube;
mod display;
mod ellipsoid;
mod material;
mod oval;
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
            width: 960,
            height: 600,
            max_bounces: 2,
        },
    );
}

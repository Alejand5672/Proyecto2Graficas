mod accel;
mod camera;
mod cars;
mod color;
mod cube;
mod display;
mod ellipsoid;
mod material;
mod oval;
mod ray;
#[cfg(test)]
mod regression;
mod renderer;
mod scene;
mod texture;
mod vec3;
mod views;

use renderer::RenderConfig;
use scene::Scene;

fn main() {
    let scene = Scene::base();
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some_and(|s| s == "--render") {
        let view = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let path = args.get(3).map(String::as_str).unwrap_or("preview.bmp");
        let start = std::time::Instant::now();
        let image = renderer::render(
            &scene,
            &views::View::preset(view).camera(),
            RenderConfig {
                width: 960,
                height: 600,
                max_bounces: 4,
            },
        );
        image.save_bmp(path).expect("guardar imagen");
        println!("Render: {:?} -> {}", start.elapsed(), path);
        return;
    }
    display::show(
        scene,
        RenderConfig {
            width: 960,
            height: 600,
            max_bounces: 4,
        },
    );
}

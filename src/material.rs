use crate::{color::Color, texture::Texture};
#[derive(Clone)]
pub struct Material {
    pub texture: Texture,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub ior: f32,
}
impl Material {
    pub fn matte(texture: Texture) -> Self {
        Self {
            texture,
            ambient: 0.10,
            diffuse: 0.85,
            specular: 0.20,
            shininess: 32.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 1.0,
        }
    }
    pub fn glass(texture: Texture) -> Self {
        Self {
            texture,
            ambient: 0.02,
            diffuse: 0.10,
            specular: 0.60,
            shininess: 96.0,
            reflectivity: 0.15,
            transparency: 0.85,
            ior: 1.5,
        }
    }
    pub fn albedo(&self, u: f32, v: f32) -> Color {
        self.texture.sample(u, v)
    }
}

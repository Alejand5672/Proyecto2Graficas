use crate::color::Color;
#[derive(Clone)]
pub enum Texture {
    Solid(Color),
    Checker {
        first: Color,
        second: Color,
        scale: f32,
    },
}
impl Texture {
    pub fn sample(&self, u: f32, v: f32) -> Color {
        match *self {
            Self::Solid(c) => c,
            Self::Checker {
                first,
                second,
                scale,
            } => {
                if ((u * scale).floor() as i32 + (v * scale).floor() as i32).rem_euclid(2) == 0 {
                    first
                } else {
                    second
                }
            }
        }
    }
}

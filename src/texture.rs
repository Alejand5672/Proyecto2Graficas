use crate::color::Color;
#[derive(Clone)]
pub enum Texture {
    Solid(Color),
    Checker {
        first: Color,
        second: Color,
        scale: f32,
    },
    Stripes {
        first: Color,
        second: Color,
        scale: f32,
    },
    /// Una sola textura de pintura contiene las tres libreas. El entero de U
    /// selecciona rojo, azul o verde; la fracción conserva el patrón de pintura.
    RacePaint,
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
            Self::Stripes {
                first,
                second,
                scale,
            } => {
                if ((u * scale).floor() as i32).rem_euclid(2) == 0 {
                    first
                } else {
                    second
                }
            }
            Self::RacePaint => {
                let palette = [
                    (
                        Color::new(0.92, 0.018, 0.028),
                        Color::new(0.55, 0.006, 0.012),
                    ),
                    (Color::new(0.04, 0.48, 0.82), Color::new(0.015, 0.20, 0.48)),
                    (Color::new(0.18, 0.68, 0.10), Color::new(0.045, 0.31, 0.025)),
                ];
                let index = (u.floor() as i32).rem_euclid(3) as usize;
                let local_u = u.rem_euclid(1.0);
                let (bright, dark) = palette[index];
                if ((local_u * 8.0).floor() as i32 + (v * 3.0).floor() as i32).rem_euclid(5) == 0 {
                    dark
                } else {
                    bright
                }
            }
        }
    }
}

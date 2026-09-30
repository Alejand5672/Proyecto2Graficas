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
    Asphalt,
    Crowd,
}
impl Texture {
    pub fn sample(&self, u: f32, v: f32) -> Color {
        match *self {
            Self::Asphalt => {
                let noise = ((u * 831.7 + v * 193.1).sin() * 43758.5).fract().abs();
                let wear = (u * 18.0 + v * 0.3).sin().abs().powf(28.0);
                Color::new(0.085, 0.093, 0.105) * (0.85 + noise * 0.25 - wear * 0.12)
            }
            Self::Crowd => {
                let x = (u * 110.0).floor();
                let y = (v * 130.0).floor();
                let n = ((x * 73.3 + y * 127.1).sin() * 43758.5).fract().abs();
                let colors = [
                    Color::new(0.08, 0.18, 0.34),
                    Color::new(0.64, 0.14, 0.08),
                    Color::new(0.75, 0.72, 0.57),
                    Color::new(0.12, 0.39, 0.32),
                    Color::new(0.12, 0.13, 0.18),
                ];
                colors[(n * 4.99) as usize]
                    * if (u * 110.0).fract() < 0.15 || (v * 130.0).fract() < 0.2 {
                        0.3
                    } else {
                        1.0
                    }
            }
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
                if v > 0.38 && v < 0.42 && local_u > 0.35 {
                    dark
                } else {
                    bright
                }
            }
        }
    }
}

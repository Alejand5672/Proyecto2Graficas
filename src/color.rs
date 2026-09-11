#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    pub fn mix(self, o: Self, t: f32) -> Self {
        self * (1.0 - t) + o * t
    }
}
impl std::ops::Add for Color {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.r + r.r, self.g + r.g, self.b + r.b)
    }
}
impl std::ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, r: f32) -> Self {
        Self::new(self.r * r, self.g * r, self.b * r)
    }
}
impl std::ops::Mul for Color {
    type Output = Self;
    fn mul(self, r: Self) -> Self {
        Self::new(self.r * r.r, self.g * r.g, self.b * r.b)
    }
}
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}
impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            pixels: vec![Color::BLACK; w * h],
        }
    }
    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y * self.width + x] = c
    }
}

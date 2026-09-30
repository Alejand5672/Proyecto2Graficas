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
    pub fn rgba(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|p| {
                let map =
                    |x: f32| ((x.max(0.0) / (1.0 + x.max(0.0))).powf(1.0 / 2.2) * 255.0) as u8;
                [map(p.r), map(p.g), map(p.b), 255]
            })
            .collect()
    }
    pub fn save_bmp(&self, path: &str) -> std::io::Result<()> {
        use std::io::Write;
        let mut f = std::fs::File::create(path)?;
        let size = (54 + self.width * self.height * 4) as u32;
        let mut h = vec![0u8; 54];
        h[0..2].copy_from_slice(b"BM");
        h[2..6].copy_from_slice(&size.to_le_bytes());
        h[10..14].copy_from_slice(&54u32.to_le_bytes());
        h[14..18].copy_from_slice(&40u32.to_le_bytes());
        h[18..22].copy_from_slice(&(self.width as i32).to_le_bytes());
        h[22..26].copy_from_slice(&(-(self.height as i32)).to_le_bytes());
        h[26..28].copy_from_slice(&1u16.to_le_bytes());
        h[28..30].copy_from_slice(&32u16.to_le_bytes());
        f.write_all(&h)?;
        let mut bytes = self.rgba();
        for p in bytes.chunks_exact_mut(4) {
            p.swap(0, 2);
        }
        f.write_all(&bytes)
    }
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
            pixels: vec![Color::BLACK; w * h],
        }
    }
}

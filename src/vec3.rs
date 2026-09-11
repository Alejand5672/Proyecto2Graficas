use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalize(self) -> Self {
        let l = self.length();
        if l > 0.0 { self / l } else { self }
    }
    pub fn reflect(self, n: Self) -> Self {
        self - n * (2.0 * self.dot(n))
    }
    pub fn refract(self, n: Self, eta: f32) -> Option<Self> {
        let c = (-self).dot(n).min(1.0);
        let s = eta * eta * (1.0 - c * c);
        (s <= 1.0).then(|| self * eta + n * (eta * c - (1.0 - s).sqrt()))
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, r: Self) -> Self {
        Self::new(self.x + r.x, self.y + r.y, self.z + r.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Self::new(self.x - r.x, self.y - r.y, self.z - r.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, r: f32) -> Self {
        Self::new(self.x * r, self.y * r, self.z * r)
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, r: f32) -> Self {
        Self::new(self.x / r, self.y / r, self.z / r)
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

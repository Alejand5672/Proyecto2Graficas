use crate::{camera::Camera, vec3::Vec3};
#[derive(Clone, Copy)]
pub struct View {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target: Vec3,
}
impl View {
    pub fn preset(n: usize) -> Self {
        match n {
            1 => Self {
                yaw: 3.14,
                pitch: 0.20,
                distance: 7.8,
                target: Vec3::new(8.45, -0.55, -4.1),
            },
            2 => Self {
                yaw: -1.57,
                pitch: 0.42,
                distance: 9.0,
                target: Vec3::new(8.45, -0.5, -4.0),
            },
            3 => Self {
                yaw: 0.0,
                pitch: 1.25,
                distance: 54.0,
                target: Vec3::new(0.0, -0.5, -8.0),
            },
            _ => Self {
                yaw: 3.65,
                pitch: 0.31,
                distance: 6.6,
                target: Vec3::new(8.45, -0.45, -4.0),
            },
        }
    }
    pub fn camera(&self) -> Camera {
        Camera::orbit(self.target, self.distance, self.yaw, self.pitch, 50.0)
    }
    pub fn constrain(&mut self) {
        self.pitch = self.pitch.clamp(0.15, 1.25);
        self.distance = self.distance.clamp(4.5, 65.0);
        self.target.x = self.target.x.clamp(-14.0, 14.0);
        self.target.z = self.target.z.clamp(-28.0, 13.0);
    }
    pub fn avoid_solids(&mut self, scene: &crate::scene::Scene) {
        // Elevar el observador si termina dentro de una estructura del estadio.
        for _ in 0..24 {
            let eye = self.target
                + Vec3::new(
                    self.distance * self.yaw.sin() * self.pitch.cos(),
                    self.distance * self.pitch.sin(),
                    self.distance * self.yaw.cos() * self.pitch.cos(),
                );
            let inside = scene.cubes.iter().any(|c| {
                let d = eye - c.center;
                d.x.abs() < c.half_size.x + 0.15
                    && d.y.abs() < c.half_size.y + 0.15
                    && d.z.abs() < c.half_size.z + 0.15
            });
            if !inside {
                break;
            }
            self.pitch = (self.pitch + 0.04).min(1.25);
            self.distance += 0.2;
        }
    }
}

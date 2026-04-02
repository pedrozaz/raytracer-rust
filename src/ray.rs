use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    // P(t) = A + t * B
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

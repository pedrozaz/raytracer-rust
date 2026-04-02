use rand::RngExt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(self) -> f64 {
        self.squared_lenght().sqrt()
    }

    pub fn squared_lenght(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn dot(v1: Self, v2: Self) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: Self, v2: Self) -> Self {
        Self::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        )
    }

    pub fn random_in_unit_sphere(rng: &mut rand::rngs::ThreadRng) -> Self {
        loop {
            let p = Self::new(
                rng.random::<f64>() * 2.0 - 1.0,
                rng.random::<f64>() * 2.0 - 1.0,
                rng.random::<f64>() * 2.0 - 1.0,
            );
            if p.squared_lenght() < 1.0 {
                return p;
            }
        }
    }

    pub fn reflect(v: Self, n: Self) -> Self {
        v - (n * (2.0 * Self::dot(v, n)))
    }

    pub fn refract(v: Self, n: Self, ni_over_nt: f64) -> Option<Self> {
        let uv = v.normalize();
        let dt = Self::dot(uv, n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

        if discriminant > 0.0 {
            Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
        } else {
            None
        }
    }

    pub fn random_in_unit_disk(rng: &mut rand::rngs::ThreadRng) -> Self {
        loop {
            let p = Self::new(
                rng.random::<f64>() * 2.0 - 1.0,
                rng.random::<f64>() * 2.0 - 1.0,
                0.0,
            );
            if p.squared_lenght() < 1.0 {
                return p;
            }
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self::Output {
        Self::new(self.x / t, self.y / t, self.z / t)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self::Output {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

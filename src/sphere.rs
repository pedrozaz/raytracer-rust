use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let oc = ray.origin - self.center;

        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let mut temp = (-b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.as_ref(),
                });
            }

            temp = (-b + root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.as_ref(),
                });
            }
        }
        None
    }
}

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'_>> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in &self.list {
            if let Some(record) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                hit_anything = Some(record);
            }
        }

        hit_anything
    }
}

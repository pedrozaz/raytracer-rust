use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_ny = 2.0 * h;
        let viewport_nx = aspect_ratio * viewport_ny;

        let w = (lookfrom - lookat).normalize();
        let u = Vec3::cross(vup, w).normalize();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = u * viewport_nx * focus_dist;
        let vertical = v * viewport_ny * focus_dist;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_dist;
        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut rand::rngs::ThreadRng) -> Ray {
        let rd = Vec3::random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x * self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
                - self.origin
                - offset,
        )
    }
}

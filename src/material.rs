use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::RngExt;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere(rng);
        let scattered = Ray::new(rec.p, target - rec.p);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + (Vec3::random_in_unit_sphere(rng) * self.fuzz),
        );

        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(r_in.direction, rec.normal) > 0.0 {
            (
                Vec3::new(-rec.normal.x, -rec.normal.y, -rec.normal.z),
                self.ref_idx,
                self.ref_idx * Vec3::dot(r_in.direction, rec.normal) / r_in.direction.length(),
            )
        } else {
            (
                rec.normal,
                1.0 / self.ref_idx,
                -Vec3::dot(r_in.direction, rec.normal) / r_in.direction.length(),
            )
        };

        let refracted = Vec3::refract(r_in.direction, outward_normal, ni_over_nt);

        let reflect_prob = if refracted.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        if rng.random::<f64>() < reflect_prob {
            let reflected = Vec3::reflect(r_in.direction.normalize(), rec.normal);
            Some((attenuation, Ray::new(rec.p, reflected)))
        } else {
            Some((attenuation, Ray::new(rec.p, refracted.unwrap())))
        }
    }
}

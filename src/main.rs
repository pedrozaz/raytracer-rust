mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use core::f64;
use hittable::Hittable;
use hittable_list::HittableList;
use rand::RngExt;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

use crate::material::{Lambertian, Metal};

fn calculate_color(
    r: &Ray,
    world: &dyn Hittable,
    rng: &mut rand::rngs::ThreadRng,
    depth: i32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = record.material.scatter(r, &record, rng) {
            return attenuation * calculate_color(&scattered, world, rng, depth - 1);
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);

    (white * (1.0 - t)) + (blue * t)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let samples_per_pixel = 100;

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0)),
    )));

    let camera = Camera::new();
    let mut rng = rand::rng();

    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.random::<f64>()) / nx as f64;
                let v = (j as f64 + rng.random::<f64>()) / ny as f64;

                let r = camera.get_ray(u, v);
                color = color + calculate_color(&r, &world, &mut rng, 50);
            }

            color = color / samples_per_pixel as f64;
            color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

            let ir = (255.99 * color.x) as i32;
            let ig = (255.99 * color.y) as i32;
            let ib = (255.99 * color.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

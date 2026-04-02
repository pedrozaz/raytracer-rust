mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use rand::RngExt;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = Vec3::dot(ray.direction, ray.direction);
    let b = 2.0 * Vec3::dot(oc, ray.direction);
    let c = Vec3::dot(oc, oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn calculate_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(record) = world.hit(r, 0.0, f64::INFINITY) {
        return Vec3::new(
            record.normal.x + 1.0,
            record.normal.y + 1.0,
            record.normal.z + 1.0,
        ) * 0.5;
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

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0)));

    let camera = Camera::new();
    let mut rng = rand::rng();

    println!("P3\n {} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.random::<f64>()) / nx as f64;
                let v = (j as f64 + rng.random::<f64>()) / ny as f64;

                let r = camera.get_ray(u, v);
                color = color + calculate_color(&r, &world);
            }

            color = color / samples_per_pixel as f64;

            let ir = (255.99 * color.x) as i32;
            let ig = (255.99 * color.y) as i32;
            let ib = (255.99 * color.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

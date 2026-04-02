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
use rayon::prelude::*;
use sphere::Sphere;
use vec3::Vec3;

use crate::material::{Dielectric, Lambertian, Metal};

fn calculate_color(
    r: &Ray,
    world: &(dyn Hittable + Send + Sync),
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

fn random_scene(rng: &mut rand::rngs::ThreadRng) -> HittableList {
    let mut world = HittableList::new();

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::new(
                        rng.random::<f64>() * rng.random::<f64>(),
                        rng.random::<f64>() * rng.random::<f64>(),
                        rng.random::<f64>() * rng.random::<f64>(),
                    );
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::new(
                        0.5 * (1.0 + rng.random::<f64>()),
                        0.5 * (1.0 + rng.random::<f64>()),
                        0.5 * (1.0 + rng.random::<f64>()),
                    );
                    let fuzz = 0.5 * rng.random::<f64>();
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // Glass
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn main() {
    let nx = 1200;
    let ny = 800;
    let samples_per_pixel = 10;

    let mut rng = rand::rng();
    let mut world = random_scene(&mut rng);

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)),
    )));

    // Glass Sphere
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));

    // Negative ray
    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));

    // Camera Configs
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let aspect_ratio = nx as f64 / ny as f64;
    let vfov = 20.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    let mut rng = rand::rng();

    println!("P3\n{} {}\n255", nx, ny);

    let pixels: Vec<Vec<Vec3>> = (0..ny)
        .into_par_iter()
        .map(|row_idx| {
            let j = ny - 1 - row_idx;

            let mut row = Vec::with_capacity(nx as usize);
            let mut thread_rng = rand::rng();

            for i in 0..nx {
                let mut color = Vec3::new(0.0, 0.0, 0.0);

                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + thread_rng.random::<f64>()) / nx as f64;
                    let v = (j as f64 + thread_rng.random::<f64>()) / ny as f64;

                    let r = camera.get_ray(u, v, &mut thread_rng);
                    color = color + calculate_color(&r, &world, &mut thread_rng, 50);
                }

                color = color / samples_per_pixel as f64;
                color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());

                row.push(color);
            }

            eprintln!("Line processed...");
            row
        })
        .collect();

    for row in pixels {
        for color in row {
            let ir = (255.99 * color.x) as i32;
            let ig = (255.99 * color.y) as i32;
            let ib = (255.99 * color.z) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("Done.")
}

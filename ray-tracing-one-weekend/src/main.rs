use std::{io, rc::Rc};

use camera::Camera;
use color::Color;
use common::random_double;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dialetric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod common;
mod camera;
mod material;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center_sphere = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left_sphere = Rc::new(Dialetric::new(1.5));
    let material_right_sphere = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), material_ground, 100.0)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), material_center_sphere, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), material_left_sphere, -0.5)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), material_right_sphere, 0.5)));

    // Camera
    let camera = Camera::new();

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone.\n");
}


fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, common::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();

        if rec.mat.as_ref().unwrap().scatter(ray, &rec, &mut attenuation, &mut scattered) {
            return  attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

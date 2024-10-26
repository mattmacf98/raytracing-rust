use std::sync::Arc;

use camera::Camera;
use color::Color;
use common::{random_double, random_double_range};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Point3;

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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 250;
    const MAX_DEPTH: i32 = 50;

    let world = random_scene();

    // Camera
    let eye = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    camera.render(&world);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), ground_material, 1000.0)));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(a as f64 + 0.9 *random_double(), 0.2, b as f64 + 0.9 *random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = random_double();
                if choose_mat < 0.8 {
                    //Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, sphere_material, 0.2)));
                } else if choose_mat < 0.95 {
                    //Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, sphere_material, 0.2)));
                } else {
                    //Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, sphere_material, 0.2)));
                }
            }
        }
    }


    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        material1,
        1.0,
    )));
 
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        material2,
        1.0,
    )));
 
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        material3,
        1.0,
    )));
 
    world
}
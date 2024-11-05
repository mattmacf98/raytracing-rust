use std::sync::Arc;

use camera::Camera;
use color::Color;
use common::{random_double, random_double_range};
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use noise_texture::NoiseTexture;
use ray::Ray;
use sphere::Sphere;
use texture::{CheckerTexture, SolidColor};
use texture_image::TextureImage;
use vec3::{Point3, Vec3};

mod vec2;
mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod common;
mod camera;
mod material;
mod texture;
mod perlin;
mod texture_image;
mod interval;
mod noise_texture;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    perlin_spheres();
}

fn perlin_spheres() {
    let mut world = HittableList::new();
    let perlin_texture_one = NoiseTexture::new(4.0);
    let perlin_texture_two = NoiseTexture::new(4.0);

    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(perlin_texture_one))), 1000.0)));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, 2.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(perlin_texture_two))), 2.0)));

    let eye = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    camera.render(&world);
}

fn earth() {
    let mut world = HittableList::new();

    let earth_texture = TextureImage::new("assets/earthmap.jpg");
    let earth_mat = Lambertian::new(Box::new(earth_texture));

    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, 0.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(earth_mat), 2.0)));

    let eye = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    camera.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();
    let checker_texture_one = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let checker_texture_two = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, -10.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Metal::new(Box::new(checker_texture_one), 0.1)), 10.0)));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, 10.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(checker_texture_two))), 10.0)));

    let eye = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    camera.render(&world);

}

fn random_scene() {
    let mut world = HittableList::new();

    let checker_texture = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Arc::new(Lambertian::new(Box::new(checker_texture)));
    world.add(Box::new(Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), ground_material, 1000.0)));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(a as f64 + 0.9 *random_double(), 0.2, b as f64 + 0.9 *random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = random_double();
                if choose_mat < 0.8 {
                    //Diffuse
                    let moving_ray = Ray::new(center , Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0), 0.0);
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::from_color(albedo));
                    world.add(Box::new(Sphere::new(moving_ray, sphere_material, 0.2)));
                } else if choose_mat < 0.95 {
                    //Metal
                    let stationary_ray = Ray::new(center , Vec3::new(0.0, 0.0, 0.0), 0.0);
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(Box::new(SolidColor::from_rgb(albedo.x(), albedo.y(), albedo.z())), fuzz));
                    world.add(Box::new(Sphere::new(stationary_ray, sphere_material, 0.2)));
                } else {
                    //Glass
                    let stationary_ray = Ray::new(center , Vec3::new(0.0, 0.0, 0.0), 0.0);
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(stationary_ray, sphere_material, 0.2)));
                }
            }
        }
    }


    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        material1,
        1.0,
    )));
 
    let material2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(-4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        material2,
        1.0,
    )));
 
    let material3 = Arc::new(Metal::new(Box::new(SolidColor::from_rgb(0.7, 0.6, 0.5)), 0.0));
    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        material3,
        1.0,
    )));
 
    let eye = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    camera.render(&world);
}
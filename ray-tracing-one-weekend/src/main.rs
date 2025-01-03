use std::sync::Arc;

use camera::Camera;
use color::Color;
use common::{random_double, random_double_range};
use constant_medium::ConstantMedium;
use hittable_list::HittableList;
use material::{Dielectric, DiffuseLight, Empty, Lambertian, Metal};
use noise_texture::NoiseTexture;
use quad::Quad;
use ray::Ray;
use sphere::Sphere;
use texture::{CheckerTexture, SolidColor};
use texture_image::TextureImage;
use transfomation::{RotateY, Translate};
use vec3::{Point3, Vec3};

mod vec2;
mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod quad;
mod common;
mod camera;
mod material;
mod texture;
mod perlin;
mod texture_image;
mod interval;
mod noise_texture;
mod transfomation;
mod constant_medium;
mod onb;
mod pdf;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    cornell_box();
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Lambertian::from_color(Color::new(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::from_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));


    world.add(Arc::new(Quad::new(Point3::new(555.0,0.0,0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::new(green))));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::new(red))));
    world.add(Arc::new(Quad::new(Point3::new(113.0, 554.0, 127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), Arc::new(light))));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = Arc::new(Quad::get_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 = Arc::new(Quad::get_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white.clone()));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::from_color(box1.clone(), 0.01, Color::new(0.0, 0.0, 0.0))));
    world.add(Arc::new(ConstantMedium::from_color(box2.clone(), 0.01, Color::new(1.0, 1.0, 1.0))));

    let lights = Arc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), Arc::new(Empty::new())));


    let eye = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(600, 600, 200, MAX_DEPTH, eye, lookat, up, 40.0, 1.0, aperture, dist_to_focus, Color::new(0.0, 0.0, 0.0));

    camera.render(&world, lights);
}

fn cornell_box() {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    let red = Lambertian::from_color(Color::new(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let aluminum = Arc::new(Metal::from_color(Color::new(0.8, 0.85, 0.88), 0.0));
    let green = Lambertian::from_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));

    world.add(Arc::new(Quad::new(Point3::new(555.0,0.0,0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::new(green))));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Arc::new(red))));
    world.add(Arc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), Arc::new(light))));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Arc::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = Arc::new(Quad::get_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(190.0, 90.0, 190.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), glass, 90.0)));

    lights.add(Arc::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), Arc::new(Empty::new()))));

    let eye = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(600, 600, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 40.0, 1.0, aperture, dist_to_focus, Color::new(0.0, 0.0, 0.0));

    camera.render(&world, Arc::new(lights));
}

fn simple_light() {
    let mut world = HittableList::new();

    let perlin_texture_one = NoiseTexture::new(4.0);
    let perlin_texture_two = NoiseTexture::new(4.0);

    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(perlin_texture_one))), 1000.0)));
    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, 2.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(perlin_texture_two))), 2.0)));
    
    let mut lights = HittableList::new();

    let diff_light_one = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    let diff_light_two = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));

    lights.add(Arc::new(Quad::new(Point3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Arc::new(diff_light_one))));
    lights.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, 7.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(diff_light_two), 2.0)));


    let eye = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus, Color::new(0.0, 0.0, 0.0));

    camera.render(&world, Arc::new(lights));
}

fn quads() {
    let mut world = HittableList::new();

    let left_red = Lambertian::from_color(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::from_color(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::from_color(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::from_color(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::from_color(Color::new(0.2, 0.8, 0.8));

    world.add(Arc::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), Arc::new(left_red))));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), Arc::new(back_green))));
    world.add(Arc::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), Arc::new(right_blue))));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), Arc::new(upper_orange))));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), Arc::new(lower_teal))));

    let lights = Arc::new(HittableList::new());

    let eye = Point3::new(0.0, 0.0, 9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 80.0, ASPECT_RATIO, aperture, dist_to_focus, Color::new(0.7, 0.8, 1.0));

    camera.render(&world, lights);
}

fn perlin_spheres() {
    let mut world = HittableList::new();
    let perlin_texture_one = NoiseTexture::new(4.0);
    let perlin_texture_two = NoiseTexture::new(4.0);

    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(perlin_texture_one))), 1000.0)));
    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, 2.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(perlin_texture_two))), 2.0)));

    let lights = Arc::new(HittableList::new());

    let eye = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus, Color::new(0.7, 0.8, 1.0));

    camera.render(&world, lights);
}

fn earth() {
    let mut world = HittableList::new();
    let lights = HittableList::new();

    let earth_texture = TextureImage::new("assets/earthmap.jpg");
    let earth_mat = Lambertian::new(Box::new(earth_texture));

    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, 0.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(earth_mat), 2.0)));

    let eye = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus, Color::new(0.7, 0.8, 1.0));

    camera.render(&world, Arc::new(lights));
}

fn checkered_spheres() {
    let mut world = HittableList::new();
    let checker_texture_one = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let checker_texture_two = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, -10.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Metal::new(Box::new(checker_texture_one), 0.1)), 10.0)));
    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, 10.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), Arc::new(Lambertian::new(Box::new(checker_texture_two))), 10.0)));

    let lights = Arc::new(HittableList::new());

    let eye = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (eye - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus, Color::new(0.7, 0.8, 1.0));

    camera.render(&world, lights);

}

fn random_scene() {
    let mut world = HittableList::new();

    let checker_texture = CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Arc::new(Lambertian::new(Box::new(checker_texture)));
    world.add(Arc::new(Sphere::new(Ray::new(Point3::new(0.0, -1000.0, 0.0) , Vec3::new(0.0, 0.0, 0.0), 0.0), ground_material, 1000.0)));

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
                    world.add(Arc::new(Sphere::new(moving_ray, sphere_material, 0.2)));
                } else if choose_mat < 0.95 {
                    //Metal
                    let stationary_ray = Ray::new(center , Vec3::new(0.0, 0.0, 0.0), 0.0);
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(Box::new(SolidColor::from_rgb(albedo.x(), albedo.y(), albedo.z())), fuzz));
                    world.add(Arc::new(Sphere::new(stationary_ray, sphere_material, 0.2)));
                } else {
                    //Glass
                    let stationary_ray = Ray::new(center , Vec3::new(0.0, 0.0, 0.0), 0.0);
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(stationary_ray, sphere_material, 0.2)));
                }
            }
        }
    }


    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Ray::new(Point3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        material1,
        1.0,
    )));
 
    let material2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Ray::new(Point3::new(-4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        material2,
        1.0,
    )));
 
    let material3 = Arc::new(Metal::new(Box::new(SolidColor::from_rgb(0.7, 0.6, 0.5)), 0.0));
    world.add(Arc::new(Sphere::new(
        Ray::new(Point3::new(4.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        material3,
        1.0,
    )));

    let lights = Arc::new(HittableList::new());
 
    let eye = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let up = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, eye, lookat, up, 20.0, ASPECT_RATIO, aperture, dist_to_focus, Color::new(0.7, 0.8, 1.0));

    camera.render(&world, lights);
}
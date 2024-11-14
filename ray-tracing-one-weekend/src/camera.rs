use std::{io, sync::Arc};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{color::{self, Color}, common::{self, degrees_to_radians, random_double, random_double_range}, hittable::Hittable, hittable_list::HittableList, pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf}, ray::Ray, vec2::UV, vec3::{self, dot, Point3, Vec3}};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    background: Color,
    sqrt_samples: i32,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32,
         eye: Point3, lookat: Point3, up: Vec3, vfov: f64, aspect_ratio: f64, aperature: f64, focus_dist: f64, background: Color) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = vec3::unit_vector(eye - lookat);
        let u = vec3::unit_vector(vec3::cross(up, w));
        let v = vec3::cross(w, u);
 
        let origin = eye;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperature/2.0;

        let sqrt_samples = f64::sqrt(samples_per_pixel as f64) as i32;

 
        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            background,
            sqrt_samples
        }
    }

    pub fn render(&self, world: &HittableList, lights: Arc<dyn Hittable>) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in (0..self.image_height).rev() {
            eprint!("\rScanlines remaining: {}", j);
            let pixel_colors: Vec<_> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for s_i in 0..self.sqrt_samples {
                        for s_j in 0..self.sqrt_samples {
                            let u = (i as f64 + random_double()) / (self.image_width - 1) as f64;
                            let v = (j as f64 + random_double()) / (self.image_height - 1) as f64;
                            let r = self.get_ray(u, v, s_i, s_j);
                            pixel_color += self.ray_color(&r, world, lights.clone(), self.max_depth);
                        }
                    }
                    pixel_color
                })
                .collect();
            for pixel_color in pixel_colors {
                color::write_color(&mut io::stdout(), pixel_color, self.samples_per_pixel);
            }
        }

        eprint!("\nDone.\n");
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable, lights: Arc<dyn Hittable>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
    
        if let Some(hit_rec) = world.hit(ray, 0.001, common::INFINITY) {
            let color_from_emission = hit_rec.mat.emitted(&hit_rec, hit_rec.u, hit_rec.v, &hit_rec.p);

            return match hit_rec.mat.scatter(ray, &hit_rec) {
                Some(scatter_rec) => {
                    if let None = scatter_rec.pdf {
                        return scatter_rec.attenuation * self.ray_color(&scatter_rec.scattered, world, lights.clone(), depth - 1);
                    }
                    let light_pdf = HittablePdf::new(hit_rec.p, lights.clone());
                    let mixture_pdf = MixturePdf::new(Arc::new(light_pdf), scatter_rec.pdf.unwrap().clone());
                    
                    let scattered_ray = Ray::new(hit_rec.p, mixture_pdf.generate(), ray.time());
                    let pdf_value = mixture_pdf.value(scattered_ray.direction());

                    let scattered_pdf = hit_rec.mat.scatter_pdf(ray, &hit_rec, &scattered_ray);

                    let sample_color = self.ray_color(&scattered_ray, world, lights.clone(), depth - 1);
                    let color_from_scatter = (scatter_rec.attenuation * scattered_pdf * sample_color) / pdf_value;

                    return color_from_emission + color_from_scatter;
                },
                None => color_from_emission
            };
        } else {
            return self.background;
        }
    }

    fn get_ray(&self, s: f64, t: f64, s_i: i32, s_j: i32) -> Ray {
        let offset = self.sample_square_stratification(s_i, s_j);
        let pixel_sample = self.lower_left_corner + s * self.horizontal + t * self.vertical;

        let ray_origin = if self.lens_radius <= 0.0 {
            self.origin
        } else {
            let rd = self.lens_radius * vec3::random_in_unit_disk();
            self.origin + self.u * rd.x() + self.v * rd.y()
        };
        let ray_direction = pixel_sample - (ray_origin + offset);
        let ray_time = random_double();
        
        Ray::new(ray_origin + offset, ray_direction, ray_time)
    }

    fn sample_square_stratification(&self, s_i: i32, s_j: i32) -> Vec3 {
        let recip_sqrt_samples = 1.0 / self.sqrt_samples as f64;
        let px = ((s_i as f64 + random_double()) * recip_sqrt_samples) - 0.5;
        let py = ((s_j as f64 + random_double()) * recip_sqrt_samples) - 0.5;

        Vec3::new(px, py, 0.0)
    }
}
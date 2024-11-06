use core::f64;
use std::sync::Arc;

use crate::{color::Color, common::random_double, hittable::{HitRecord, Hittable}, material::{Isotropic, Material}, vec3::Vec3};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, phase_function: Arc<dyn Material>) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function
        }
    }

    pub fn from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {

        let phase_function = Arc::new(Isotropic::from_color(albedo));

        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let hit_rec_one =  self.boundary.hit(ray, -f64::INFINITY, f64::INFINITY);
        if hit_rec_one.is_none() {
            return None;
        }
        let mut hit_rec_one = hit_rec_one.unwrap();

        let hit_rec_two = self.boundary.hit(ray, hit_rec_one.t + 0.0001, f64::INFINITY);
        if hit_rec_two.is_none() {
            return None;
        }
        let mut hit_rec_two = hit_rec_two.unwrap();

        if hit_rec_one.t < t_min {
            hit_rec_one.t = t_min;
        }
        if hit_rec_two.t > t_max {
            hit_rec_two.t = t_max;
        }

        if hit_rec_one.t >= hit_rec_two.t {
            return None;
        }
        
        if hit_rec_one.t < 0.0 {
            hit_rec_one.t = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (hit_rec_two.t - hit_rec_one.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(random_double());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit_rec_one.t + hit_distance / ray_length;

        Some(HitRecord {
            p: ray.at(t),
            normal: Vec3::new(1.0, 0.0, 0.0), //arbitrary
            mat: self.phase_function.clone(),
            t: t,
            u: 0.0,
            v: 0.0,
            front_face: true, //arbitrary
        })
    }
}
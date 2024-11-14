use std::sync::Arc;

use crate::{material::Material, ray::Ray, vec3::{self, Point3, Vec3}};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_norm) < 0.0;
        self.normal = if self.front_face {
            outward_norm
        } else {
            -outward_norm
        };
    }
}

pub trait Hittable: Send + Sync{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        0.0
    }

    fn random(&self, origin: Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
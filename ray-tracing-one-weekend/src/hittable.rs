use std::rc::Rc;

use crate::{material::Material, ray::Ray, vec3::{self, Point3, Vec3}};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_norm) < 0.0;
        self.normal = if self.front_face {
            outward_norm
        } else {
            -outward_norm
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
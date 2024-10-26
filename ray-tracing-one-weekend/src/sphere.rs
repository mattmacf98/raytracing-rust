use std::sync::Arc;

use crate::{hittable::{HitRecord, Hittable}, material::Material, vec3::{self, Point3}};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, mat: Arc<dyn Material>, radius: f64) -> Sphere {
        Sphere {
            center,
            mat,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = f64::sqrt(discriminant);

        // find nearest root
        let mut root = (-half_b - sqrt_disc) / a;
        if root <= t_min || root >= t_max {
            root = (-half_b + sqrt_disc) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: ray.at(root),
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };

        let outward_norm = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_norm);
        Some(rec)
    }
}
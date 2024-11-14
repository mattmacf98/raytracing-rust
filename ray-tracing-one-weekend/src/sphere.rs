use core::f64;
use std::sync::Arc;

use crate::{common::random_double, hittable::{HitRecord, Hittable}, material::Material, onb::Onb, ray::Ray, vec2::UV, vec3::{self, Point3, Vec3}};

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center: Ray, mat: Arc<dyn Material>, radius: f64) -> Sphere {
        Sphere {
            center,
            mat,
            radius
        }
    }

    fn get_sphere_uv(p: Point3) -> UV {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + f64::consts::PI;

        let u = phi / (2.0 * f64::consts::PI);
        let v = theta / f64::consts::PI;

        UV::new(u, v)
    }

    fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 =  random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * (f64::sqrt(1.0 - (radius * radius)/distance_squared) - 1.0);

        let phi = 2.0 * f64::consts::PI * r1;
        let x = f64::cos(phi) * f64::sqrt(1.0 - z * z);
        let y = f64::sin(phi) * f64::sqrt(1.0 - z * z);

        Vec3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc = ray.origin() - current_center;
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

        let outward_norm = (ray.at(root) - current_center) / self.radius;
        let uv: UV = Sphere::get_sphere_uv(outward_norm);
        let mut rec = HitRecord {
            t: root,
            p: ray.at(root),
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
            u: uv.x(),
            v: uv.y(),
        };

        
        rec.set_face_normal(ray, outward_norm);
        
        Some(rec)
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        if let None = self.hit(&Ray::new(origin, direction, 0.0), 0.001, f64::INFINITY) {
            return 0.0;
        }

        let dist_squared = (self.center.at(0.0) - origin).length_squared();
        let cos_theta_max = f64::sqrt(1.0 - self.radius * self.radius / dist_squared);
        let solid_angle = 2.0 * f64::consts::PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: Point3) -> Vec3 {
        let direction = self.center.at(0.0) - origin;
        let distance_squared = direction.length_squared();

        let uwu = Onb::new(&direction);
        uwu.transform(Sphere::random_to_sphere(self.radius, distance_squared))
    }
}
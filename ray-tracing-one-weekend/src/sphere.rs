use crate::{hittable::Hittable, vec3::{self, Point3}};

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_disc = f64::sqrt(discriminant);

        // find nearest root
        let mut root = (-half_b - sqrt_disc) / a;
        if root <= t_min || root >= t_max {
            root = (-half_b + sqrt_disc) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        let outward_norm = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_norm);
        true
    }
}
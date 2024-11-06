use std::sync::Arc;

use crate::{common::degrees_to_radians, hittable::Hittable, ray::Ray, vec3::{Point3, Vec3}};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Translate {
            object,
            offset
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

         match self.object.hit(&offset_ray, t_min, t_max) {
            None => return None,
            Some(mut hit_record) => {
                hit_record.p += self.offset;
                return Some(hit_record);
            }
         }
    }
}


pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        RotateY {
            object,
            sin_theta,
            cos_theta
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        let origin = Point3::new((self.cos_theta * ray.origin().x()) - (self.sin_theta * ray.origin().z()), ray.origin().y(), (self.sin_theta * ray.origin().x()) + (self.cos_theta * ray.origin().z()));
        let direction = Vec3::new((self.cos_theta * ray.direction().x()) - (self.sin_theta * ray.direction().z()), ray.direction().y(), (self.sin_theta * ray.direction().x()) + (self.cos_theta * ray.direction().z()));

        let rotated_ray = Ray::new(origin, direction, ray.time());

        if let Some(mut hit_record) = self.object.hit(&rotated_ray, t_min, t_max) {
            hit_record.p = Point3::new((self.cos_theta * hit_record.p.x()) + (self.sin_theta * hit_record.p.z()), hit_record.p.y(), (-self.sin_theta * hit_record.p.x()) + (self.cos_theta * hit_record.p.z()));
            hit_record.normal= Point3::new((self.cos_theta * hit_record.normal.x()) + (self.sin_theta * hit_record.normal.z()), hit_record.normal.y(), (-self.sin_theta * hit_record.normal.x()) + (self.cos_theta * hit_record.normal.z()));

            return Some(hit_record);
        } else {
            return None;
        }
    }
}
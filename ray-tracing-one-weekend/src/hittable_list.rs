use std::sync::Arc;

use crate::{common::random_int_range, hittable::{HitRecord, Hittable}, vec3::{Point3, Vec3}};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }

    fn pdf_value(&self, origin: Point3, direction: Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        self.objects.iter()
            .map(|object| weight * object.pdf_value(origin, direction))
            .sum()
    }
    
    fn random(&self, origin: Point3) -> Vec3 {
        let index: usize = random_int_range(0, (self.objects.len() - 1) as i32) as usize;
        self.objects[index].random(origin)
    }
}
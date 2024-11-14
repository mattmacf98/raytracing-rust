use std::sync::Arc;

use crate::{common::random_double, hittable::Hittable, onb::Onb, vec3::{dot, random_unit_vector, unit_vector, Point3, Vec3}};

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct SpherePdf {}

impl SpherePdf {
    pub fn new() -> SpherePdf {
        SpherePdf {}
    }
}

impl Pdf for SpherePdf {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: Onb
}

impl CosinePdf {
    pub fn new(w: Vec3) -> CosinePdf {
        CosinePdf {
            uvw: Onb::new(&w)
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cos_theta = dot(unit_vector(direction), self.uvw.w());
        f64::max(0.0, cos_theta / std::f64::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(Vec3::random_cosine_direction())
    }
}

pub struct HittablePdf {
    origin: Point3,
    objects: Arc<dyn Hittable>
}

impl HittablePdf {
    pub fn new(origin: Point3, objects: Arc<dyn Hittable>) -> HittablePdf {
        HittablePdf { origin, objects }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}

pub struct MixturePdf {
    p: Arc<dyn Pdf>,
    q: Arc<dyn Pdf>
}

impl MixturePdf {
    pub fn new(p: Arc<dyn Pdf>, q: Arc<dyn Pdf>) -> MixturePdf {
        MixturePdf { p, q }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p.value(direction) + 0.5 * self.q.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p.generate()
        } else {
            self.q.generate()
        }
    }
}
use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
    time: f64
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            dir,
            time
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    } 

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t:f64) -> Point3 {
        self.origin + t * self.dir
    }
}
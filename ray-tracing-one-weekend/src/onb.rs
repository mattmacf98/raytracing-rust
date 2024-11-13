use crate::vec3::{self, Vec3};

pub struct Onb {
    axis: [Vec3; 3]
}

impl Onb {
    pub fn new(n: &Vec3) -> Self {
        let normal = vec3::unit_vector(n.clone());
        let a = if f64::abs(normal.x()) > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let u = vec3::unit_vector(vec3::cross(normal, a));
        let v = vec3::cross(normal, u);

        Onb {
            axis: [u, v, normal]
        }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }   

    pub fn transform(&self, v: Vec3) -> Vec3 {
        self.axis[0] * v.x() + self.axis[1] * v.y() + self.axis[2] * v.z()
    }
}  
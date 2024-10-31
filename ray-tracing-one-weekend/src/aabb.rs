use crate::{hittable::{HitRecord, Hittable}, interval::Interval, vec3::Point3};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB {
            x,
            y,
            z
        }
    }

    pub fn from_sub_aabbs(a: AABB, b: AABB) -> AABB {
        let x = Interval::from_sub_intervals(a.x, b.x);
        let y = Interval::from_sub_intervals(a.y, b.y);
        let z = Interval::from_sub_intervals(a.z, b.z);

        AABB {
            x,
            y,
            z
        }
    }

    pub fn from_points(a: Point3, b: Point3) -> AABB {
        let x =  Interval::new(f64::min(a.x(), b.x()), f64::max(a.x(), b.x()));
        let y =  Interval::new(f64::min(a.y(), b.y()), f64::max(a.y(), b.y()));
        let z =  Interval::new(f64::min(a.z(), b.z()), f64::max(a.z(), b.z()));

        AABB {
            x,
            y,
            z
        }
    }

    pub fn axis_inteval(&self, n: i32) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis")
        }
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: &crate::ray::Ray, interval: Interval) -> Option<HitRecord> {
        for i in 0..4 {
            let axis_interval = Self::axis_inteval(&self, i);
            let adinv = 1.0 / ray.direction().e()[i as usize];

            let t0 = (axis_interval.min - ray.origin().e()[i as usize]) * adinv;
            let t1 = (axis_interval.max - ray.origin().e()[i as usize]) * adinv;

            let mut result_interval = Interval::new(interval.min, interval.max);
            if t0 < t1 {
                if t0 > interval.min {result_interval.min = t0}
                if t1 < interval.max {result_interval.max = t1}
            } else {
                if t1 > interval.min {result_interval.min = t1}
                if t0 < interval.max {result_interval.max = t0}
            }

            if result_interval.max <= result_interval.min {
                return None;
            }
        }


        let rec = HitRecord {
            t: Default::default(),
            p: Default::default(),
            mat: None,
            normal: Default::default(),
            front_face: Default::default(),
        };

        Some(rec)
    }
}
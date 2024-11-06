use std::sync::Arc;

use crate::{hittable::{self, HitRecord, Hittable}, hittable_list::HittableList, material::Material, vec2::Vec2, vec3::{self, cross, dot, unit_vector, Point3, Vec3}};

pub struct Quad {
    q: Point3,
    v: Vec3,
    u: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f64,
    mat: Arc<dyn Material>
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = dot(normal, q);
        let w = n / dot(n, n);

        Quad {
            q,
            u,
            v,
            w,
            mat,
            d,
            normal
        }
    }

    pub fn get_box(min: Point3, max: Point3, mat: Arc<dyn Material>) -> HittableList {
        let mut sides = HittableList::new();

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Arc::new(Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, mat.clone()))); // front
        sides.add(Arc::new(Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, mat.clone()))); // right
        sides.add(Arc::new(Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, mat.clone()))); // back
        sides.add(Arc::new(Quad::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, mat.clone()))); // left
        sides.add(Arc::new(Quad::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, mat.clone()))); // top
        sides.add(Arc::new(Quad::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, mat.clone()))); // bottom
        sides
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let denom = dot(self.normal, ray.direction());

        // ray parallel to plane
        if f64::abs(denom) < 1e-8 {
            return None;
        }

        let t = (self.d - dot(self.normal, ray.origin())) / denom;
        if t < t_min || t > t_max {
            return None;
        }

        let intersection = ray.at(t);

        let planar_hit_point = intersection - self.q;

        let alpha = dot(self.w, cross(planar_hit_point, self.v));
        let beta = dot(self.w, cross(self.u, planar_hit_point));

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }


        let mut rec = HitRecord {
            t: t,
            p: intersection,
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
            u: alpha,
            v: beta,
        };

        rec.set_face_normal(ray, self.normal);
        
        Some(rec)
    }
}
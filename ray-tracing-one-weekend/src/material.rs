use std::sync::Arc;

use crate::{color::Color, common::random_double, hittable::HitRecord, onb::Onb, pdf::{CosinePdf, Pdf, SpherePdf}, ray::Ray, texture::{SolidColor, Texture}, vec3::{self, dot, random_unit_vector, unit_vector, Point3, Vec3}};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf: Option<Arc<dyn Pdf>>,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, hit_rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Empty {}

impl Empty {
    pub fn new() -> Empty {
        Empty {}
    }
}

impl Material for Empty {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}

pub struct Lambertian {
    albedo: Box<dyn Texture>
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Lambertian {
        Lambertian {
            albedo
        }
    }

    pub fn from_color(albedo_color: Color) -> Lambertian {
        let albedo = SolidColor::new(albedo_color);

        Lambertian {
            albedo: Box::new(albedo)
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {

        let uvw = Onb::new(&rec.normal);
        let scatter_direction = uvw.transform(Vec3::random_cosine_direction());
        
        Some(ScatterRecord {
            attenuation: self.albedo.get_color(rec.u, rec.v, &rec.p),
            scattered: Ray::new(rec.p, scatter_direction, r_in.time()),
            pdf: Some(Arc::new(CosinePdf::new(rec.normal)))
        })
    }

    fn scatter_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = vec3::dot(rec.normal, unit_vector(scattered.direction()));
        if cos_theta > 0.0 {
            cos_theta / std::f64::consts::PI
        } else {
            0.0
        }
    }
}

pub struct Metal {
    albedo: Box<dyn Texture>,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Box<dyn Texture>, f: f64) -> Metal {
        Metal { 
            albedo,
            fuzz: if f < 1.0 {f} else {1.0}
        }
    }

    pub fn from_color(albedo_color: Color, f: f64) -> Metal {
        let albedo = SolidColor::new(albedo_color);

        Metal {
            albedo: Box::new(albedo),
            fuzz: if f < 1.0 {f} else {1.0}
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);

        let scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere(), r_in.time());
        if  vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo.get_color(rec.u, rec.v, &rec.p),
                scattered,
                pdf: None
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ior: f64
}

impl Dielectric {
    pub fn new(ior: f64) -> Dielectric {
        Dielectric {
            ior
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Schlcik's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {1.0 / self.ior} else {self.ior};
        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = f64::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() { 
             vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(rec.p, direction, r_in.time()),
            pdf: None
        })
    }
}

pub struct DiffuseLight {
    albedo: Box<dyn Texture>
}

impl DiffuseLight {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        DiffuseLight {
            albedo
        }
    }

    pub fn from_color(albedo_color: Color) -> Self {
        let albedo = SolidColor::new(albedo_color);

        DiffuseLight {
            albedo: Box::new(albedo)
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, hit_rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if hit_rec.front_face {
            self.albedo.get_color(u, v, p)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

pub struct Isotropic {
    albedo: Box<dyn Texture>
}

impl Isotropic {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Isotropic {
            albedo
        }
    }

    pub fn from_color(albedo_color: Color) -> Self {
        let albedo = SolidColor::new(albedo_color);

        Isotropic {
            albedo: Box::new(albedo)
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scattered = Ray::new(rec.p, random_unit_vector(), r_in.time());
        let attenuation = self.albedo.get_color(rec.u, rec.v, &rec.p);


        Some(ScatterRecord {
            attenuation,
            scattered,
            pdf: Some(Arc::new(SpherePdf::new()))
        })
    }

    fn scatter_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1.0 / (4.0 * std::f64::consts::PI)
    }
}
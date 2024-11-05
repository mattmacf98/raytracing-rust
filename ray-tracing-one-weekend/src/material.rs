use crate::{color::Color, common::random_double, hittable::HitRecord, ray::Ray, texture::{SolidColor, Texture}, vec3};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
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
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        Some(ScatterRecord {
            attenuation: self.albedo.get_color(rec.u, rec.v, &rec.p),
            scattered: Ray::new(rec.p, scatter_direction, r_in.time())
        })
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
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);

        let scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere(), r_in.time());
        if  vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo.get_color(rec.u, rec.v, &rec.p),
                scattered
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
            scattered: Ray::new(rec.p, direction, r_in.time())
        })
    }
}
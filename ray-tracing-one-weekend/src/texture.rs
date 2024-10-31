use crate::{color::Color, vec3::Point3};

pub trait Texture: Send + Sync {
    fn get_color(&self, u: f64, v:f64, point: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color
}

impl SolidColor {
    pub fn new(albedo: Color) -> SolidColor {
        SolidColor {
            albedo
        }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        let albedo = Color::new(red, green, blue);
        SolidColor {
            albedo
        }
    }
}

impl Texture for SolidColor {
    fn get_color(&self, _u: f64, _v:f64, _point: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd
        }
    }

    pub fn from_colors(scale: f64, color1: Color, color2: Color) -> CheckerTexture {
        let texture1 = SolidColor::new(color1);
        let texture2 = SolidColor::new(color2);

        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Box::new(texture1),
            odd: Box::new(texture2)
        }
    }
}


impl Texture for CheckerTexture {
    fn get_color(&self, u: f64, v:f64, point: &Point3) -> Color {
        let x = f64::floor(self.inv_scale * point.x()) as i32;
        let y = f64::floor(self.inv_scale * point.y()) as i32;
        let z = f64::floor(self.inv_scale * point.z()) as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.get_color(u, v, point)
        } else {
            self.odd.get_color(u, v, point)
        }
    }
}
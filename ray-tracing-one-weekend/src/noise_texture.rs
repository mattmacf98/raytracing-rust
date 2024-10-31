use crate::{color::Color, perlin::Perlin, texture::Texture, vec3::Point3};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale
        }
    }
}

impl Texture for NoiseTexture {
    fn get_color(&self, u: f64, v:f64, point: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + f64::sin(self.scale * point.z() + 10.0 * self.noise.turbulence(point, 7)))
    }
}
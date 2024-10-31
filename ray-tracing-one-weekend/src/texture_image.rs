use image::{DynamicImage, GenericImageView, ImageReader, Pixel};

use crate::{color::Color, interval::Interval, texture::Texture};

pub struct TextureImage {
    image: DynamicImage
}

impl TextureImage {
    pub fn new(image_file: &str) -> TextureImage {
        let image = ImageReader::open(image_file).expect("should be a valid image path").decode().expect("should decode");
    
        TextureImage {
            image
        }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        let clamped_x = u32::clamp(x, 0, self.width());
        let calmped_y = u32::clamp(y, 0, self.height());

        let pixel = self.image.get_pixel(clamped_x, calmped_y).to_rgb();

        let r = (pixel.0[0] as f64) / 255.0;
        let g = (pixel.0[1] as f64) / 255.0;
        let b = (pixel.0[2] as f64) / 255.0;

        Color::new(r, g, b)
    }
}

impl Texture for TextureImage {
    fn get_color(&self, u: f64, v:f64, _point: &crate::vec3::Point3) -> Color {
        let u_clamped = f64::clamp(u, 0.0, 1.0);
        let v_clamped = 1.0 - f64::clamp(v, 0.0, 1.0);

        let x = (u_clamped * self.width() as f64) as u32; 
        let y = (v_clamped * self.height() as f64) as u32; 

        self.pixel_data(x, y)
    }
}

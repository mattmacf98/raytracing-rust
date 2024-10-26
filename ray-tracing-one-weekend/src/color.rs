use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    writeln!(
        out,
        "{} {} {}",
        (256.0 * f64::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * f64::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * f64::clamp(b, 0.0, 0.999)) as i32,
    ).expect("writing color");
}
use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = if pixel_color.x().is_nan() { 0.0 } else { pixel_color.x() };
    let mut g = if pixel_color.y().is_nan() { 0.0 } else { pixel_color.y() };
    let mut b = if pixel_color.z().is_nan() { 0.0 } else { pixel_color.z() };

    

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    writeln!(
        out,
        "{} {} {}",
        (256.0 * f64::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * f64::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * f64::clamp(b, 0.0, 0.999)) as i32,
    ).expect("writing color");
}
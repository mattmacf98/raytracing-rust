use std::io;

use color::Color;

mod vec3;
mod color;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r: f64 = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let pixel_color = Color::new(r, g, b);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprint!("\nDone.\n");
}


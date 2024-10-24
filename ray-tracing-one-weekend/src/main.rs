fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r: f32 = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let int_r = (255.99 * r) as i32;
            let int_g = (255.99 * g) as i32;
            let int_b = (255.99 * b) as i32;

            print!("{} {} {}\n", int_r, int_g, int_b);
        }
    }

    eprint!("\nDone.\n");
}

use std::io;

use rust_ray::bitmap::{Bitmap, Color};
use rust_ray::math::inv_lerp;

fn main() -> io::Result<()> {
    let mut prev_progress = -1;
    let mut r = Bitmap::new(640, 480, Color::black());

    for y in 0..r.height() {
        let y_ratio = inv_lerp(y as f32, 0.0, (r.height() - 1) as f32);

        let progress = (100.0 * y_ratio) as i32;
        if progress > prev_progress {
            println!("{}%", progress);
            prev_progress = progress;
        }

        for x in 0..r.width() {
            let x_ratio = inv_lerp(x as f32, 0.0, (r.width() - 1) as f32);

            let pixel_color = (x_ratio, y_ratio, 0.25);
            r.set_pixel(x, y, pixel_color);
        }
    }

    println!("saving 'output.bmp'...");
    r.save("output.bmp")?;

    println!("done.");
    Ok(())
}

use rust_ray::bitmap::{Bitmap, Color};
use std::io;

fn main() -> io::Result<()> {
    let mut bmp = Bitmap::new(256, 256, Color::black());

    for y in 0..bmp.height() {
        for x in 0..bmp.width() {
            let r = x as f32 / (bmp.width() - 1) as f32;
            let g = y as f32 / (bmp.height() - 1) as f32;
            let b = 0.25;

            bmp.set_pixel(x, y, (r, g, b));
        }
    }

    bmp.save("example.bmp")
}

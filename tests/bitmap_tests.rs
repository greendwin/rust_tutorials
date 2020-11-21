use rust_ray::bitmap::{Bitmap, Color};

#[test]
fn bitmap_new() {
    let bmp = Bitmap::new(16, 8, Color::new(1, 2, 3));

    assert_eq!(bmp.width(), 16);
    assert_eq!(bmp.height(), 8);
}

#[test]
fn bitmap_new_fill_color() {
    let bmp = Bitmap::new(16, 16, Color::new(1, 2, 3));
    assert_eq!(bmp.get_pixel(5, 5), Color::new(1, 2, 3));
}

#[test]
fn bitmap_set_pixel() {
    let mut bmp = Bitmap::new(16, 16, Color::black());
    bmp.set_pixel(5, 10, Color::red());

    assert_eq!(bmp.get_pixel(5, 10), Color::red());
}

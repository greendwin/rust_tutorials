// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::io;

use rust_ray::bitmap::Bitmap;
use rust_ray::math::*;
use rust_ray::ray::Ray;
use rust_ray::vec3::Vec3;

fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere((0.2, 0.0, -1.0).into(), 0.3, ray) {
        return (1, 0, 0).into();
    }

    let norm_dir = ray.dir.norm();
    let t = 0.5 * (norm_dir.y + 1.0);

    lerp(t, Vec3::new(1, 1, 1), Vec3::new(0.5, 0.7, 1.0))
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = center - ray.orig;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius * radius;

    let discriminant = b * b - a * c * 4.0;

    discriminant > 0.0
}

fn main() -> io::Result<()> {
    // Image
    const IMAGE_WIDTH: usize = 640;
    const IMAGE_HEIGHT: usize = 480;
    const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

    // Camera

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vec3::new(0, 0, 0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0, 0);
    let vertical = Vec3::new(0, VIEWPORT_HEIGHT, 0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0, 0, FOCAL_LENGTH);

    // Render

    let mut prev_progress = -1;
    let mut r = Bitmap::new(IMAGE_WIDTH, IMAGE_HEIGHT, (0, 0, 0));

    for y in 0..r.height() {
        let y_ratio = inv_lerp(y as f32, 0.0, (r.height() - 1) as f32);

        let progress = (100.0 * y_ratio) as i32;
        if progress > prev_progress {
            println!("{}%", progress);
            prev_progress = progress;
        }

        for x in 0..r.width() {
            let u = inv_lerp(x as f64, 0.0, (r.width() - 1) as f64);
            let v = inv_lerp(y as f64, 0.0, (r.height() - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );

            let pixel_color = ray_color(&ray);
            r.set_pixel(x, y, pixel_color);
        }
    }

    println!("saving 'output.bmp'...");
    r.save("output.bmp")?;

    println!("done.");
    Ok(())
}

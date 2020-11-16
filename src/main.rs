// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::f64;
use std::io;
use std::rc::Rc;

use rust_ray::bitmap::Bitmap;
use rust_ray::math::*;
use rust_ray::utils::*;
use rust_ray::world::*;

fn ray_color(ray: &Ray, hittable: &impl HitRay, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some((hit, mat)) = hittable.hit(ray, 0.001, f64::MAX) {
        if let Some((next_ray, color)) = mat.scatter(&ray, &hit) {
            return color * ray_color(&next_ray, hittable, depth - 1);
        }

        return Vec3::zero();
    }

    let norm_dir = ray.dir.norm();
    let t = 0.5 * (norm_dir.y + 1.0);
    lerp(t, Vec3::new(1, 1, 1), Vec3::new(0.5, 0.7, 1.0))
}

fn release_debug<T>(_rel_val: T, _debug_val: T) -> T {
    #[cfg(debug_assertions)]
    return _debug_val;

    #[cfg(not(debug_assertions))]
    return _rel_val;
}

fn main() -> io::Result<()> {
    // Image

    let image_width = 640;
    let image_height = 480;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel = release_debug(100, 4);
    let max_depth = release_debug(50, 2);

    // World

    let mat_ground: MaterialPtr = DiffuseMat::new((0.8, 0.8, 0));
    let mat_center: MaterialPtr = DiffuseMat::new((0.1, 0.2, 0.5));
    let mat_left: MaterialPtr = DielectricMat::new(1.5);
    let mat_right: MaterialPtr = MetalMat::new((0.8, 0.6, 0.2), 0.2);

    let mut scene = Scene::new();
    scene.add(Sphere::new((0, -100.5, -1), 100, Rc::clone(&mat_ground)));
    scene.add(Sphere::new((0, 0, -1), 0.5, Rc::clone(&mat_center)));
    scene.add(Sphere::new((-1, 0, -1), 0.5, Rc::clone(&mat_left)));
    scene.add(Sphere::new((-1, 0, -1), -0.45, Rc::clone(&mat_left)));
    scene.add(Sphere::new((1, 0, -1), 0.5, Rc::clone(&mat_right)));

    // Camera

    let lookfrom = Vec3::new(-2, 2, 1);
    let lookat = Vec3::new(-0.1, 0, -1);
    let vup = Vec3::new(0, 1, 0);
    let camera = Camera::new(lookfrom, lookat, vup, 45.0, aspect_ratio);

    // Render

    let mut prev_progress = -1;
    let mut r = Bitmap::new(image_width, image_height, (0, 0, 0));

    for y in 0..r.height() {
        let y_ratio = inv_lerp(y as f64, 0.0, (r.height() - 1) as f64);

        let progress = (100.0 * y_ratio) as i32;
        if progress > prev_progress {
            println!("{}%", progress);
            prev_progress = progress;
        }

        for x in 0..r.width() {
            let mut accum_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let x = x as f64;
                let y = y as f64;

                let u = inv_lerp(x + random(), 0.0, (r.width() - 1) as f64);
                let v = inv_lerp(y + random(), 0.0, (r.height() - 1) as f64);
                let ray = camera.get_ray(u, v);
                accum_color += ray_color(&ray, &scene, max_depth);
            }

            r.set_pixel(x, y, (accum_color / samples_per_pixel).sqrt());
        }
    }

    println!("saving 'output.bmp'...");
    r.save("output.bmp")?;

    println!("done.");
    Ok(())
}

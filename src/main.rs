// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::io;

use rust_ray::bitmap::Bitmap;
use rust_ray::math::*;
use rust_ray::utils::*;
use rust_ray::world::*;

fn ray_color(ray: &Ray, hittable: &impl HitRay, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit) = hittable.hit(ray, 0.001, f64::MAX) {
        // let next_dir = hit.norm + rand_vec3_in_unit_sphere().norm(); // lambertian
        let next_dir = hit.norm + rand_vec3_in_hemisphere(hit.norm); // old times

        let next_ray = Ray::new(hit.pt, next_dir);
        return ray_color(&next_ray, hittable, depth - 1) * 0.5;
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

    // Scene

    let mut scene = Scene::new();
    scene.add(Sphere::new((0, 0, -1), 0.5));
    scene.add(Sphere::new((0, -100.5, -1), 100));

    // Camera

    let camera = Camera::new(aspect_ratio);

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

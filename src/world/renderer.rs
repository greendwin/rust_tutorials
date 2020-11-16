use super::camera::Camera;
use crate::math::*;
use crate::utils::*;

pub trait RenderTarget {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set_pixel(&mut self, x: usize, y: usize, color: Vec3);
}

pub struct Renderer {
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Renderer {
    pub fn new(samples_per_pixel: usize, max_depth: usize) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, camera: &Camera, scene: &impl HitRay, target: &mut impl RenderTarget) {
        let mut prev_progress = -1;

        let r_height = target.height() as f64;
        let r_width = target.width() as f64;

        for y in 0..target.height() {
            let y_ratio = inv_lerp(y as f64, 0.0, r_height - 1.0);
            let progress = (100.0 * y_ratio) as i32;
            if progress > prev_progress {
                println!("{}%", progress);
                prev_progress = progress;
            }
            for x in 0..target.width() {
                let mut accum_color = Vec3::zero();
                for _ in 0..self.samples_per_pixel {
                    let x = x as f64;
                    let y = y as f64;
                    let u = inv_lerp(x + random(), 0.0, r_width - 1.0);
                    let v = inv_lerp(y + random(), 0.0, r_height - 1.0);
                    let ray = camera.get_ray(u, v);
                    accum_color += ray_color(&ray, scene, self.max_depth as i32);
                }

                let color = (accum_color / self.samples_per_pixel as f64).sqrt();
                target.set_pixel(x, y, color);
            }
        }
    }
}

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

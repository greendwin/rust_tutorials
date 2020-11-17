use super::camera::Camera;
use crate::math::*;
use crate::utils::*;

pub trait RenderTarget {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set_pixel(&mut self, x: usize, y: usize, color: Vec3);
}

pub enum RenderProgress {
    InProgress(usize), // percents
    Done,
}

use RenderProgress::*;

pub struct Renderer<'a, Scene, Target> {
    samples_per_pixel: usize,
    max_depth: usize,

    camera: &'a Camera,
    scene: &'a Scene,
    target: &'a mut Target,

    // iteration data
    cur_y: usize,
}

impl<'a, Scene, Target> Renderer<'a, Scene, Target>
where
    Scene: HitRay,
    Target: RenderTarget,
{
    pub fn new(
        samples_per_pixel: usize,
        max_depth: usize,
        camera: &'a Camera,
        scene: &'a Scene,
        target: &'a mut Target,
    ) -> Self {
        let init_y = target.height() - 1;
        Self {
            samples_per_pixel,
            max_depth,
            camera,
            scene,
            target,
            cur_y: init_y,
        }
    }

    pub fn target_mut(&mut self) -> &mut Target {
        &mut self.target
    }

    pub fn next(&mut self) -> RenderProgress {
        if self.cur_y == usize::MAX {
            return Done;
        }

        render_next(
            self.cur_y,
            self.samples_per_pixel,
            self.max_depth,
            self.camera,
            self.scene,
            self.target,
        );

        if self.cur_y == 0 {
            self.cur_y = usize::MAX;
            return InProgress(100);
        }

        self.cur_y -= 1;

        let y_ratio = inv_lerp(self.cur_y as f64, (self.target.height() - 1) as f64, 0.0);
        RenderProgress::InProgress((y_ratio * 100.0).round() as usize)
    }
}

fn render_next(
    y: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    camera: &Camera,
    scene: &impl HitRay,
    target: &mut impl RenderTarget,
) {
    let u_last = (target.width() - 1) as f64;
    let v_last = (target.height() - 1) as f64;

    for x in 0..target.width() {
        let mut accum_color = Vec3::zero();
        for _ in 0..samples_per_pixel {
            let x = x as f64;
            let y = y as f64;
            let u = inv_lerp(x + random(), 0.0, u_last);
            let v = inv_lerp(y + random(), 0.0, v_last);
            let ray = camera.get_ray(u, v);
            accum_color += ray_color(&ray, scene, max_depth as i32);
        }

        let color = (accum_color / samples_per_pixel as f64).sqrt();
        target.set_pixel(x, y, color);
    }
}

fn ray_color(ray: &Ray, scene: &impl HitRay, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some((hit, mat)) = scene.hit(ray, 0.001, f64::MAX) {
        if let Some((next_ray, color)) = mat.scatter(&ray, &hit) {
            return color * ray_color(&next_ray, scene, depth - 1);
        }

        return Vec3::zero();
    }

    let norm_dir = ray.dir.norm();
    let t = 0.5 * (norm_dir.y + 1.0);
    lerp(t, Vec3::new(1, 1, 1), Vec3::new(0.5, 0.7, 1.0))
}

use std::sync::Arc;

use super::{Camera, Scene};
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

struct JobResult {
    x: usize,
    y: usize,
    color: Vec3,
}

pub struct Renderer<'a, Scene, Target> {
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pub scene: Arc<Scene>,
    pub camera: &'a Camera,
    pub target: &'a mut Target,
    pub num_threads: usize,

    pub ambient_grad: (Vec3, Vec3),

    // iteration data
    jobs: JobRunner<JobResult>,
    cur_y: usize,
    cur_samples: usize,
    accum_colors: Vec<Vec3>,
}

impl<'a, Scn: Scene, Tgt: RenderTarget> Renderer<'a, Scn, Tgt>
where
    Scn: Sync + Send + 'static,
{
    pub fn new(
        samples_per_pixel: usize,
        max_depth: usize,
        ambient_grad: (Vec3, Vec3),
        num_threads: usize,
        scene: Arc<Scn>,
        camera: &'a Camera,
        target: &'a mut Tgt,
    ) -> Self {
        let num_pixels = target.width() * target.height();

        Self {
            samples_per_pixel,
            max_depth,
            ambient_grad,
            num_threads,
            scene,
            camera,
            target,

            // iteration
            jobs: JobRunner::new(num_threads),
            cur_y: 0,
            cur_samples: 0,
            accum_colors: vec![Vec3::zero(); num_pixels],
        }
    }

    pub fn target_mut(&mut self) -> &mut Tgt {
        &mut self.target
    }

    pub fn next(&mut self) -> RenderProgress {
        if self.cur_samples == self.samples_per_pixel {
            return Done;
        }

        render_next(
            self.target.height() - self.cur_y - 1, // iterate lines top-down for better preview
            self.cur_samples,
            &mut self.accum_colors,
            self.ambient_grad,
            self.max_depth,
            &mut self.jobs,
            self.camera,
            Arc::clone(&self.scene),
            self.target,
        );

        self.cur_y += 1;

        if self.cur_y == self.target.height() {
            self.cur_y = 0;
            self.cur_samples += 1;
        }

        let cur_iters = self.cur_samples * self.target.height() + self.cur_y;
        let total_iters = self.samples_per_pixel * self.target.height();
        let ratio = cur_iters as f64 / total_iters as f64;
        RenderProgress::InProgress((ratio * 100.0).round() as usize)
    }
}

fn render_next<Scene, Target>(
    cur_y: usize,
    cur_samples: usize,
    accum_colors: &mut [Vec3],
    ambient_grad: (Vec3, Vec3),
    max_depth: usize,
    jobs: &mut JobRunner<JobResult>,
    camera: &Camera,
    scene: Arc<Scene>,
    target: &mut Target,
) where
    Scene: HitRay,
    Scene: Sync + Send + 'static,
    Target: RenderTarget,
{
    let u_last = (target.width() - 1) as f64;
    let v_last = (target.height() - 1) as f64;

    for x in 0..target.width() {
        let u = (x as f64 + random()).inv_lerp(0.0, u_last);
        let v = (cur_y as f64 + random()).inv_lerp(0.0, v_last);
        let ray = camera.get_ray(u, v);

        let scene = Arc::clone(&scene);

        jobs.add_job(move || {
            let color = ray_color(&ray, &ambient_grad, &*scene, max_depth as i32);

            JobResult { x, y: cur_y, color }
        });
    }

    while let Some(r) = jobs.get_result() {
        let accum_color = &mut accum_colors[r.y * target.width() + r.x];
        *accum_color += r.color;

        let color = (*accum_color / (cur_samples + 1) as f64).sqrt();
        target.set_pixel(r.x, r.y, color);
    }
}

fn ray_color(ray: &Ray, ambient_grad: &(Vec3, Vec3), scene: &impl HitRay, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some((hit, mat)) = scene.hit(ray, 0.001, f64::MAX) {
        use ScatterResult::*;
        return match mat.scatter(&ray, &hit) {
            Scatter { scatter, color } => {
                color * ray_color(&scatter, ambient_grad, scene, depth - 1)
            }
            Glow { color } => color,
            None => Vec3::zero(),
        };
    }

    let norm_dir = ray.dir.norm();
    let t = 0.5 * (norm_dir.y + 1.0);
    t.lerp(ambient_grad.0, ambient_grad.1)
}

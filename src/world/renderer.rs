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

pub struct RenderIter<'re, 'tgt, Scn, Tgt> {
    rend: &'re mut Renderer<'tgt, Scn, Tgt>,
    cur_y: usize,
    cur_samples: usize,
    accum_colors: Vec<Vec3>,
}

impl<'re, 'tgt, Scn, Tgt> RenderIter<'re, 'tgt, Scn, Tgt>
where
    Scn: Scene,
    Scn: Sync + Send + 'static,
    Tgt: RenderTarget,
{
    fn new(rend: &'re mut Renderer<'tgt, Scn, Tgt>) -> Self {
        let target = &rend.target;
        let num_pixels = target.width() * target.height();

        Self {
            rend,
            cur_y: 0,
            cur_samples: 0,
            accum_colors: vec![Vec3::zero(); num_pixels],
        }
    }

    pub fn next(&mut self) -> RenderProgress {
        let samples_per_pixel = self.rend.samples_per_pixel;
        let target_height = self.rend.target.height();

        if self.cur_samples == samples_per_pixel {
            return Done;
        }

        render_next(
            target_height - self.cur_y - 1, // iterate lines top-down for better preview
            self.cur_samples,
            &mut self.accum_colors,
            self.rend.ambient_grad,
            self.rend.max_depth,
            &mut self.rend.jobs,
            self.rend.camera,
            Arc::clone(&self.rend.scene),
            self.rend.target,
        );

        self.cur_y += 1;

        if self.cur_y == target_height {
            self.cur_y = 0;
            self.cur_samples += 1;
        }

        let cur_iters = self.cur_samples * target_height + self.cur_y;
        let total_iters = samples_per_pixel * target_height;
        let ratio = cur_iters as f64 / total_iters as f64;
        RenderProgress::InProgress((ratio * 100.0).round() as usize)
    }

    pub fn target_mut(&mut self) -> &mut Tgt {
        self.rend.target_mut()
    }
}

struct JobResult {
    x: usize,
    y: usize,
    color: Vec3,
}

pub struct Renderer<'tgt, Scene, Target> {
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pub scene: Arc<Scene>,
    pub camera: &'tgt Camera,
    pub target: &'tgt mut Target,
    pub num_threads: usize,

    pub ambient_grad: (Vec3, Vec3),

    // iteration data
    jobs: JobRunner<JobResult>,
}

impl<'tgt, Scn: Scene, Tgt: RenderTarget> Renderer<'tgt, Scn, Tgt>
where
    Scn: Sync + Send + 'static,
{
    pub fn new(
        samples_per_pixel: usize,
        max_depth: usize,
        ambient_grad: (Vec3, Vec3),
        num_threads: usize,
        scene: Arc<Scn>,
        camera: &'tgt Camera,
        target: &'tgt mut Tgt,
    ) -> Self {
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
        }
    }

    pub fn target_mut(&mut self) -> &mut Tgt {
        &mut self.target
    }

    pub fn iter<'re>(&'re mut self) -> RenderIter<'re, 'tgt, Scn, Tgt> {
        RenderIter::new(self)
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

use crate::bitmap::Bitmap;
use crate::math::Vec3;
use crate::utils::{Parser, ParserPlugin};
use crate::world::{Camera, RenderTarget, Renderer, Scene};
use serde::Deserialize;
use std::cell::Cell;
use std::sync::Arc;

pub struct RendererLoader {
    image_size: Cell<[usize; 2]>,
    samples: Cell<usize>,
    num_threads: Cell<usize>,
    max_depth: Cell<usize>,
    ambient_grad: Cell<(Vec3, Vec3)>,
}

impl RendererLoader {
    pub fn new() -> Self {
        Self {
            image_size: Cell::new([640, 480]),
            samples: Cell::new(100),
            max_depth: Cell::new(50),
            num_threads: Cell::new(num_cpus::get()),
            ambient_grad: Cell::new((Vec3::new(1, 1, 1), Vec3::new(0.5, 0.7, 1.0))),
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        let [width, height] = self.image_size.get();
        width as f64 / height as f64
    }

    pub fn new_image(&self) -> Bitmap {
        let [width, height] = self.image_size.get();
        Bitmap::new(width, height, (0, 0, 0))
    }

    pub fn new_renderer<'a, Scn: Scene, Tgt: RenderTarget>(
        &self,
        scene: Arc<Scn>,
        camera: &'a Camera,
        target: &'a mut Tgt,
    ) -> Renderer<'a, Scn, Tgt>
    where
        Scn: Sync + Send + 'static,
    {
        Renderer::new(
            self.samples.get(),
            self.max_depth.get(),
            self.ambient_grad.get(),
            self.num_threads.get(),
            scene,
            camera,
            target,
        )
    }
}

#[derive(Deserialize)]
struct AmbientConfig {
    from: (f64, f64, f64),
    to: (f64, f64, f64),
}

impl<'a> ParserPlugin<'a> for RendererLoader {
    fn init(&'a self, parser: &mut Parser<'a>) {
        parser.add_cmd("image", move |data| {
            let data: [usize; 2] = data;

            self.image_size.set(data);

            Ok(())
        });

        parser.add_cmd("samples", move |data| {
            let data: usize = data;

            self.samples.set(data);

            Ok(())
        });

        parser.add_cmd("ambient", move |data| {
            let data: AmbientConfig = data;

            self.ambient_grad.set((data.from.into(), data.to.into()));

            Ok(())
        });
    }
}

use crate::bitmap::Bitmap;
use crate::math::HitRay;
use crate::utils::{Parser, ParserPlugin};
use crate::world::{Camera, RenderTarget, Renderer};
use std::cell::Cell;
use std::sync::Arc;

pub struct RendererLoader {
    image_size: Cell<[usize; 2]>,
    samples: Cell<usize>,
    num_threads: Cell<usize>,
    max_depth: Cell<usize>,
}

impl RendererLoader {
    pub fn new() -> Self {
        Self {
            image_size: Cell::new([640, 480]),
            samples: Cell::new(100),
            max_depth: Cell::new(50),
            num_threads: Cell::new(num_cpus::get()),
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

    pub fn new_renderer<'a, Scene, Target>(
        &self,
        scene: Arc<Scene>,
        camera: &'a Camera,
        target: &'a mut Target,
    ) -> Renderer<'a, Scene, Target>
    where
        Scene: HitRay,
        Scene: Sync + Send + 'static,
        Target: RenderTarget,
    {
        Renderer::new(
            self.samples.get(),
            self.max_depth.get(),
            self.num_threads.get(),
            scene,
            camera,
            target,
        )
    }
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
    }
}

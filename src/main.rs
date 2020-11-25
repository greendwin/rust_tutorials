// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::time::Instant;

use rust_ray::utils::*;
use rust_ray::world::*;

use RenderProgress::*;

// const SCENE_YAML: &str = include_str!("../shadow_scene.yaml");
const SCENE_YAML: &str = include_str!("../random_shadows.yaml");

fn main() {
    let start_time = Instant::now();

    let renderer_loader = RendererLoader::new();
    let camera_loader = CameraLoader::new();
    let scene_loader = SceneLoader::new();
    let rnd_gen_loader = RndGenLoader::new(&scene_loader);

    let mut parser = Parser::new();
    parser.register(&renderer_loader);
    parser.register(&camera_loader);
    parser.register(&scene_loader);
    parser.register(&rnd_gen_loader);
    parser.parse(SCENE_YAML).expect("load scenario");

    let scene = scene_loader.new_scene();
    let camera = camera_loader.new_camera(renderer_loader.aspect_ratio());
    let mut image = renderer_loader.new_image();
    println!("image: {}, {}", image.width(), image.height());

    let mut renderer = renderer_loader.new_renderer(scene.into(), &camera, &mut image);
    println!("num_threads: {}", renderer.num_threads);
    println!("samples: {}", renderer.samples_per_pixel);
    println!("max_depth: {}", renderer.max_depth);
    println!("-----");

    let mut prev_progress = 0;
    let mut prev_save = start_time;
    let mut iter = renderer.iter();
    while let InProgress(progress) = iter.next() {
        let cur_time = Instant::now();

        if prev_progress != progress {
            let since_start = cur_time.duration_since(start_time);

            let estimate_sec =
                (100 - progress) as f64 / progress as f64 * since_start.as_secs_f64();

            if estimate_sec > 60.0 {
                println!("{}%, est {:.2} min", progress, estimate_sec / 60.0);
            } else {
                println!("{}%, est {:.0} sec", progress, estimate_sec);
            }
            prev_progress = progress;
        }

        // flush intermediate results each 5 seconds
        let since_last_save = cur_time.duration_since(prev_save);
        if since_last_save.as_secs() >= 5 {
            iter.target_mut().save("output.bmp").expect("save file");
            prev_save = cur_time;
        }
    }

    // save final result
    image.save("output.bmp").expect("save file");

    let total_time = Instant::now().duration_since(start_time);
    println!(
        "total time: {:.1} min ({} sec)",
        total_time.as_secs_f32() / 60.0,
        total_time.as_secs()
    );

    println!("done.");
}

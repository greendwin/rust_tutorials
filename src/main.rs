// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::time::Instant;

use rust_ray::math::*;
use rust_ray::utils::*;
use rust_ray::world::*;

use RenderProgress::*;

const SCENE_YAML: &str = include_str!("../random_scene.yaml");

type SomeScene = Scene<SomeObject>;

fn random_gen(
    scene: &mut SomeScene,
    range: i32,
    step: f64,
    radius: f64,
    dmg_weights: (f64, f64, f64),
) {
    let (diff_weight, metal_weight, glass_weight) = dmg_weights;
    let total_weight = diff_weight + metal_weight + glass_weight;

    for a in -range..range {
        for b in -range..range {
            let choose_mat = rand_range(0, total_weight);
            let center = Vec3::new(
                step * a as f64 + (step - radius) * random(),
                radius - 0.01,
                step * b as f64 + (step - radius) * random(),
            );

            if (center - Vec3::new(4, radius, 0)).length() <= 0.9 {
                continue;
            }

            if choose_mat < diff_weight {
                let albedo = rand_vec3(0, 1) * rand_vec3(0, 1);
                let mat = DiffuseMat::new(albedo);
                scene.add(SphereObject::new(center, radius, mat));
            } else if choose_mat < diff_weight + metal_weight {
                let albedo = rand_vec3(0.5, 1);
                let fuzz = rand_range(0, 0.5);
                let mat = MetalMat::new(albedo, fuzz);
                scene.add(SphereObject::new(center, radius, mat));
            } else {
                let mat = DielectricMat::new(2.0);
                scene.add(SphereObject::new(center, radius, mat));
                scene.add(SphereObject::new(center, -0.9 * radius, mat));
            };
        }
    }
}

fn random_scene(loader: &SceneLoader) -> SomeScene {
    let mut scene = loader.new_scene();

    random_gen(&mut scene, 11, 1.0, 0.18, (7.0, 2.0, 1.0));
    random_gen(&mut scene, 20, 0.5, 0.05, (5.0, 3.0, 2.0));

    scene
}

fn main() {
    let start_time = Instant::now();

    let renderer_loader = RendererLoader::new();
    let camera_loader = CameraLoader::new();
    let scene_loader = SceneLoader::new();

    let mut parser = Parser::new();
    parser.register(&renderer_loader);
    parser.register(&camera_loader);
    parser.register(&scene_loader);
    parser.parse(SCENE_YAML).expect("load scenario");

    let scene = random_scene(&scene_loader);
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
    while let InProgress(progress) = renderer.next() {
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
            renderer.target_mut().save("output.bmp").expect("save file");
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

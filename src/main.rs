// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use rust_ray::math::*;
use rust_ray::utils::*;
use rust_ray::world::*;

// const SCENE_DECL: &str = include_str!("../scene_example.txt");
const SCENE_DECL: &str = include_str!("../random_scene.txt");

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
                step * (a as f64 + (1.0 - radius) * random()),
                radius - 0.1,
                step * (b as f64 + (1.0 - radius) * random()),
            );

            if (center - Vec3::new(4, radius, 0)).length() <= 0.9 {
                continue;
            }

            if choose_mat < diff_weight {
                let albedo = rand_vec3(0, 1) * rand_vec3(0, 1);
                let mat = DiffuseMat::new(albedo);
                scene.add(Sphere::new(center, radius, mat));
            } else if choose_mat < diff_weight + metal_weight {
                let albedo = rand_vec3(0.5, 1);
                let fuzz = rand_range(0, 0.5);
                let mat = MetalMat::new(albedo, fuzz);
                scene.add(Sphere::new(center, radius, mat));
            } else {
                let mat = DielectricMat::new(2.0);
                scene.add(Sphere::new(center, radius, mat));
                scene.add(Sphere::new(center, -0.9 * radius, mat));
            };
        }
    }
}

fn random_scene(loader: &Loader) -> SomeScene {
    let mut scene = loader.new_scene();

    random_gen(&mut scene, 10, 1.0, 0.2, (7.0, 2.0, 1.0));
    random_gen(&mut scene, 20, 0.5, 0.05, (5.0, 3.0, 2.0));

    return scene;
}

fn main() {
    let loader = Loader::from_str(SCENE_DECL).expect("load scenario");

    let mut image = loader.new_image();
    let camera = loader.new_camera();
    let rend = loader.new_renderer();

    let scene = random_scene(&loader);

    rend.render(&camera, &scene, &mut image);

    println!("saving 'output.bmp'...");
    image.save("output.bmp").expect("save file");

    println!("done.");
}

// TODO:
//   render iteration:
//      - move logging out to iterator
//      - print total time
//      - save intermediate results for preview
//      - randomize pixels order for better preview
//   rendering performance:
//      - render pixels multiple threads
//      - add voxels for objects collection

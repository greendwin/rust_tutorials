// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use rust_ray::math::*;
use rust_ray::utils::*;
use rust_ray::world::*;

// const SCENE_DECL: &str = include_str!("../scene_example.txt");
const SCENE_DECL: &str = include_str!("../random_scene.txt");

type SomeScene = Scene<SomeObject>;

fn random_scene(loader: &Loader) -> SomeScene {
    let mut r = loader.new_scene();

    for a in -11i32..11 {
        for b in -11i32..11 {
            let choose_mat = random();
            let center = Vec3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Vec3::new(4, 0.2, 0)).length() <= 0.9 {
                continue;
            }

            let sphere_material: SomeMaterial = if choose_mat < 0.8 {
                let albedo = rand_vec3(0, 1) * rand_vec3(0, 1);
                DiffuseMat::new(albedo).into()
            } else if choose_mat < 0.95 {
                let albedo = rand_vec3(0.5, 1);
                let fuzz = rand_range(0, 0.5);
                MetalMat::new(albedo, fuzz).into()
            } else {
                DielectricMat::new(1.5).into()
            };

            r.add(Sphere::new(center, 0.2, sphere_material).into());
        }
    }

    return r;
}

fn main() {
    let loader = Loader::from_str(SCENE_DECL).expect("load scenario");

    let mut image = loader.new_image();
    let camera = loader.new_camera();
    let rend = loader.new_renderer();
    // let scene = loader.new_scene();

    let scene = random_scene(&loader);

    rend.render(&camera, &scene, &mut image);

    println!("saving 'output.bmp'...");
    image.save("output.bmp").expect("save file");

    println!("done.");
}

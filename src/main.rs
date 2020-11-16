// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use rust_ray::world::*;

const SCENE_DECL: &str = "
    # image
    IMG 640 480

    # camera
    CAM_POS: [-2, 2, 1]
    CAM_LOOKAT: [-0.2, 0, -1]
    CAM_FOV: 45

    # materials
    MAT_DIFF ground (0.8, 0.8, 0)
    MAT_DIFF baloon (0.1, 0.2, 0.5)
    MAT_DI water 1.5
    MAT_METAL metal (0.8, 0.6, 0.2) 0.2

    # objects
    SPHERE ground (0, -100.5, -1), 100
    SPHERE baloon (0, 0, -1), 0.5
    SPHERE water (-1, 0, -1), 0.5
    SPHERE water (-1, 0, -1), -0.45
    SPHERE metal (1, 0, -1), 0.5
";

fn main() {
    let loader = Loader::from_str(SCENE_DECL).expect("load scenario");

    let mut image = loader.new_image();
    let camera = loader.new_camera();
    let rend = loader.new_renderer();
    let scene = loader.new_scene();

    rend.render(&camera, &scene, &mut image);

    println!("saving 'output.bmp'...");
    image.save("output.bmp").expect("save file");

    println!("done.");
}

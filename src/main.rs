// url: https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::error::Error;

use rust_ray::math::*;
use rust_ray::world::*;

const SCENE_DECL: &str = "
    # image
    IMG 640 480
    SAMPLES: 100

    # camera
    CAM_POS: [-2, 2, 1]
    CAM_LOOKAT: [-0.2, 0, -1]
    CAM_FOV: 45

    # materials
    MAT_DIFF ground (0.8, 0.8, 0)
    MAT_DIFF center (0.1, 0.2, 0.5)
    MAT_DI left 1.5
    MAT_METAL right (0.8, 0.6, 0.2) 0.2
";

fn main() -> Result<(), Box<dyn Error>> {
    let loader = Loader::from_str(SCENE_DECL)?;

    // World

    let mut scene = Scene::new();
    scene.add(Sphere::new((0, -100.5, -1), 100, loader.get_mat("ground")));
    scene.add(Sphere::new((0, 0, -1), 0.5, loader.get_mat("center")));
    scene.add(Sphere::new((-1, 0, -1), 0.5, loader.get_mat("left")));
    scene.add(Sphere::new((-1, 0, -1), -0.45, loader.get_mat("left")));
    scene.add(Sphere::new((1, 0, -1), 0.5, loader.get_mat("right")));

    // Render

    let camera = loader.new_camera();
    let mut image = loader.new_image();
    let rend = loader.new_renderer();

    rend.render(&camera, &scene, &mut image);

    // Save

    println!("saving 'output.bmp'...");
    image.save("output.bmp")?;

    println!("done.");
    Ok(())
}

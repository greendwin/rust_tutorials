use rust_ray::math::Vec3;
use rust_ray::world::{Loader, LoaderError, SomeMaterial, SomeObject};

use LoaderError::*;

#[test]
fn setup_image() {
    let mut loader = Loader::new();
    loader.parse("IMG=320, 240").expect("no errors");

    assert_eq!(320, loader.image_width());
    assert_eq!(240, loader.image_height());
}

#[test]
fn setup_samples() {
    let mut loader = Loader::new();
    loader.parse("SAMPLES=42").expect("no errors");

    assert_eq!(42, loader.samples_per_pixel());
}

#[test]
fn setup_max_depth() {
    let mut loader = Loader::new();
    loader.parse("MAX_DEPTH=42").expect("no errors");

    assert_eq!(42, loader.max_depth());
}

#[test]
fn support_comments() {
    let mut loader = Loader::new();
    loader
        .parse(
            "# just a comment\n\
                 # that should be skipped\n\
                 \n\
                 # empty lines should be skipped too",
        )
        .expect("no errors");
}

#[test]
fn cam_props() {
    let text = "\
            CAM_POS=(-2, 2, 1)\n\
            CAM_LOOKAT=(-0.1, 0, -1)\n\
            CAM_UP=(0, 1, 0)\n\
            FOV=45\n\
        ";

    let loader = Loader::from_str(text).expect("no errors");

    assert_eq!(Vec3::new(-2, 2, 1), loader.cam_pos());
    assert_eq!(Vec3::new(-0.1, 0, -1), loader.cam_lookat());
    assert_eq!(Vec3::new(0, 1, 0), loader.cam_up());
    assert_eq!(45.0, loader.cam_fov());
}

#[test]
fn cam_aperture() {
    let text = "
            APERTURE=2.0
        ";

    let loader = Loader::from_str(text).expect("no errors");

    assert_eq!(loader.cam_aperture(), 2.0);
}

#[test]
fn cam_focus() {
    let text = "
            FOCUS=42
        ";

    let loader = Loader::from_str(text).expect("no errors");

    assert_eq!(loader.cam_focus(), 42.0);
}

#[test]
fn create_diff_material() {
    let text = "
            MAT_DIFF white 1 2 3
        ";

    let loader = Loader::from_str(text).expect("no errors");

    let mat = match loader.get_mat("white") {
        Some(SomeMaterial::Diff(m)) => m,
        mat => panic!("wrong material {:?}", mat),
    };

    assert_eq!(mat.albedo, Vec3::new(1, 2, 3));
}

#[test]
fn create_dielectric_material() {
    let text = "
            MAT_DI water 1.5
        ";

    let loader = Loader::from_str(text).expect("no errors");

    let mat = match loader.get_mat("water") {
        Some(SomeMaterial::Di(m)) => m,
        mat => panic!("wrong material {:?}", mat),
    };

    assert_eq!(mat.ir, 1.5);
}

#[test]
fn create_metal_material() {
    let text = "
            MAT_METAL block (0.2, 0.3, 0.4) 0.5
        ";

    let loader = Loader::from_str(text).expect("no errors");

    let mat = match loader.get_mat("block") {
        Some(SomeMaterial::Metal(m)) => m,
        mat => panic!("wrong material {:?}", mat),
    };

    assert_eq!(mat.albedo, Vec3::new(0.2, 0.3, 0.4));
    assert_eq!(mat.fuzz, 0.5);
}

#[test]
fn create_obj() {
    let text = "
            MAT_METAL block (0.2, 0.3, 0.4) 0.5
            SPHERE block (1, 2, 3) 10
        ";

    let loader = Loader::from_str(text).expect("no errors");

    assert_eq!(1, loader.get_objs().len());

    let sphere = match &loader.get_objs()[0] {
        SomeObject::Sphere(sph) => sph,
    };

    assert_eq!(sphere.sphere.center, Vec3::new(1, 2, 3));
    assert_eq!(sphere.sphere.radius, 10.0);

    let mat = match &sphere.material {
        SomeMaterial::Metal(m) => m,
        _ => panic!("unexpected material type {:?}", sphere.material),
    };

    assert_eq!(mat.albedo, Vec3::new(0.2, 0.3, 0.4));
    assert_eq!(mat.fuzz, 0.5);
}

#[test]
fn unknown_material() {
    let text = "
            SPHERE unkn (1, 2, 3) 10
        ";

    let err = Loader::from_str(text).expect_err("must fail on unkown error");

    match err {
        SyntaxError { line, msg } => {
            assert_eq!(line, 2);
            assert!(msg.contains("'unkn'"));
            assert!(msg.contains("unknown material"));
        }
    }
}

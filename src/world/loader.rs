use std::collections::HashMap;
use std::sync::Arc;

use crate::bitmap::Bitmap;
use crate::math::{HitRay, Vec3};

use super::camera::Camera;
use super::loader_error::*;
use super::materials::*;
use super::renderer::{RenderTarget, Renderer};
use super::scene::Scene;
use super::some_object::SomeObject;
use super::sphere_object::SphereObject;

use LoaderError::*;

#[derive(Debug)]
pub struct Loader {
    // image
    image_size: Option<(usize, usize)>,
    samples_per_pixel: Option<usize>,
    max_depth: Option<usize>,

    // camera
    cam_pos: Option<Vec3>,
    cam_lookat: Option<Vec3>,
    cam_up: Option<Vec3>,
    cam_fov: Option<f64>,
    cam_aperture: Option<f64>,
    cam_focus: Option<f64>,

    // world
    materials: HashMap<String, SomeMaterial>,
    objs: Vec<SomeObject>,
}

macro_rules! num_args {
    () => {
        0
    };
    ($t:tt) => {
        1
    };
    ($t:tt, $($ts:tt),*) => {
        (1 + num_args!($($ts),*))
    };
}

macro_rules! parse_args {
    ($command:expr, $line:expr, ($($tp:tt),+)) => {{
        let mut it = $command[1..].iter();
        if $command.len() != 1 + num_args!($($tp),+) {
            return Err(SyntaxError {
                msg: format!(
                    "{}: wrong arguments count, expected ({})",
                    $command[0],
                    stringify!($($tp),+)
                ),
                line: $line,
            });
        }

        ($(parse_args!($line; @next it, $tp)),+)
    }};
    ($line:expr; @next $it:expr, str) => {
        $it.next().unwrap()
    };
    ($line:expr; @next $it:expr, $tp:tt) => {
        $it.next().unwrap().parse::<$tp>().with_context($line)?
    };
}

impl Loader {
    pub fn new() -> Self {
        Self {
            // image
            image_size: None,
            samples_per_pixel: None,
            max_depth: None,

            // camera
            cam_pos: None,
            cam_lookat: None,
            cam_up: None,
            cam_fov: None,
            cam_aperture: None,
            cam_focus: None,

            // world
            materials: HashMap::new(),
            objs: Vec::new(),
        }
    }

    pub fn image_width(&self) -> usize {
        self.image_size.map_or(640, |s| s.0)
    }

    pub fn image_height(&self) -> usize {
        self.image_size.map_or(480, |s| s.1)
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.image_width() as f64 / self.image_height() as f64
    }

    pub fn samples_per_pixel(&self) -> usize {
        self.samples_per_pixel.unwrap_or(100)
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth.unwrap_or(50)
    }

    pub fn cam_pos(&self) -> Vec3 {
        self.cam_pos.unwrap_or(Vec3::new(0, 0, 0))
    }

    pub fn cam_lookat(&self) -> Vec3 {
        self.cam_lookat.unwrap_or(Vec3::new(0, 0, -1))
    }

    pub fn cam_up(&self) -> Vec3 {
        self.cam_up.unwrap_or(Vec3::new(0, 1, 0))
    }

    pub fn cam_fov(&self) -> f64 {
        self.cam_fov.unwrap_or(90.0)
    }

    pub fn cam_aperture(&self) -> f64 {
        self.cam_aperture.unwrap_or(0.0)
    }

    pub fn cam_focus(&self) -> f64 {
        self.cam_focus
            .unwrap_or((self.cam_lookat() - self.cam_pos()).length())
    }

    pub fn get_mat(&self, name: &str) -> Option<&SomeMaterial> {
        self.materials.get(name)
    }

    pub fn new_image(&self) -> Bitmap {
        Bitmap::new(self.image_width(), self.image_height(), (0, 0, 0))
    }

    pub fn new_camera(&self) -> Camera {
        Camera::new(
            self.cam_pos(),
            self.cam_lookat(),
            self.cam_up(),
            self.cam_fov(),
            self.aspect_ratio(),
            self.cam_aperture(),
            self.cam_focus(),
        )
    }

    pub fn get_objs(&self) -> &[SomeObject] {
        &self.objs
    }

    pub fn new_scene(&self) -> Scene<SomeObject> {
        let mut scene = Scene::new();

        for obj in &self.objs {
            scene.add(obj.clone());
        }

        scene
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
            self.samples_per_pixel(),
            self.max_depth(),
            scene,
            camera,
            target,
        )
    }

    pub fn from_str(text: &str) -> LoaderResult<Self> {
        let mut loader = Self::new();
        loader.parse(text)?;
        Ok(loader)
    }

    pub fn parse(&mut self, text: &str) -> LoaderResult<()> {
        for (idx, s) in text.split("\n").enumerate() {
            let line = idx + 1;
            let command = parse_command(s);
            if command.is_empty() {
                continue;
            }

            match &*command[0] {
                "IMG" => {
                    self.image_size
                        .replace(parse_args!(command, line, (usize, usize)));
                }
                "SAMPLES" => {
                    self.samples_per_pixel
                        .replace(parse_args!(command, line, (usize)));
                }
                "MAX_DEPTH" => {
                    self.max_depth.replace(parse_args!(command, line, (usize)));
                }
                "CAM_POS" => {
                    self.cam_pos
                        .replace(parse_args!(command, line, (f64, f64, f64)).into());
                }
                "CAM_LOOKAT" => {
                    self.cam_lookat
                        .replace(parse_args!(command, line, (f64, f64, f64)).into());
                }
                "CAM_UP" => {
                    self.cam_up
                        .replace(parse_args!(command, line, (f64, f64, f64)).into());
                }
                "FOV" => {
                    self.cam_fov.replace(parse_args!(command, line, (f64)));
                }
                "APERTURE" => {
                    self.cam_aperture.replace(parse_args!(command, line, (f64)));
                }
                "FOCUS" => {
                    self.cam_focus.replace(parse_args!(command, line, (f64)));
                }
                "MAT_DIFF" => {
                    let (name, r, g, b) = parse_args!(command, line, (str, f64, f64, f64));
                    self.materials
                        .insert(name.clone(), DiffuseMat::new((r, g, b)).into());
                }
                "MAT_DI" => {
                    let (name, index_of_refraction) = parse_args!(command, line, (str, f64));
                    self.materials
                        .insert(name.clone(), DielectricMat::new(index_of_refraction).into());
                }
                "MAT_METAL" => {
                    let (name, r, g, b, fuzz) =
                        parse_args!(command, line, (str, f64, f64, f64, f64));
                    self.materials
                        .insert(name.clone(), MetalMat::new((r, g, b), fuzz).into());
                }
                "SPHERE" => {
                    let (name, x, y, z, radius) =
                        parse_args!(command, line, (str, f64, f64, f64, f64));
                    let mat = match self.get_mat(name) {
                        Some(m) => m.clone(),
                        None => {
                            return Err(SyntaxError {
                                msg: format!("unknown material '{}'", name),
                                line,
                            })
                        }
                    };

                    self.objs
                        .push(SphereObject::new((x, y, z), radius, mat).into());
                }
                _ => {
                    return Err(SyntaxError {
                        msg: format!("unknown command '{}'", command[0]),
                        line,
                    });
                }
            }
        }

        Ok(())
    }
}

fn parse_command(mut text: &str) -> Vec<String> {
    text = text.trim(); // remove white spaces

    if text.starts_with("#") {
        return Vec::new(); // skip comments
    }

    let mut r = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '(' | ')' | '[' | ']' | ':' | '=' | ',' => {
                r.push(' ');
            }
            _ => r.push(ch),
        }
    }

    r.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

use super::camera::Camera;
use super::loader_error::{LoaderError, LoaderResult, SyntaxContext};
use super::renderer::Renderer;
use crate::bitmap::Bitmap;
use crate::math::Vec3;

use LoaderError::*;

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
}

impl Loader {
    pub fn new() -> Self {
        Self {
            image_size: None,
            samples_per_pixel: None,
            max_depth: None,
            cam_pos: None,
            cam_lookat: None,
            cam_up: None,
            cam_fov: None,
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
        )
    }

    pub fn new_renderer(&self) -> Renderer {
        Renderer::new(self.samples_per_pixel(), self.max_depth())
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

            let req_args = vec![
                ("IMG", 2, "two args: width and height"),
                ("SAMPLES", 1, "one number"),
                ("MAX_DEPTH", 1, "one number"),
                ("CAM_POS", 3, "three numbers: x, y, z"),
                ("CAM_LOOKAT", 3, "three numbers: x, y, z"),
                ("CAM_UP", 3, "three numbers: x, y, z"),
                ("CAM_FOV", 1, "one number"),
            ];

            check_num_args(line, &command, &req_args)?;

            macro_rules! parse_args {
                ($($tp:tt),+) =>{
                    parse_args!(@offset 1usize; $($tp),+)
                };
                (@offset $off:expr; $tp:tt) => {
                    command[$off].parse::<$tp>().with_context(line, &command)?
                };
                (@offset $off:expr; $t1:tt, $t2:tt) => {
                    (
                        parse_args!(@offset $off; $t1),
                        parse_args!(@offset $off + 1; $t2),
                    )
                };
                (@offset $off:expr; $t1:tt, $t2:tt, $t3:tt) => {
                    (
                        parse_args!(@offset $off; $t1),
                        parse_args!(@offset $off + 1; $t2),
                        parse_args!(@offset $off + 2; $t3),
                    )
                };
            }

            match &*command[0] {
                "IMG" => {
                    self.image_size.replace(parse_args!(usize, usize));
                }
                "SAMPLES" => {
                    self.samples_per_pixel.replace(parse_args!(usize));
                }
                "MAX_DEPTH" => {
                    self.max_depth.replace(parse_args!(usize));
                }
                "CAM_POS" => {
                    self.cam_pos.replace(parse_args!(f64, f64, f64).into());
                }
                "CAM_LOOKAT" => {
                    self.cam_lookat.replace(parse_args!(f64, f64, f64).into());
                }
                "CAM_UP" => {
                    self.cam_up.replace(parse_args!(f64, f64, f64).into());
                }
                "CAM_FOV" => {
                    self.cam_fov.replace(parse_args!(f64));
                }
                _ => panic!("uncovered command: {:?}", command),
            }
        }

        Ok(())
    }
}

fn check_num_args(
    line: usize,
    command: &[String],
    check_config: &[(&str, usize, &str)],
) -> LoaderResult<()> {
    for &(cmd, num_args, require_str) in check_config {
        if command[0] == cmd {
            if command.len() != 1 + num_args {
                return Err(SyntaxError {
                    msg: format!("{}: require {}", cmd, require_str),
                    line,
                    command: command.to_owned(),
                });
            }

            return Ok(());
        }
    }

    let mut available_commands: Vec<String> = check_config
        .iter()
        .map(|&(cmd, _, _)| cmd.to_owned())
        .collect();
    available_commands.sort_unstable();

    return Err(SyntaxError {
        msg: format!(
            "unknown command '{}', available commands: {:?}",
            command[0], available_commands
        ),
        line,
        command: command.to_owned(),
    });
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

#[cfg(test)]
mod test {
    use super::*;

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
            CAM_FOV=45\n\
        ";

        let loader = Loader::from_str(text).expect("no errors");

        assert_eq!(Vec3::new(-2, 2, 1), loader.cam_pos());
        assert_eq!(Vec3::new(-0.1, 0, -1), loader.cam_lookat());
        assert_eq!(Vec3::new(0, 1, 0), loader.cam_up());
        assert_eq!(45.0, loader.cam_fov());
    }
}

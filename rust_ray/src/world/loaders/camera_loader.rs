use crate::utils::{Parser, ParserPlugin};
use crate::world::Camera;
use serde::Deserialize;
use std::cell::RefCell;

// TODO: make derive macro Update for optional fields

#[derive(Deserialize)]
struct CameraConfig {
    pos: Option<(f64, f64, f64)>,
    lookat: Option<(f64, f64, f64)>,
    up: Option<(f64, f64, f64)>,
    fov: Option<f64>,
    focus: Option<f64>,
    aperture: Option<f64>,
}

pub struct CameraLoader {
    config: RefCell<CameraConfig>,
}

impl CameraLoader {
    pub fn new() -> Self {
        Self {
            config: RefCell::new(CameraConfig {
                pos: Some((0.0, 0.0, 0.0)),
                lookat: Some((0.0, 0.0, -1.0)),
                up: Some((0.0, 1.0, 0.0)),
                fov: Some(90.0),
                focus: Some(1.0),
                aperture: Some(0.0),
            }),
        }
    }

    pub fn new_camera(&self, aspect_ratio: f64) -> Camera {
        let cfg = self.config.borrow();

        Camera::new(
            cfg.pos.unwrap().into(),
            cfg.lookat.unwrap().into(),
            cfg.up.unwrap().into(),
            cfg.fov.unwrap(),
            aspect_ratio,
            cfg.aperture.unwrap(),
            cfg.focus.unwrap(),
        )
    }
}

impl<'a> ParserPlugin<'a> for CameraLoader {
    fn init(&'a self, parser: &mut Parser<'a>) {
        parser.add_cmd("camera", move |data| {
            let data: CameraConfig = data;

            let mut cfg = self.config.borrow_mut();

            if data.pos.is_some() {
                cfg.pos = data.pos;
            }
            if data.lookat.is_some() {
                cfg.lookat = data.lookat;
            }
            if data.up.is_some() {
                cfg.up = data.up;
            }
            if data.fov.is_some() {
                cfg.fov = data.fov;
            }
            if data.focus.is_some() {
                cfg.focus = data.focus;
            }
            if data.aperture.is_some() {
                cfg.aperture = data.aperture;
            }

            Ok(())
        });
    }
}

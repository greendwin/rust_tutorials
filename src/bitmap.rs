use std::io;
use std::path::Path;

use crate::math::clamp_f2u8;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }
}

pub struct ColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorF {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<ColorF> for Color {
    fn from(col: ColorF) -> Self {
        Color::new(clamp_f2u8(col.r), clamp_f2u8(col.g), clamp_f2u8(col.b))
    }
}

pub struct Bitmap {
    width: usize,
    height: usize,
}

impl Bitmap {
    pub fn new(width: usize, height: usize, fill: Color) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel<T: Into<Color>>(&mut self, x: usize, y: usize, color: T) {}
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        Color::black()
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> io::Result<()> {
        Err(io::ErrorKind::NotFound.into())
    }
}

// TODO: test me!

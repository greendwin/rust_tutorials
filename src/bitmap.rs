use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};
use std::mem;
use std::path::Path;

use crate::utils::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

unsafe impl RawStruct for Color {}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn red() -> Self {
        Self::new(u8::MAX, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, u8::MAX, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, u8::MAX)
    }
}

pub struct Bitmap {
    width: usize,
    height: usize,
    data: Box<[Color]>,
}

#[repr(C, packed)]
pub struct BmpFileHeader {
    pub magic: [u8; 2],
    pub size: u32,
    pub reserved1: u16,
    pub reserved2: u16,
    pub data_offset: u32,
}

unsafe impl RawStruct for BmpFileHeader {}

impl BmpFileHeader {
    pub fn new(width: usize, height: usize, bpp: usize) -> Self {
        let data_offset = mem::size_of::<BmpFileHeader>() + mem::size_of::<BitmapInfoHeader>();
        let data_size = bmp_row_size(width, bpp) * height;

        Self {
            magic: [0x42, 0x4d],
            size: (data_offset + data_size) as u32,
            reserved1: 0,
            reserved2: 0,
            data_offset: data_offset as u32,
        }
    }
}

pub fn bmp_row_size(width: usize, bpp: usize) -> usize {
    4 * ((bpp * width + 31) / 32)
}

#[repr(C, packed)]
struct BitmapInfoHeader {
    struct_size: u32,
    width: i32,
    height: i32,
    num_color_planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    image_size: u32,
    hor_res: i32,
    vert_res: i32,
    num_colors: u32,
    num_important_colors: u32,
}

unsafe impl RawStruct for BitmapInfoHeader {}

impl BitmapInfoHeader {
    fn new(width: usize, height: usize, bpp: usize) -> Self {
        Self {
            struct_size: 40,
            width: width as i32,
            height: height as i32,
            num_color_planes: 1,
            bits_per_pixel: bpp as u16, // RGB
            compression: 0,             // BI_RGB
            image_size: 0,
            hor_res: 96,
            vert_res: 96,
            num_colors: 0,
            num_important_colors: 0,
        }
    }
}

impl Bitmap {
    pub fn new(width: usize, height: usize, fill: impl Into<Color>) -> Self {
        let mut data = Vec::new();
        data.resize(width * height, fill.into());

        Self {
            width,
            height,
            data: data.into_boxed_slice(),
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn get_offset(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn set_pixel<T: Into<Color>>(&mut self, x: usize, y: usize, color: T) {
        let offset = self.get_offset(x, y);
        self.data[offset] = color.into();
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let offset = self.get_offset(x, y);
        self.data[offset]
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> io::Result<()> {
        let f = File::create(path)?;
        let mut f = BufWriter::new(f);

        let file_header = BmpFileHeader::new(self.width, self.height, 24);
        f.write(file_header.raw_view())?;

        let bmp_info = BitmapInfoHeader::new(self.width, self.height, 24);
        f.write(bmp_info.raw_view())?;

        let row_size = bmp_row_size(self.width, 24);
        let padding = vec![0; row_size - self.width * mem::size_of::<Color>()];
        for y in 0..self.height {
            f.write(self.data[y * self.width..(y + 1) * self.width].raw_view())?;
            f.write(&padding)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let bmp = Bitmap::new(16, 8, Color::new(1, 2, 3));

        assert_eq!(bmp.width(), 16);
        assert_eq!(bmp.height(), 8);
    }

    #[test]
    fn new_fill_color() {
        let bmp = Bitmap::new(16, 16, Color::new(1, 2, 3));
        assert_eq!(bmp.get_pixel(5, 5), Color::new(1, 2, 3));
    }

    #[test]
    fn set_pixel() {
        let mut bmp = Bitmap::new(16, 16, Color::black());
        bmp.set_pixel(5, 10, Color::red());

        assert_eq!(bmp.get_pixel(5, 10), Color::red());
    }
}

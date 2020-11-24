use super::bitmap::{Bitmap, Color};
use crate::math::*;
use crate::world::RenderTarget;

fn lerp_ratio_to_u8(v: f64) -> u8 {
    (256.0 * v.clamp(0.0, 0.99999)) as u8
}

impl From<Vec3> for Color {
    fn from(col: Vec3) -> Self {
        Color::new(
            lerp_ratio_to_u8(col.x),
            lerp_ratio_to_u8(col.y),
            lerp_ratio_to_u8(col.z),
        )
    }
}

impl<R, G, B> From<(R, G, B)> for Color
where
    R: Into<f64>,
    G: Into<f64>,
    B: Into<f64>,
{
    fn from(col: (R, G, B)) -> Self {
        Color::new(
            lerp_ratio_to_u8(col.0.into()),
            lerp_ratio_to_u8(col.1.into()),
            lerp_ratio_to_u8(col.2.into()),
        )
    }
}

impl RenderTarget for Bitmap {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        self.set_pixel(x, y, color);
    }
}

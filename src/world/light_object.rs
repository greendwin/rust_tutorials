use super::{GlowMat, LightDecl};
use crate::math::*;

#[derive(Clone, Debug)]
pub struct LightObject {
    sphere: Sphere,
    color: Vec3,
    intensity: f64, // C*I*1/d^2
}

impl LightObject {
    pub fn new(
        center: impl Into<Vec3>,
        radius: impl Into<f64>,
        color: impl Into<Vec3>,
        intensity: impl Into<f64>,
    ) -> Self {
        Self {
            sphere: Sphere::new(center, radius),
            color: color.into(),
            intensity: intensity.into(),
        }
    }
}

impl<Mat> HitRay<Mat> for LightObject
where
    Mat: From<GlowMat>,
{
    #[inline]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, Mat)> {
        self.sphere
            .hit(&ray, t_min, t_max)
            // .map(|hit| (hit, GlowMat::new(self.color).into()))
            .map(|hit| (hit, GlowMat::new(Vec3::zero()).into()))
    }
}

impl LightDecl for LightObject {
    #[inline]
    fn orig(&self) -> Vec3 {
        self.sphere.center
    }

    #[inline]
    fn radius(&self) -> f64 {
        self.sphere.radius
    }

    fn color_at(&self, pt: Vec3) -> Vec3 {
        let dist2 = (pt - self.sphere.center).length_squared();
        if dist2 <= self.sphere.radius.sqr() {
            return self.color;
        }

        self.color * self.intensity / dist2
    }
}

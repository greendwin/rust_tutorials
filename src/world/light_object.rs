use super::materials::GlowMat;
use crate::math::*;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct LightObject<Mat> {
    sphere: Sphere,
    color: Vec3,
    intensity: f64,            // C*I*1/d^2
    _marker: PhantomData<Mat>, // use it so that we can make generic HitRay
}

impl<Mat> LightObject<Mat> {
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
            _marker: PhantomData,
        }
    }

    pub fn color_at(&self, pt: Vec3) -> Vec3 {
        let dist2 = (pt - self.sphere.center).length_squared();
        if dist2 <= self.sphere.radius.sqr() {
            return self.color;
        }

        self.color * self.intensity / dist2
    }
}

impl<Mat> HitRay for LightObject<Mat>
where
    Mat: Material + Clone,
    Mat: From<GlowMat>,
{
    type Mat = Mat;

    #[inline]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, Mat)> {
        self.sphere
            .hit(&ray, t_min, t_max)
            .map(|hit| (hit, GlowMat::new(self.color_at(ray.orig)).into()))
    }
}

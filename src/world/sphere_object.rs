use crate::math::*;

#[derive(Debug, Clone)]
pub struct SphereObject<Mat>
where
    Mat: Material,
{
    pub sphere: Sphere,
    pub material: Mat,
}

impl<'a, Mat> SphereObject<Mat>
where
    Mat: Material,
{
    pub fn new(center: impl Into<Vec3>, radius: impl Into<f64>, material: impl Into<Mat>) -> Self {
        Self {
            sphere: Sphere::new(center, radius),
            material: material.into(),
        }
    }
}

impl<Mat> HitRay for SphereObject<Mat>
where
    Mat: Material + Clone,
{
    type Mat = Mat;

    #[inline]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, Mat)> {
        self.sphere
            .hit(&ray, t_min, t_max)
            .map(|hit| (hit, self.material.clone()))
    }
}

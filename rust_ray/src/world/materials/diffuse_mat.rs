use crate::math::*;
use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct DiffuseMat {
    pub albedo: Vec3,
}

impl DiffuseMat {
    pub fn new(albedo: impl Into<Vec3>) -> Self {
        Self {
            albedo: albedo.into(),
        }
    }
}

impl Material for DiffuseMat {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> ScatterResult {
        let mut next_dir = hit.norm + rand_vec3_in_unit_sphere().norm();
        // let next_dir = hit.norm + rand_vec3_in_hemisphere(hit.norm); // old times

        if next_dir.near_zero() {
            next_dir = hit.norm;
        }

        ScatterResult::scatter(Ray::new(hit.pt, next_dir), self.albedo, 1.0)
    }
}

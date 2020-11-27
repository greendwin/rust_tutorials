use crate::math::*;
use crate::utils::*;
use crate::world::methods::*;

#[derive(Debug, Copy, Clone)]
pub struct MetalMat {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl MetalMat {
    pub fn new(albedo: impl Into<Vec3>, fuzz: impl Into<f64>) -> Self {
        Self {
            albedo: albedo.into(),
            fuzz: fuzz.into().min(1.0),
        }
    }
}

impl Material for MetalMat {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> ScatterResult {
        let reflected = reflect(ray_in.dir.norm(), hit.norm);
        let next_dir = reflected + rand_vec3_in_unit_sphere() * self.fuzz;

        if next_dir.dot(hit.norm) <= 0.0 {
            return ScatterResult::None;
        }

        ScatterResult::scatter(Ray::new(hit.pt, next_dir), self.albedo, 0.01)
    }
}

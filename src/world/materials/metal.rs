use crate::math::*;
use crate::utils::*;
use crate::world::methods::*;

#[derive(Debug, Clone)]
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
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray_in.dir.norm(), hit.norm);
        let next_dir = reflected + rand_vec3_in_unit_sphere() * self.fuzz;

        if next_dir.dot(hit.norm) <= 0.0 {
            return None;
        }

        (Ray::new(hit.pt, next_dir), self.albedo).into()
    }
}

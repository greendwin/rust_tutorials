use crate::math::*;
use crate::utils::*;
use std::rc::Rc;

pub struct DiffuseMat {
    albedo: Vec3,
}

impl DiffuseMat {
    pub fn new(albedo: impl Into<Vec3>) -> Rc<Self> {
        Rc::new(Self {
            albedo: albedo.into(),
        })
    }
}

impl Material for DiffuseMat {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)> {
        let mut next_dir = hit.norm + rand_vec3_in_unit_sphere().norm();
        // let next_dir = hit.norm + rand_vec3_in_hemisphere(hit.norm); // old times

        if next_dir.near_zero() {
            next_dir = hit.norm;
        }

        (Ray::new(hit.pt, next_dir), self.albedo).into()
    }
}

pub struct MetalMat {
    albedo: Vec3,
    fuzz: f64,
}

impl MetalMat {
    pub fn new(albedo: impl Into<Vec3>, fuzz: impl Into<f64>) -> Rc<Self> {
        Rc::new(Self {
            albedo: albedo.into(),
            fuzz: fuzz.into().min(1.0),
        })
    }
}

impl Material for MetalMat {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)> {
        let reflected = ray_in.dir.norm().reflect(hit.norm);
        let next_dir = reflected + rand_vec3_in_unit_sphere() * self.fuzz;

        if next_dir.dot(hit.norm) <= 0.0 {
            return None;
        }

        (Ray::new(hit.pt, next_dir), self.albedo).into()
    }
}

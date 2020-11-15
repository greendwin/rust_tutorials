use crate::math::*;
use crate::utils::*;
use std::rc::Rc;

pub struct DiffuseMat {
    color: Vec3,
}

impl DiffuseMat {
    pub fn new(color: impl Into<Vec3>) -> Rc<Self> {
        Rc::new(Self {
            color: color.into(),
        })
    }
}

impl Material for DiffuseMat {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)> {
        let next_dir = hit.norm + rand_vec3_in_unit_sphere().norm(); // lambertian

        // let next_dir = hit.norm + rand_vec3_in_hemisphere(hit.norm); // old times

        (Ray::new(hit.pt, next_dir), self.color).into()
    }
}

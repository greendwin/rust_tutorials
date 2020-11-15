use std::rc::Rc;

use super::ray::{Hit, Ray};
use super::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Vec3)>;
}

pub type MaterialPtr = Rc<dyn Material>;

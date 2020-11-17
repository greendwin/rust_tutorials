use crate::math::*;

use super::materials::SomeMaterial;
use super::sphere_object::SphereObject;

#[derive(Debug, Clone)]
pub enum SomeObject {
    Sphere(SphereObject<SomeMaterial>),
}

type SomeSphere = SphereObject<SomeMaterial>;

impl From<SomeSphere> for SomeObject {
    fn from(sphere: SomeSphere) -> Self {
        SomeObject::Sphere(sphere)
    }
}

impl HitRay for SomeObject {
    type Mat = SomeMaterial;

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, Self::Mat)> {
        match self {
            SomeObject::Sphere(p) => p.hit(ray, t_min, t_max),
        }
    }
}

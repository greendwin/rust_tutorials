use crate::math::*;

use super::materials::SomeMaterial;
use super::sphere_object::SphereObject;

#[derive(Debug, Clone)]
pub enum SomeObject {
    Sphere(SphereObject<SomeMaterial>),
}

use SomeObject::*;
type SomeSphere = SphereObject<SomeMaterial>;

impl From<SomeSphere> for SomeObject {
    fn from(sphere: SomeSphere) -> Self {
        Sphere(sphere)
    }
}

impl HitRay for SomeObject {
    type Mat = SomeMaterial;

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, Self::Mat)> {
        match self {
            Sphere(p) => p.hit(ray, t_min, t_max),
        }
    }
}

impl BoundBox for SomeObject {
    fn get_bounds(&self) -> AABB {
        match self {
            Sphere(p) => AABB::new(
                p.sphere.center - p.sphere.radius,
                p.sphere.center + p.sphere.radius,
            ),
        }
    }
}

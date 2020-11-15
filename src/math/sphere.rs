use std::rc::Rc;

use super::material::MaterialPtr;
use super::ray::{Hit, HitRay, Ray};
use super::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: MaterialPtr,
}

impl Sphere {
    pub fn new(center: impl Into<Vec3>, radius: impl Into<f64>, material: MaterialPtr) -> Self {
        Self {
            center: center.into(),
            radius: radius.into(),
            material,
        }
    }
}

impl HitRay for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, MaterialPtr)> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd) / a;

        if t < t_min || t > t_max {
            // try second root
            t = (-half_b + sqrtd) / a;

            if t < t_min || t > t_max {
                return None;
            }
        }

        let pt = ray.at(t);
        let outward_norm = (pt - self.center) / self.radius;

        Some((
            Hit::new(ray, t, pt, outward_norm),
            Rc::clone(&self.material),
        ))
    }
}

// TODO: test me!

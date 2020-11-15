use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: impl Into<Vec3>, radius: impl Into<f64>) -> Self {
        Self {
            center: center.into(),
            radius: radius.into(),
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            // assume closest `t` is in front of camera (for now)
            Some((-half_b - discriminant.sqrt()) / a)
        }
    }
}

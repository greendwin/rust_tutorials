use super::material::Material;
use super::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Hit {
    pub pt: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(ray: &Ray, t: f64, pt: Vec3, outward_norm: Vec3) -> Self {
        let front_face = ray.dir.dot(outward_norm) < 0.0;
        let norm = if front_face {
            outward_norm
        } else {
            -outward_norm
        };

        Self {
            pt,
            norm,
            t,
            front_face,
        }
    }
}

pub trait HitRay {
    type Mat: Material;

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, Self::Mat)>;
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: impl Into<Vec3>, direction: impl Into<Vec3>) -> Self {
        Self {
            orig: origin.into(),
            dir: direction.into(),
        }
    }

    #[inline]
    pub fn at(&self, t: impl Into<f64>) -> Vec3 {
        self.orig + self.dir * t.into()
    }
}

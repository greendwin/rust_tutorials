use super::material::MaterialPtr;
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, MaterialPtr)>;
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new<T: Into<Vec3>>(origin: T, direction: T) -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_at() {
        let r = Ray::new((0, 0, 0), (1, 2, 3));

        assert_eq!(r.at(0), r.orig);
    }
}

use super::vec3::Vec3;

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

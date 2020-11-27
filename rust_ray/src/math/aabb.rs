use super::sphere::Sphere;
use super::vec3::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: impl Into<Vec3>, max: impl Into<Vec3>) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }

    pub fn zero() -> Self {
        AABB::new(Vec3::zero(), Vec3::zero())
    }

    pub fn from_many(arr: &[AABB]) -> Self {
        let (first, rest) = match arr.split_first() {
            None => return AABB::zero(),
            Some(p) => p,
        };

        let mut result = first.clone();
        for p in rest {
            result.expand(p);
        }
        result
    }

    pub fn is_inside(&self, point: impl Into<Vec3>) -> bool {
        let point = point.into();
        (self.min - f64::EPSILON) < point && point < (self.max + f64::EPSILON)
    }

    pub fn is_intersects(&self, other: &AABB) -> bool {
        if !(self.min <= other.max) {
            return false;
        }

        if !(self.max >= other.min) {
            return false;
        }

        true
    }

    pub fn intersect(&self, other: &AABB) -> Option<AABB> {
        if !self.is_intersects(other) {
            return None;
        }

        Some(AABB::new(self.min.max(other.min), self.max.min(other.max)))
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn expand(&mut self, other: &AABB) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Into<Vec3>> From<T> for AABB {
    fn from(other: T) -> Self {
        let other = other.into();
        Self::new(other, other)
    }
}

impl From<Sphere> for AABB {
    fn from(other: Sphere) -> Self {
        Self::new(other.center - other.radius, other.center + other.radius)
    }
}

use std::fmt::{self, Display};
use std::ops::*;

use crate::math::AvgEq;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[inline]
    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    #[inline]
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    #[inline]
    pub fn from_hex(val: u32) -> Self {
        let r = (val >> 16) & 0xff;
        let g = (val >> 8) & 0xff;
        let b = val & 0xff;

        Self::from_rgb(r as u8, g as u8, b as u8)
    }

    #[inline]
    pub fn from_scalar(scalar: impl Into<f64>) -> Self {
        let scalar = scalar.into();

        Self {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        let r = *self * other;
        r.x + r.y + r.z
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    pub fn norm(&self) -> Self {
        *self / self.length()
    }

    #[inline]
    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<X, Y, Z> From<(X, Y, Z)> for Vec3
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
{
    fn from(other: (X, Y, Z)) -> Self {
        Self::new(other.0, other.1, other.2)
    }
}

impl AvgEq for Vec3 {
    fn avg_eq(self, other: Self) -> bool {
        self.x.avg_eq(other.x) && self.y.avg_eq(other.y) && self.z.avg_eq(other.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Into<f64>> Mul<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: T) -> Self {
        let other = other.into();

        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: Into<f64>> Div<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: T) -> Self {
        self * (1.0 / other.into())
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec3_create() {
        let v = Vec3::new(1, 2, 3);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn vec3_print() {
        let v = Vec3::new(1, 2, 3);
        assert_eq!("(1, 2, 3)", format!("{}", v));
    }

    #[test]
    fn vec3_neg() {
        let v = -Vec3::new(1, 2, 3);
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, -2.0);
        assert_eq!(v.z, -3.0);
    }

    #[test]
    fn vec3_add() {
        let v = Vec3::new(1, 2, 3) + Vec3::new(4, 5, 6);
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 7.0);
        assert_eq!(v.z, 9.0);
    }

    #[test]
    fn vec3_add_scalar() {
        let v = Vec3::new(1, 2, 3) + 100.0;
        assert_eq!(v.x, 101.0);
        assert_eq!(v.y, 102.0);
        assert_eq!(v.z, 103.0);
    }

    #[test]
    fn vec3_add_assign() {
        let mut v = Vec3::new(1, 2, 3);
        v += Vec3::new(4, 5, 6);
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 7.0);
        assert_eq!(v.z, 9.0);
    }

    #[test]
    fn vec3_add_assign_scalar() {
        let mut v = Vec3::new(1, 2, 3);
        v += 10.0;
        assert_eq!(v.x, 11.0);
        assert_eq!(v.y, 12.0);
        assert_eq!(v.z, 13.0);
    }

    #[test]
    fn vec3_sub() {
        let v = Vec3::new(4, 5, 6) - Vec3::new(1, 3, 5);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn vec3_sub_scalar() {
        assert_eq!(Vec3::new(4, 5, 6) - 1.0, Vec3::new(3, 4, 5));
        assert_eq!(
            Vec3::from_scalar(10.0) - Vec3::new(4, 5, 6),
            Vec3::new(6, 5, 4)
        );
    }

    #[test]
    fn vec3_mul() {
        assert_eq!(
            Vec3::new(4, 5, 6) * Vec3::new(2, 3, 4),
            Vec3::new(8, 15, 24)
        );
    }

    #[test]
    fn vec3_mul_scalar() {
        assert_eq!(Vec3::new(4, 5, 6) * 10.0, Vec3::new(40, 50, 60));
    }

    #[test]
    fn vec3_mul_assign_scalar() {
        let mut v = Vec3::new(1, 2, 3);
        v *= 10.0;
        assert_eq!(v.x, 10.0);
        assert_eq!(v.y, 20.0);
        assert_eq!(v.z, 30.0);
    }

    #[test]
    fn vec3_div_scalar() {
        assert_eq!(Vec3::new(4, 5, 6) / 2.0, Vec3::new(2, 2.5, 3));
    }

    #[test]
    fn vec3_div_assign_scalar() {
        let mut v = Vec3::new(1, 2, 3);
        v /= 10.0;
        assert_eq!(v.x, 0.1);
        assert_eq!(v.y, 0.2);
        assert!(v.z.avg_eq(0.3));
    }

    #[test]
    fn vec3_dot() {
        assert_eq!(Vec3::new(1, 2, 3).dot(Vec3::new(2, 3, 4)), 20.0);
    }

    #[test]
    fn vec3_cross() {
        assert_eq!(
            Vec3::new(1, 0, 0).cross(Vec3::new(0, 1, 0)),
            Vec3::new(0, 0, 1)
        );
    }

    #[test]
    fn vec3_norm() {
        assert!(Vec3::new(1, 1, 1).norm().avg_eq(Vec3::new(
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0
        )));
    }
}

use num::{Float, Num};
use std::fmt::{self, Display};
use std::ops::*;

use crate::math::AvgEq;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct V3<T>
where
    T: Num + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3 = V3<f64>;

impl<T: Num + Copy> V3<T> {
    #[inline]
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    #[inline]
    pub fn new(x: impl Into<T>, y: impl Into<T>, z: impl Into<T>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    #[inline]
    pub fn from_scalar(scalar: impl Into<T>) -> Self {
        let scalar = scalar.into();

        Self {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }

    #[inline]
    pub fn length<R>(&self) -> R
    where
        R: Float,
        T: Into<R>,
    {
        let r: R = self.length_squared().into();
        r.sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn dot(&self, other: Self) -> T {
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
    pub fn norm(&self) -> Self
    where
        T: Float,
    {
        *self / self.length()
    }
}

impl<T> Display for V3<T>
where
    T: Num + Copy + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> AvgEq for V3<T>
where
    T: Float,
{
    fn avg_eq(self, other: Self) -> bool {
        self.x.avg_eq(other.x) && self.y.avg_eq(other.y) && self.z.avg_eq(other.z)
    }
}

impl<T> Neg for V3<T>
where
    T: Num + Copy + Neg<Output = T>,
{
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

impl<T> Add for V3<T>
where
    T: Num + Copy,
{
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

impl<T> Add<T> for V3<T>
where
    T: Num + Copy,
{
    type Output = Self;

    #[inline]
    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<T> AddAssign for V3<T>
where
    T: Num + Copy + AddAssign,
{
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> AddAssign<T> for V3<T>
where
    T: Num + Copy + AddAssign,
{
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl<T> Sub for V3<T>
where
    T: Num + Copy,
{
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

impl<T> Sub<T> for V3<T>
where
    T: Num + Copy,
{
    type Output = Self;

    #[inline]
    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T> Mul for V3<T>
where
    T: Num + Copy,
{
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

impl<T> Mul<T> for V3<T>
where
    T: Num + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> MulAssign<T> for V3<T>
where
    T: Num + Copy + MulAssign,
{
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T> Div<T> for V3<T>
where
    T: Num + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> DivAssign<T> for V3<T>
where
    T: Num + Copy + DivAssign,
{
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
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
        assert_eq!(v.z, 0.3);
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

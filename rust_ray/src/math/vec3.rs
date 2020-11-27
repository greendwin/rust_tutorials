use std::cmp::Ordering;
use std::f64;
use std::fmt::{self, Display};
use std::ops::*;

#[derive(Debug, PartialEq, Copy, Clone)]
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
    pub fn near_zero(&self) -> bool {
        self.x.abs() < 1e-6 && self.y.abs() < 1e-6 && self.z.abs() < 1e-6
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

    #[inline]
    pub fn min(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    #[inline]
    pub fn max(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Vec3) -> Option<Ordering> {
        use Ordering::*;

        if self == other {
            return Some(Equal);
        }

        if self.x < other.x && self.y < other.y && self.z < other.z {
            return Some(Less);
        }

        if self.x > other.x && self.y > other.y && self.z > other.z {
            return Some(Greater);
        }

        return None;
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

use num::{Float, One, Zero};
use std::f64::consts::PI;
use std::ops::*;

pub trait Sqr {
    fn sqr(self) -> Self;
}

impl<T: Float> Sqr for T {
    fn sqr(self) -> Self {
        self * self
    }
}

pub trait Clamp: Zero + One {
    fn clamp(self, min: Self, max: Self) -> Self;

    #[inline]
    fn clamp01(self) -> Self {
        self.clamp(Self::zero(), Self::one())
    }
}

impl<T: Float> Clamp for T {
    #[inline]
    fn clamp(self, min: T, max: T) -> T {
        self.max(min).min(max)
    }
}

pub trait Lerp<U>: Clamp {
    fn lerp_unclamped(self, a: U, b: U) -> U;

    fn lerp(self, a: U, b: U) -> U {
        self.clamp01().lerp_unclamped(a, b)
    }
}

impl<T, U> Lerp<U> for T
where
    T: Float,
    U: Copy + Add<Output = U> + Sub<Output = U> + Mul<Self, Output = U>,
{
    #[inline]
    fn lerp_unclamped(self, a: U, b: U) -> U {
        a + (b - a) * self
    }
}

pub trait InvLerp<U = Self> {
    fn inv_lerp(self, a: Self, b: Self) -> U;
}

impl<T: Float> InvLerp for T {
    #[inline]
    fn inv_lerp(self, a: Self, b: Self) -> Self {
        if a == b {
            return T::zero();
        }

        (self - a) / (b - a)
    }
}

pub trait DegToRad {
    fn deg_to_rad(self) -> Self;
}

impl DegToRad for f64 {
    #[inline]
    fn deg_to_rad(self) -> f64 {
        self * PI / 180.0
    }
}

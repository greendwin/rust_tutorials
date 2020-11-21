use num::Float;
use std::f64;
use std::ops::*;

#[inline]
pub fn clamp<T>(val: T, min: T, max: T) -> T
where
    T: Float,
{
    val.max(min).min(max)
}

#[inline]
pub fn clamp01<T>(val: T) -> T
where
    T: Float,
{
    clamp(val, T::zero(), T::one())
}

#[inline]
pub fn lerp<T, U>(t: T, a: U, b: U) -> U
where
    T: Float,
    U: Copy + Add<Output = U> + Sub<Output = U> + Mul<T, Output = U>,
{
    a + (b - a) * clamp01(t)
}

#[inline]
pub fn lerp_unclamped<T, U>(t: T, a: U, b: U) -> U
where
    T: Float,
    U: Copy + Add<Output = U> + Sub<Output = U> + Mul<T, Output = U>,
{
    a + (b - a) * t
}

#[inline]
pub fn inv_lerp<T>(val: T, a: T, b: T) -> T
where
    T: Float,
{
    if a == b {
        return T::zero();
    }

    (val - a) / (b - a)
}

#[inline]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * f64::consts::PI / 180.0
}

use rand;
use rand::prelude::*;

use crate::math::*;

#[inline]
pub fn random() -> f64 {
    rand::thread_rng().gen_range(0f32, 1f32) as f64
}

#[inline]
pub fn rand_range(min: impl Into<f64>, max: impl Into<f64>) -> f64 {
    lerp_unclamped(random(), min.into(), max.into())
}

#[inline]
pub fn rand_vec3(min: impl Into<f64>, max: impl Into<f64>) -> Vec3 {
    let min = min.into();
    let max = max.into();

    Vec3::new(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    )
}

#[inline]
pub fn rand_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let r = rand_vec3(-1, 1);
        if r.length_squared() < 1.0 {
            return r;
        }
    }
}

pub fn rand_vec3_in_hemisphere(norm: Vec3) -> Vec3 {
    let r = rand_vec3_in_unit_sphere();
    if norm.dot(r) > 0.0 {
        r
    } else {
        -r
    }
}

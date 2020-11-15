use rand;
use rand::prelude::*;

use crate::math::{lerp, Vec3};

#[inline]
pub fn random() -> f64 {
    rand::thread_rng().gen_range(0f64, 1f64)
}

#[inline]
pub fn rand_range(min: f64, max: f64) -> f64 {
    lerp(random(), min, max)
}

#[inline]
pub fn rand_vec3() -> Vec3 {
    Vec3::new(random(), random(), random())
}

#[inline]
pub fn rand_in_unit_sphere() -> Vec3 {
    loop {
        let r = rand_vec3();
        if r.length_squared() < 1.0 {
            return r;
        }
    }
}

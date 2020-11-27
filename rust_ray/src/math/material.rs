use super::ray::{Hit, Ray};
use super::vec3::Vec3;

#[derive(Clone, Debug)]
pub enum ScatterResult {
    Scatter {
        scatter: Ray,
        color: Vec3,
        light_absorb: f64,
    },
    Glow {
        color: Vec3,
    },
    None,
}

impl ScatterResult {
    pub fn scatter(scatter: Ray, color: Vec3, light_absorb: f64) -> Self {
        Self::Scatter {
            scatter,
            color,
            light_absorb,
        }
    }

    pub fn glow(color: Vec3) -> Self {
        Self::Glow { color }
    }

    pub fn none() -> Self {
        Self::None
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> ScatterResult;
}

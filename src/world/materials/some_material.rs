use super::*;
use crate::math::*;

#[derive(Debug, Copy, Clone)]
pub enum SomeMaterial {
    Diff(DiffuseMat),
    Metal(MetalMat),
    Di(DielectricMat),
    Glow(GlowMat),
}

use SomeMaterial::*;

macro_rules! impl_from {
    ($($mat:ident => $var:ident),*) => {
        $(
            impl From<$mat> for SomeMaterial {
                fn from(mat: $mat) -> Self {
                    $var(mat)
                }
            }
        )*
    };
}

impl_from!(
    DiffuseMat => Diff,
    DielectricMat => Di,
    MetalMat => Metal,
    GlowMat => Glow);

impl Material for SomeMaterial {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> ScatterResult {
        match self {
            Diff(mat) => mat.scatter(ray_in, hit),
            Di(mat) => mat.scatter(ray_in, hit),
            Metal(mat) => mat.scatter(ray_in, hit),
            Glow(mat) => mat.scatter(ray_in, hit),
        }
    }
}

use crate::math::Vec3;

pub fn reflect(dir: Vec3, norm: Vec3) -> Vec3 {
    dir - norm * (2.0 * dir.dot(norm))
}

pub fn refract(uv: Vec3, norm: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = norm.dot(-uv).min(1.0);
    let r_out_perp = (uv + norm * cos_theta) * etai_over_etat;
    let r_out_parallel = norm * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());

    r_out_perp + r_out_parallel
}

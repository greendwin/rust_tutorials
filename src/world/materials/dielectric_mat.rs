use crate::math::*;
use crate::utils::*;
use crate::world::methods::*;

#[derive(Debug, Copy, Clone)]
pub struct DielectricMat {
    pub ir: f64,
}

impl DielectricMat {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}

impl Material for DielectricMat {
    fn hit(&self, ray_in: &Ray, hit: &Hit) -> HitResult {
        let attenuation = Vec3::new(1, 1, 1);

        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let norm_dir = ray_in.dir.norm();
        let cos_theta = hit.norm.dot(-norm_dir).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let out_dir = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
            reflect(norm_dir, hit.norm)
        } else {
            refract(norm_dir, hit.norm, refraction_ratio)
        };

        HitResult::scatter(Ray::new(hit.pt, out_dir), attenuation)
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

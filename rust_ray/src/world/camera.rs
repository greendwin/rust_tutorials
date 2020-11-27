use crate::math::*;
use crate::utils::rand_range;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

fn random_vec3_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand_range(-1, 1), rand_range(-1, 1), 0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov_deg: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov_deg.deg_to_rad();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).norm();
        let u = vup.cross(w).norm();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - w * focus_dist;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_vec3_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let next_orig = self.origin + offset;
        Ray::new(
            next_orig,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - next_orig,
        )
    }
}

use crate::math::*;

pub struct Scene<'a, Mat: Material> {
    objects: Vec<Box<dyn HitRay<'a, Mat> + 'a>>,
}

impl<'a, Mat: Material> Scene<'a, Mat> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: impl HitRay<'a, Mat> + 'a) {
        self.objects.push(Box::new(obj));
    }
}

impl<'a, 'b, Mat: Material> HitRay<'a, Mat> for Scene<'a, Mat> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, &'a Mat)> {
        let mut closest_hit = None;
        let mut cur_t_max = t_max;

        for obj in &self.objects {
            if let Some((hit, mat)) = obj.hit(ray, t_min, cur_t_max) {
                cur_t_max = hit.t;
                closest_hit.replace((hit, mat));
            }
        }

        closest_hit
    }
}

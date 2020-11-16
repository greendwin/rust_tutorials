use crate::math::*;

pub struct Scene<Obj> {
    objs: Vec<Obj>,
}

impl<Obj> Scene<Obj> {
    pub fn new() -> Self {
        Self { objs: Vec::new() }
    }

    pub fn add(&mut self, obj: Obj) {
        self.objs.push(obj);
    }
}

impl<'a, Obj> HitRay<'a> for Scene<Obj>
where
    Obj: HitRay<'a>,
{
    type Mat = Obj::Mat;

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, &'a Self::Mat)> {
        let mut closest_hit = None;
        let mut cur_t_max = t_max;

        for obj in &self.objs {
            if let Some((hit, mat)) = obj.hit(ray, t_min, cur_t_max) {
                cur_t_max = hit.t;
                closest_hit.replace((hit, mat));
            }
        }

        closest_hit
    }
}

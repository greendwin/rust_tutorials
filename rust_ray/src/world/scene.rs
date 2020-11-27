use crate::math::*;

pub trait LightDecl {
    fn orig(&self) -> Vec3;
    fn radius(&self) -> f64;
    fn color_at(&self, pt: Vec3) -> Vec3;
}

pub trait Scene {
    type Mat: Material;
    type Obj: HitRay<Self::Mat>;
    type Light: LightDecl;

    fn objs(&self) -> &[Self::Obj];
    fn lights(&self) -> &[Self::Light];
}

impl<Scn> HitRay<Scn::Mat> for Scn
where
    Scn: Scene,
{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(Hit, <Self as Scene>::Mat)> {
        let mut closest_hit = None;
        let mut cur_t_max = t_max;

        for obj in self.objs() {
            if let Some((hit, mat)) = obj.hit(ray, t_min, cur_t_max) {
                cur_t_max = hit.t;
                closest_hit.replace((hit, mat));
            }
        }

        // for lgt in self.lights() {
        //     if let Some((hit, mat)) = lgt.hit(ray, t_min, cur_t_max) {
        //         cur_t_max = hit.t;
        //         closest_hit.replace((hit, mat));
        //     }
        // }

        closest_hit
    }
}

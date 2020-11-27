use super::scene_loader::SceneLoader;
use crate::math::*;
use crate::utils::*;
use crate::world::*;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum MaterialRndConfig {
    #[serde(rename = "diff")]
    DiffRnd { weight: f64 },

    #[serde(rename = "metal")]
    MetalRnd { weight: f64 },

    #[serde(rename = "glass")]
    Glass {
        weight: f64,
        refraction: f64,
        width: f64,
    },
}

impl MaterialRndConfig {
    fn weight(&self) -> f64 {
        use MaterialRndConfig::*;
        match self {
            DiffRnd { weight } | MetalRnd { weight } | Glass { weight, .. } => *weight,
        }
    }

    fn make_sphere(&self, center: Vec3, radius: f64, scene: &SceneLoader) {
        use MaterialRndConfig::*;
        match self {
            DiffRnd { .. } => {
                let albedo = rand_vec3(0, 1) * rand_vec3(0, 1);
                let mat = DiffuseMat::new(albedo);
                scene.add_obj(SphereObject::new(center, radius, mat));
            }
            MetalRnd { .. } => {
                let albedo = rand_vec3(0.5, 1);
                let fuzz = rand_range(0, 0.5);
                let mat = MetalMat::new(albedo, fuzz);
                scene.add_obj(SphereObject::new(center, radius, mat));
            }
            Glass {
                refraction, width, ..
            } => {
                let mat = DielectricMat::new(*refraction);
                scene.add_obj(SphereObject::new(center, radius, mat));
                scene.add_obj(SphereObject::new(center, radius - width, mat));
            }
        }
    }
}

#[derive(Deserialize)]
struct RandomGenConfig {
    nx: usize,
    nz: usize,
    center: (f64, f64, f64),
    step: f64,
    sphere_radius: f64,
    materials: Vec<MaterialRndConfig>,
}

pub struct RndGenLoader<'a> {
    scene: &'a SceneLoader,
}

impl<'a> RndGenLoader<'a> {
    pub fn new(scene: &'a SceneLoader) -> Self {
        Self { scene }
    }
}

impl<'a> ParserPlugin<'a> for RndGenLoader<'a> {
    fn init(&'a self, parser: &mut Parser<'a>) {
        parser.add_cmd("rnd_gen", move |data| {
            let data: Vec<RandomGenConfig> = data;

            for cfg in data {
                random_gen(self.scene, &cfg);
            }

            Ok(())
        })
    }
}

fn random_gen(scene: &SceneLoader, config: &RandomGenConfig) {
    let total_weight: f64 = config.materials.iter().map(|m| m.weight()).sum();
    let gen_center: Vec3 = config.center.into();

    let nx = config.nx;
    let nz = config.nz;
    let step = config.step;
    let gen_corner = gen_center - Vec3::new(nx as f64, 0, nz as f64) * (step / 2.0);

    for xx in 0..nx {
        for zz in 0..nz {
            let choose_mat = rand_range(0, total_weight);

            let sph_center =
                gen_corner + Vec3::new(xx as f64 + random(), 0, zz as f64 + random()) * step;

            let mut cur_weight = 0.0;
            for m in &config.materials {
                cur_weight += m.weight();

                if choose_mat < cur_weight {
                    m.make_sphere(sph_center, config.sphere_radius, scene);
                    break;
                }
            }
        }
    }
}

use crate::utils::{Parser, ParserError, ParserPlugin};
use crate::world::*;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;

pub type SomeScene = Scene<SomeObject>;

pub struct SceneLoader {
    materials: RefCell<HashMap<String, SomeMaterial>>,
    objects: RefCell<Vec<SomeObject>>,
}

impl SceneLoader {
    pub fn new() -> Self {
        Self {
            materials: RefCell::new(HashMap::new()),
            objects: RefCell::new(Vec::new()),
        }
    }

    pub fn new_scene(&self) -> SomeScene {
        let mut scene = Scene::new();

        for obj in self.objects.borrow().iter() {
            scene.add(obj.clone());
        }

        scene
    }

    pub fn add_obj(&self, obj: impl Into<SomeObject>) {
        self.objects.borrow_mut().push(obj.into());
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum MaterialConfig {
    #[serde(rename = "diff")]
    Diff {
        name: String,
        color: (f64, f64, f64),
    },

    #[serde(rename = "di")]
    Di { name: String, refraction: f64 },

    #[serde(rename = "metal")]
    Metal {
        name: String,
        color: (f64, f64, f64),
        fuzz: f64,
    },

    #[serde(rename = "glow")]
    Glow {
        name: String,
        color: (f64, f64, f64),
    },
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ObjectConfig {
    #[serde(rename = "sphere")]
    Sphere {
        material: String,
        center: (f64, f64, f64),
        radius: f64,
    },
}

impl<'a> ParserPlugin<'a> for SceneLoader {
    fn init(&'a self, parser: &mut Parser<'a>) {
        parser.add_cmd("materials", move |data| {
            let data: Vec<MaterialConfig> = data;
            let mut mats = self.materials.borrow_mut();

            for m in data {
                use MaterialConfig::*;

                match m {
                    Diff { name, color } => {
                        mats.insert(name, DiffuseMat::new(color).into());
                    }
                    Di { name, refraction } => {
                        mats.insert(name, DielectricMat::new(refraction).into());
                    }
                    Metal { name, color, fuzz } => {
                        mats.insert(name, MetalMat::new(color, fuzz).into());
                    }
                    Glow { name, color } => {
                        mats.insert(name, GlowMat::new(color).into());
                    }
                }
            }

            Ok(())
        });

        parser.add_cmd("objects", move |data| {
            let data: Vec<ObjectConfig> = data;
            let mats = self.materials.borrow();
            let mut objs = self.objects.borrow_mut();

            for obj in data {
                use ObjectConfig::*;

                match obj {
                    Sphere {
                        material,
                        center,
                        radius,
                    } => {
                        let mat = mats.get(&material).ok_or_else(|| {
                            ParserError::Msg(format!("unknown material: {:?}", &material))
                        })?;
                        objs.push(SphereObject::new(center, radius, mat.clone()).into());
                    }
                }
            }

            Ok(())
        });
    }
}

use super::{LightObject, Scene, SomeMaterial, SomeObject};

pub type SomeLight = LightObject<SomeMaterial>;

pub struct SomeScene {
    objs: Vec<SomeObject>,
    lights: Vec<SomeLight>,
}

impl SomeScene {
    pub fn new() -> Self {
        Self {
            objs: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, obj: impl Into<SomeObject>) {
        self.objs.push(obj.into());
    }

    pub fn add_light(&mut self, light: impl Into<SomeLight>) {
        self.lights.push(light.into());
    }
}

impl Scene for SomeScene {
    type Mat = SomeMaterial;
    type Obj = SomeObject;
    type Light = SomeLight;

    fn objs(&self) -> &[SomeObject] {
        &self.objs
    }

    fn lights(&self) -> &[SomeLight] {
        &self.lights
    }
}

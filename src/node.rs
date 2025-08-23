use hashbrown::HashMap;
use std::any::Any;

use crate::render::Ctx;
use crate::app::App;
use crate::cross::*;
use crate::draw::*;


#[derive(Default)]
pub struct Group {
    object_list: HashMap<String, Box<dyn Node>>,
}
impl Group {
    pub fn add(&mut self, name: &str, object: impl Node) {
        self.object_list.insert(name.to_string(), Box::new(object));
    }
}
impl Node for Group {
    fn update(&mut self, app: &App) {
        for (_, obj) in &mut self.object_list {
            obj.update(app);
        }
    }
    fn draw(&mut self, ctx: &mut Ctx, mvp: &Mat4) {
        for (_, obj) in &mut self.object_list {
            obj.draw(ctx, mvp);
        }
    }
}


pub struct Factory {
    object_list: Vec<Object>,
}
impl Factory {
    pub fn new() -> Self {
        Self {
            object_list: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: Object) {
        self.object_list.push(obj);
    }
}
impl Node for Factory {
    fn update(&mut self, app: &App) {
        cross_chunks(&mut self.object_list, 1000).for_each(|chunk| {
            for obj in chunk {
                obj.update(app);
            }
        });
    }
    fn draw(&mut self, ctx: &mut Ctx, mvp: &Mat4) {
        for obj in &mut self.object_list {
            obj.draw(ctx, mvp);
        }
    }
}


pub struct Object {
    pub modules: ModulesObject,
    draw: Draw,

    position: LData<Vec3>,
    scale: LData<Vec3>,
    rotation: LData<Vec3>,
    
    color: LData<Color>,
}
impl Object {
    pub(crate) fn new(draw: Draw) -> Self {
        Self {
            modules: ModulesObject::default(),
            draw,

            position: LData::new(Vec3::ZERO),
            scale: LData::new(Vec3::ONE),
            rotation: LData::new(Vec3::ZERO),

            color: LData::new(Color::new(1., 1., 1., 1.)),
        }
    }
    pub fn position(&self) -> MutexGuard<'_, Vec3> {
        self.position.lock()
    }
    pub fn scale(&self) -> MutexGuard<'_, Vec3> {
        self.scale.lock()
    }
    pub fn rotation(&self) -> MutexGuard<'_, Vec3> {
        self.rotation.lock()
    }
    pub fn color(&self) -> MutexGuard<'_, Color> {
        self.color.lock()
    }
}
impl Node for Object {
    fn update(&mut self, app: &App) {
        if self.modules.is_size() {
            let mut modules = take(&mut self.modules);
            modules.update(app, &self);
            self.modules = modules;
        }
    }
    fn draw(&mut self, ctx: &mut Ctx, mvp: &Mat4) {
        let position = *self.position.lock();
        let position = Mat4::from_translation(position);

        let scale = *self.scale.lock();
        let scale = Mat4::from_scale(scale);

        let rotation = *self.rotation.lock();
        let rot_x = Mat4::from_rotation_x(rotation.x);
        let rot_y = Mat4::from_rotation_y(rotation.y);
        let rot_z = Mat4::from_rotation_z(rotation.z);
        let rotation = rot_y * rot_x * rot_z;

        let mvp = mvp * position * rotation * scale;
        let color = *self.color();
        self.draw.draw(ctx, mvp, &color);
    }
}


#[derive(Default)]
pub struct ModulesObject {
    module_list: Vec<Box<dyn ModuleObject>>,
    module_list_len: usize,
}
impl ModulesObject {
    pub fn add(&mut self, module: impl ModuleObject) {
        self.module_list.push(Box::new(module));
    }
    pub(crate) fn update(&mut self, app: &App, obj: &Object) {
        for module in &mut self.module_list[self.module_list_len..] {
            module.ready(app, obj);
        }
        self.module_list_len = self.module_list.len();

        for module in &mut self.module_list {
            module.procces(app, obj);
        }
    }
    pub(crate) fn is_size(&self) -> bool {
        self.module_list.len() != 0
    }
}
pub trait ModuleObject: Any + Sync + Send {
    fn ready(&mut self, _app: &App, _obj: &Object) {}
    fn procces(&mut self, _app: &App, _obj: &Object) {}
}


pub trait Node: Any + Sync + Send {
    fn update(&mut self, app: &App);
    fn draw(&mut self, ctx: &mut Ctx, mvp: &Mat4);
}


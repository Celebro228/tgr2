use hashbrown::HashMap;
use std::any::Any;

use crate::app::App;
use crate::cross::*;
use crate::object::*;
use crate::render::Ctx;
use crate::draw::*;


pub fn cube(w: f32, h: f32, l: f32) -> Model {
    let w = w / 2.;
    let h = h / 2.;
    let l = l / 2.;
    let verts = vec![
        vec3(-w, -h, l), // 0
        vec3(w, -h, l), // 1
        vec3(w, h, l), // 2
        vec3(-w, h, l), // 3
        vec3(w, h, -l), // 4
        vec3(w, -h, -l), // 5
        vec3(-w, -h, -l), // 6
        vec3(-w, h, -l), // 7
    ];
    let indis = vec![
        0, 1, 2, 2, 3, 0,
        7, 6, 0, 0, 3, 7,
        0, 6, 5, 5, 1, 0,
        2, 1, 5, 5, 4, 2,
        7, 3, 2, 2, 4, 7,
        4, 5, 6, 6, 7, 4,
    ];
    Model::new(Draw::new(verts, indis))
}

pub fn model() -> Model {
    Model::new(Draw::new(Vec::new(), Vec::new()))
}


#[derive(Default)]
pub struct Group3d {
    pub object_list: HashMap<String, Box<dyn Object3d>>,
}
impl Group3d {
    pub fn add(&mut self, name: &str, object: impl Object3d) {
        self.object_list.insert(name.to_string(), Box::new(object));
    }
}
impl Object for Group3d {
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
impl Object3d for Group3d {}


#[derive(Default)]
pub struct Factory3d {
    pub object_list: Vec<Model>,
}
impl Factory3d {
    pub fn add(&mut self, shape: Model) {
        self.object_list.push(shape);
    }
}
impl Object for Factory3d {
    fn update(&mut self, app: &App) {
        for obj in &mut self.object_list {
            obj.update(app);
        }
    }
    fn draw(&mut self, ctx: &mut Ctx, mvp: &Mat4) {
        for obj in &mut self.object_list {
            obj.draw(ctx, mvp);
        }
    }
}
impl Object3d for Factory3d {}


pub struct Model {
    draw: Draw,
    pub modules: ModulesModel,

    // Transwofm
    pub position: LData<Vec3>,
    pub scale: LData<Vec3>,
    pub rotation: LData<Vec3>,
    
    pub color: Color,
}
impl Model {
    pub(crate) fn new(draw: Draw) -> Self {
        Self {
            draw,
            modules: ModulesModel::default(),
            position: LData::new(Vec3::ZERO),
            scale: LData::new(Vec3::ONE),
            rotation: LData::new(Vec3::ZERO),
            color: Color::new(1., 1., 1., 1.),
        }
    }
    /*pub(crate) fn get_mat(&self) -> Mat4 {
        
    }*/
}
impl Object for Model {
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
        self.draw.draw(ctx, mvp, &self.color);
    }
}
impl Object3d for Model {}


#[derive(Default)]
pub struct ModulesModel {
    module_list: Vec<Box<dyn ModuleModel>>,
    module_list_len: usize,
}
impl ModulesModel {
    pub fn add(&mut self, module: impl ModuleModel) {
        self.module_list.push(Box::new(module));
    }
    pub(crate) fn update(&mut self, app: &App, obj: &Model) {
        for module in &mut self.module_list[self.module_list_len..] {
            module.ready(app, obj);
        }
        self.module_list_len = self.module_list.len();

        cross_iter(&mut self.module_list).for_each(|module| {
            module.procces(app, obj);
        });
    }
    pub(crate) fn is_size(&self) -> bool {
        self.module_list.len() != 0
    }
}
pub trait ModuleModel: Any + Sync + Send {
    fn ready(&mut self, app: &App, obj: &Model);
    fn procces(&mut self, app: &App, obj: &Model);
}


pub trait Object3d: Object {}

/*pub struct ObjRef<'a, T> {
    object_add_list_lock: MutexGuard<'a, Vec<(String, T)>>
}

impl<'a, T> ObjRef<'a, T> {
    pub(crate) fn new(mutex: MutexGuard<'a, Vec<(String, T)>>) -> Self {
        Self {
            object_add_list_lock: mutex
        }
    }

    pub fn obj(&self) -> &T {
        &self.object_add_list_lock
            .last()
            .expect("Failed get obj for objRef")
            .1
    }

    pub fn name(&self) -> &String {
        &self.object_add_list_lock
            .last()
            .expect("Failed get name for objRef")
            .0
    }
} */


/*use hashbrown::HashMap;

use crate::cross::*;


pub struct Model {
    // Transwofm
    pub position: LData<Vec3>,
    pub scale: LData<Vec3>,
    pub rotation: LData<Vec3>,
}

impl Model {
    pub(crate) fn new() -> Self {
        Self {
            position: LData::new(Vec3::ZERO),
            scale: LData::new(Vec3::ONE),
            rotation: LData::new(Vec3::ZERO),
        }
    }

    pub(crate) fn get_draw(&self) -> Mat4 {
        let position = *self.position.lock();
        let position = Mat4::from_translation(position);

        let scale = *self.scale.lock();
        let scale = Mat4::from_scale(scale);

        let rotation = *self.rotation.lock();
        let rot_x = Mat4::from_rotation_x(rotation.x);
        let rot_y = Mat4::from_rotation_y(rotation.y);
        let rot_z = Mat4::from_rotation_z(rotation.z);
        let rotation = rot_y * rot_x * rot_z;

        position * rotation * scale
    }
}
*/
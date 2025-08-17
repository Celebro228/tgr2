use hashbrown::HashMap;
use std::any::Any;

use crate::app::App;
use crate::cross::*;
use crate::object::*;
use crate::render::Ctx;
use crate::draw::*;


pub fn rect(w: f32, h: f32) -> Shape {
    let w = w / 2.;
    let h = h / 2.;
    let verts = vec![
        Vertex::new(vec3(-w, -h, 0.), Color::new(1., 0., 0., 1.)),
        Vertex::new(vec3(w, -h, 0.), Color::new(0., 1., 0., 1.)),
        Vertex::new(vec3(w, h, 0.), Color::new(0., 1., 1., 1.)),
        Vertex::new(vec3(-w, h, 0.), Color::new(0., 0., 1., 1.)),
    ];
    let indis = vec![0, 1, 2, 2, 3, 0];
    Shape::new(Draw::new(verts, indis))
}

pub fn shape() -> Shape {
    Shape::new(Draw::new(Vec::new(), Vec::new()))
}


#[derive(Default)]
pub struct Group2d {
    pub object_list: HashMap<String, Box<dyn Object2d>>,
}
impl Group2d {
    pub fn add(&mut self, name: &str, object: impl Object2d) {
        self.object_list.insert(name.to_string(), Box::new(object));
    }
}
impl Object for Group2d {
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
impl Object2d for Group2d {}


#[derive(Default)]
pub struct Factory2d {
    pub object_list: Vec<Shape>,
}
impl Factory2d {
    pub fn add(&mut self, shape: Shape) {
        self.object_list.push(shape);
    }
}
impl Object for Factory2d {
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
impl Object2d for Factory2d {}


pub struct Shape {
    draw: Draw,
    pub modules: ModulesShape,

    // Transwofm
    pub position: LData<Vec2>,
    pub scale: LData<Vec2>,
    pub rotation: LData<f32>,
    pub depht: LData<f32>,
}
impl Shape {
    pub(crate) fn new(draw: Draw) -> Self {
        Self {
            draw,
            modules: ModulesShape::default(),
            position: LData::new(Vec2::ZERO),
            scale: LData::new(Vec2::ONE),
            rotation: LData::new(0.),
            depht: LData::new(0.),
        }
    }
    /*pub(crate) fn get_mat(&self) -> Mat4 {
        
    }*/
}
impl Object for Shape {
    fn update(&mut self, app: &App) {
        if self.modules.is_size() {
            let mut modules = take(&mut self.modules);
            modules.update(app, &self);
            self.modules = modules;
        }
    }
    fn draw(&mut self, ctx: &mut Ctx, mvp: &Mat4) {
        let position = *self.position.lock();
        let position = vec3(position.x, position.y, *self.depht.lock());
        let position = Mat4::from_translation(position);

        let scale = *self.scale.lock();
        let scale = vec3(scale.x, scale.y, 1.);
        let scale = Mat4::from_scale(scale);

        let rotation = *self.rotation.lock();
        let rotation = Mat4::from_rotation_z(rotation);

        let mvp = mvp * position * rotation * scale;
        self.draw.draw(ctx, mvp);
    }
}
impl Object2d for Shape {}


#[derive(Default)]
pub struct ModulesShape {
    module_list: Vec<Box<dyn ModuleShape>>,
    module_list_len: usize,
}
impl ModulesShape {
    pub fn add(&mut self, module: impl ModuleShape) {
        self.module_list.push(Box::new(module));
    }
    pub(crate) fn update(&mut self, app: &App, obj: &Shape) {
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
pub trait ModuleShape: Any + Sync + Send {
    fn ready(&mut self, app: &App, obj: &Shape);
    fn procces(&mut self, app: &App, obj: &Shape);
}


pub trait Object2d: Object {}

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
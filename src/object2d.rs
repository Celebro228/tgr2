use hashbrown::HashMap;

use crate::cross::*;
use crate::object::*;
use crate::render::Ctx;
use crate::draw::Draw;


pub struct Group2d {
    pub object_list: HashMap<String, Box<dyn Object2d>>,
}
impl Group2d {
    pub fn new() -> Self {
        Self {
            object_list: HashMap::new(),
        }
    }
    pub fn add(&mut self, name: &str, object: impl Object2d) {
        self.object_list.insert(name.to_string(), Box::new(object));
    }
}
impl Object for Group2d {
    fn update(&mut self) {
        
    }
    fn draw(&mut self, ctx: &mut Ctx) {
        
    }
}
impl Object2d for Group2d {}


pub struct Factory2d {
    pub object_list: Vec<Shape>,
}
impl Factory2d {
    pub fn new() -> Self {
        Self {
            object_list: Vec::new(),
        }
    }
    pub fn add(&mut self, shape: Shape) {
        self.object_list.push(shape);
    }
}
impl Object for Factory2d {
    fn update(&mut self) {
        
    }
    fn draw(&mut self, ctx: &mut Ctx) {
        
    }
}
impl Object2d for Factory2d {}


pub struct Shape {
    draw: Draw,

    // Transwofm
    pub position: LData<Vec2>,
    pub scale: LData<Vec2>,
    pub rotation: LData<f32>,
    pub depht: LData<f32>,
    
}
impl Shape {
    pub(crate) fn new() -> Self {
        Self {
            draw: Draw::new(),
            position: LData::new(Vec2::ZERO),
            scale: LData::new(Vec2::ONE),
            rotation: LData::new(0.),
            depht: LData::new(0.),
        }
    }
    /*pub(crate) fn get_mat(&self) -> Mat4 {
        let position = *self.position.lock();
        let position = vec3(position.x, position.y, *self.depht.lock());
        let position = Mat4::from_translation(position);

        let scale = *self.scale.lock();
        let scale = vec3(scale.x, scale.y, 1.);
        let scale = Mat4::from_scale(scale);

        let rotation = *self.rotation.lock();
        let rotation = Mat4::from_rotation_z(rotation);

        position * rotation * scale
    }*/
}
impl Object for Shape {
    fn update(&mut self) {
        
    }
    fn draw(&mut self, ctx: &mut Ctx) {
        
    }
}
impl Object2d for Shape {}


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
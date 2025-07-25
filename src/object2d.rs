use std::sync::{Mutex, Arc};
use hashbrown::HashMap;
use glam::{vec3, Mat4, Vec2};

use crate::draw::Vertex;


#[derive(Default)]
pub struct Objects2d {
    object_list: HashMap<String, Object2d>,
    object_add_list: Arc<Mutex<Vec<(String, Object2d)>>>
}

impl Objects2d {
    pub fn add(&self, name: &str, object: Object2d) {
        let mut object_add_list_lock = self.object_add_list
            .lock()
            .expect("Failed add object to object_add_list");
        object_add_list_lock.push((name.to_string(), object));
    }

    pub fn get(&self, name: &str) -> Option<&Object2d> {
        self.object_list.get(name)
    }

    pub(crate) fn update(&mut self) {
        let mut object_add_list_lock = self.object_add_list
            .lock()
            .expect("Failed get object add list for object list");

        for obj in object_add_list_lock.drain(..) {
            self.object_list.insert(obj.0, obj.1);
        }
    }

    pub(crate) fn get_draw(&self) -> Vec<(&Vec<Vertex>, &Vec<u16>, Mat4)> {
        let mut draw = Vec::new();

        for obj in &self.object_list {
            draw.push(obj.1.get_draw());
        }

        draw
    }
}


pub struct Object2d {
    verts: Vec<Vertex>,
    indis: Vec<u16>,

    // Transwofm
    position: Arc<Mutex<Vec2>>,
    scale: Arc<Mutex<Vec2>>,
    rotation: Arc<Mutex<f32>>,
}

impl Object2d {
    pub(crate) fn new(verts: Vec<Vertex>, indis: Vec<u16>) -> Self {
        Self {
            verts,
            indis,
            position: Arc::new(Mutex::new(Vec2::ZERO)),
            scale: Arc::new(Mutex::new(Vec2::ONE)),
            rotation: Arc::new(Mutex::new(0.)),
        }
    }

    pub(crate) fn get_draw(&self) -> (&Vec<Vertex>, &Vec<u16>, Mat4) {
        let position = self.position_get();
        let position = vec3(position.x, position.y, 0.);
        let position = Mat4::from_translation(position);

        let scale = self.scale_get();
        let scale = vec3(scale.x, scale.y, 1.);
        let scale = Mat4::from_scale(scale);
        
        let rotation = self.rotation_get();
        let rotation = Mat4::from_rotation_z(rotation);

        let mat = position * rotation * scale;

        (&self.verts, &self.indis, mat)
    }

    pub fn position_set(&self, v: Vec2) {
        let mut position_lock = self.position
            .lock()
            .expect("Failed set position of object2d");
        *position_lock = v;
    }

    pub fn position_get(&self) -> Vec2 {
        let position_lock = self.position
            .lock()
            .expect("Failed get position of object2d");
        position_lock.clone()
    }

    pub fn scale_set(&self, v: Vec2) {
        let mut scale_lock = self.scale
            .lock()
            .expect("Failed set scale of object2d");
        *scale_lock = v;
    }

    pub fn scale_get(&self) -> Vec2 {
        let scale_lock = self.scale
            .lock()
            .expect("Failed get scale of object2d");
        scale_lock.clone()
    }

    pub fn rotation_set(&self, v: f32) {
        let mut rotation_lock = self.rotation
            .lock()
            .expect("Failed set rotation of object2d");
        *rotation_lock = v;
    }

    pub fn rotation_get(&self) -> f32 {
        let rotation_lock = self.rotation
            .lock()
            .expect("Failed get rotation of object2d");
        rotation_lock.clone()
    }
}


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
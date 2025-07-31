use std::sync::{Mutex, Arc};
use hashbrown::HashMap;
use glam::{vec3, Mat4, Vec3};

use crate::draw::Vertex;


#[derive(Default)]
pub struct Objects3d {
    object_list: HashMap<String, Object3d>,
    object_add_list: Arc<Mutex<Vec<(String, Object3d)>>>
}

impl Objects3d {
    pub fn add(&self, name: &str, object: Object3d) {
        let mut object_add_list_lock = self.object_add_list
            .lock()
            .expect("Failed add object to object_add_list");
        object_add_list_lock.push((name.to_string(), object));
    }

    pub fn get(&self, name: &str) -> Option<&Object3d> {
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


pub struct Object3d {
    verts: Vec<Vertex>,
    indis: Vec<u16>,

    // Transwofm
    position: Arc<Mutex<Vec3>>,
    scale: Arc<Mutex<Vec3>>,
    rotation: Arc<Mutex<Vec3>>,
}

impl Object3d {
    pub(crate) fn new(verts: Vec<Vertex>, indis: Vec<u16>) -> Self {
        Self {
            verts,
            indis,
            position: Arc::new(Mutex::new(Vec3::ZERO)),
            scale: Arc::new(Mutex::new(Vec3::ONE)),
            rotation: Arc::new(Mutex::new(Vec3::ZERO)),
        }
    }

    pub(crate) fn get_draw(&self) -> (&Vec<Vertex>, &Vec<u16>, Mat4) {
        let position = self.position_get();
        let position = Mat4::from_translation(position);

        let scale = self.scale_get();
        let scale = Mat4::from_scale(scale);
        
        let rotation = self.rotation_get();
        let rot_x = Mat4::from_rotation_x(rotation.x);
        let rot_y = Mat4::from_rotation_y(rotation.y);
        let rot_z = Mat4::from_rotation_z(rotation.z);
        let rotation = rot_y * rot_x * rot_z;

        let mat = position * rotation * scale;
        (&self.verts, &self.indis, mat)
    }

    pub fn position_set(&self, v: Vec3) {
        let mut position_lock = self.position
            .lock()
            .expect("Failed set position of object3d");
        *position_lock = v;
    }

    pub fn position_get(&self) -> Vec3 {
        let position_lock = self.position
            .lock()
            .expect("Failed get position of object3d");
        position_lock.clone()
    }

    pub fn scale_set(&self, v: Vec3) {
        let mut scale_lock = self.scale
            .lock()
            .expect("Failed set scale of object3d");
        *scale_lock = v;
    }

    pub fn scale_get(&self) -> Vec3 {
        let scale_lock = self.scale
            .lock()
            .expect("Failed get scale of object3d");
        scale_lock.clone()
    }

    pub fn rotation_set(&self, v: Vec3) {
        let mut rotation_lock = self.rotation
            .lock()
            .expect("Failed set rotation of object3d");
        *rotation_lock = v;
    }

    pub fn rotation_get(&self) -> Vec3 {
        let rotation_lock = self.rotation
            .lock()
            .expect("Failed get rotation of object3d");
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
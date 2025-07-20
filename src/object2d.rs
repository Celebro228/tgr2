use std::sync::{Mutex, MutexGuard, Arc};

use glam::{Vec2, Vec3};
use hashbrown::HashMap;


#[derive(Default)]
pub struct Objects2d {
    object_list: HashMap<String, Object2d>,
    object_add_list: Arc<Mutex<Vec<(String, Object2d)>>>
}

impl Objects2d {
    pub fn add(&self, name: &str, object: Object2d) -> Obj2dRef {
        let mut object_add_list_lock = self.object_add_list
            .lock()
            .expect("Failed add object to object_add_list");
        object_add_list_lock.push((name.to_string(), object));

        Obj2dRef::new(object_add_list_lock)
    }

    pub fn get(&self, name: &str) -> Option<&Object2d> {
        self.object_list.get(name)
    }
}


pub struct Obj2dRef<'a> {
    object_add_list_lock: MutexGuard<'a, Vec<(String, Object2d)>>
}

impl<'a> Obj2dRef<'a> {
    pub(crate) fn new(mutex: MutexGuard<'a, Vec<(String, Object2d)>>) -> Self {
        Self {
            object_add_list_lock: mutex
        }
    }

    pub fn obj(&self) -> &Object2d {
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
}


#[derive(Default)]
pub struct Object2d {
    position: Arc<Mutex<Vec3>>,
    rotation: Arc<Mutex<Vec2>>,
    scale: Arc<Mutex<Vec2>>,
}

impl Object2d {
    pub fn position_set(&self, p: Vec3) {
        let mut position_lock = self.position
            .lock()
            .expect("Failed get position of object2d");
        *position_lock = p;
    }

    pub fn position_get(&self) -> Vec3 {
        let position_lock = self.position
            .lock()
            .expect("Failed get position of object2d");
        position_lock.clone()
    }
}
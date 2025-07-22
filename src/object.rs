use std::sync::{Mutex, Arc};
use hashbrown::HashMap;

use crate::draw::Draw;


#[derive(Default)]
pub struct Objects<T> {
    object_list: HashMap<String, Object<T>>,
    object_add_list: Arc<Mutex<Vec<(String, Object<T>)>>>
}

impl<T> Objects<T> {
    pub fn add(&self, name: &str, object: Object<T>) {
        let mut object_add_list_lock = self.object_add_list
            .lock()
            .expect("Failed add object to object_add_list");
        object_add_list_lock.push((name.to_string(), object));
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        match self.object_list.get(name) {
            Some(a) => Some(a.obj()),
            None => None,
        }
    }
}


pub struct Object<T> {
    draw: Draw,
    object_type: T,
}

impl<T> Object<T> {
    pub(crate) fn new(object_type: T, draw: Draw) -> Self {
        Self {
            object_type,
            draw,
        }
    }

    pub fn obj(&self) -> &T {
        &self.object_type
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
use std::sync::{Mutex, MutexGuard, Arc};
use hashbrown::HashMap;


#[derive(Default)]
pub struct Objects<T> {
    object_list: HashMap<String, T>,
    object_add_list: Arc<Mutex<Vec<(String, T)>>>
}

impl<T> Objects<T> {
    pub fn add(&self, name: &str, object: T) -> ObjRef<T> {
        let mut object_add_list_lock = self.object_add_list
            .lock()
            .expect("Failed add object to object_add_list");
        object_add_list_lock.push((name.to_string(), object));

        ObjRef::new(object_add_list_lock)
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.object_list.get(name)
    }
}


pub struct ObjRef<'a, T> {
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
}
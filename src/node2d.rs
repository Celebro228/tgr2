use crate::cross::*;
use crate::node::*;
use crate::draw::*;


pub fn rect(w: f32, h: f32) -> Object {
    let w = w / 2.;
    let h = h / 2.;
    let verts = vec![
        vec3(-w, -h, 0.),
        vec3(w, -h, 0.),
        vec3(w, h, 0.),
        vec3(-w, h, 0.),
    ];
    let indis = vec![0, 1, 2, 2, 3, 0];
    Object::new(Draw::new(verts, indis))
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
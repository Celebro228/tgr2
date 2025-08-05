use hashbrown::HashMap;

use crate::cross::*;
use crate::draw::Vertex;
#[derive(Default)]
pub struct Objects2d {
    object_list: HashMap<String, Object2d>,
    object_add_list: Data<Vec<(String, Object2d)>>,
}

impl Objects2d {
    pub fn add(&self, name: &str, object: Object2d) {
        let mut object_add_list_lock = self.object_add_list.lock();
        object_add_list_lock.push((name.to_string(), object));
    }

    pub fn get(&self, name: &str) -> Option<&Object2d> {
        self.object_list.get(name)
    }

    pub(crate) fn update(&mut self) {
        let mut object_add_list_lock = self.object_add_list.lock();

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
    pub position: Data<Vec2>,
    pub scale: Data<Vec2>,
    pub rotation: Data<f32>,
    pub depht: Data<f32>,
}

impl Object2d {
    pub(crate) fn new(verts: Vec<Vertex>, indis: Vec<u16>) -> Self {
        Self {
            verts,
            indis,
            position: Data::new(Vec2::ZERO),
            scale: Data::new(Vec2::ONE),
            rotation: Data::new(0.),
            depht: Data::new(0.),
        }
    }

    pub(crate) fn get_draw(&self) -> (&Vec<Vertex>, &Vec<u16>, Mat4) {
        let position = *self.position.lock();
        let position = vec3(position.x, position.y, *self.depht.lock());
        let position = Mat4::from_translation(position);

        let scale = *self.scale.lock();
        let scale = vec3(scale.x, scale.y, 1.);
        let scale = Mat4::from_scale(scale);

        let rotation = *self.rotation.lock();
        let rotation = Mat4::from_rotation_z(rotation);

        let mat = position * rotation * scale;

        (&self.verts, &self.indis, mat)
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
use hashbrown::HashMap;

use crate::cross::*;
use crate::draw::Vertex;
#[derive(Default)]
pub struct Objects3d {
    object_list: HashMap<String, Object3d>,
    object_add_list: Data<Vec<(String, Object3d)>>,
}

impl Objects3d {
    pub fn add(&self, name: &str, object: Object3d) {
        let mut object_add_list_lock = self.object_add_list.lock();
        object_add_list_lock.push((name.to_string(), object));
    }

    pub fn get(&self, name: &str) -> Option<&Object3d> {
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

pub struct Object3d {
    verts: Vec<Vertex>,
    indis: Vec<u16>,

    // Transwofm
    pub position: Data<Vec3>,
    pub scale: Data<Vec3>,
    pub rotation: Data<Vec3>,
}

impl Object3d {
    pub(crate) fn new(verts: Vec<Vertex>, indis: Vec<u16>) -> Self {
        Self {
            verts,
            indis,
            position: Data::new(Vec3::ZERO),
            scale: Data::new(Vec3::ONE),
            rotation: Data::new(Vec3::ZERO),
        }
    }

    pub(crate) fn get_draw(&self) -> (&Vec<Vertex>, &Vec<u16>, Mat4) {
        let position = *self.position.lock();
        let position = Mat4::from_translation(position);

        let scale = *self.scale.lock();
        let scale = Mat4::from_scale(scale);

        let rotation = *self.rotation.lock();
        let rot_x = Mat4::from_rotation_x(rotation.x);
        let rot_y = Mat4::from_rotation_y(rotation.y);
        let rot_z = Mat4::from_rotation_z(rotation.z);
        let rotation = rot_y * rot_x * rot_z;

        let mat = position * rotation * scale;
        (&self.verts, &self.indis, mat)
    }
}

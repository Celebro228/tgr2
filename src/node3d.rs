use crate::cross::*;
use crate::node::*;
use crate::draw::*;


pub fn cube(w: f32, h: f32, l: f32) -> Object {
    let w = w / 2.;
    let h = h / 2.;
    let l = l / 2.;
    let verts = vec![
        vec3(-w, -h, l), // 0
        vec3(w, -h, l), // 1
        vec3(w, h, l), // 2
        vec3(-w, h, l), // 3
        vec3(w, h, -l), // 4
        vec3(w, -h, -l), // 5
        vec3(-w, -h, -l), // 6
        vec3(-w, h, -l), // 7
    ];
    let indis = vec![
        0, 1, 2, 2, 3, 0,
        7, 6, 0, 0, 3, 7,
        0, 6, 5, 5, 1, 0,
        2, 1, 5, 5, 4, 2,
        7, 3, 2, 2, 4, 7,
        4, 5, 6, 6, 7, 4,
    ];
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


/*use hashbrown::HashMap;

use crate::cross::*;


pub struct Model {
    // Transwofm
    pub position: LData<Vec3>,
    pub scale: LData<Vec3>,
    pub rotation: LData<Vec3>,
}

impl Model {
    pub(crate) fn new() -> Self {
        Self {
            position: LData::new(Vec3::ZERO),
            scale: LData::new(Vec3::ONE),
            rotation: LData::new(Vec3::ZERO),
        }
    }

    pub(crate) fn get_draw(&self) -> Mat4 {
        let position = *self.position.lock();
        let position = Mat4::from_translation(position);

        let scale = *self.scale.lock();
        let scale = Mat4::from_scale(scale);

        let rotation = *self.rotation.lock();
        let rot_x = Mat4::from_rotation_x(rotation.x);
        let rot_y = Mat4::from_rotation_y(rotation.y);
        let rot_z = Mat4::from_rotation_z(rotation.z);
        let rotation = rot_y * rot_x * rot_z;

        position * rotation * scale
    }
}
*/
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
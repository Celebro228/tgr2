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


pub struct Camera2d {
    position: LData<Vec3>,
    zoom: LData<f32>,
}
impl Camera2d {
    pub fn new() -> Self {
        Self {
            position: LData::new(Vec3::ZERO),
            zoom: LData::new(1.),
        }
    }
    pub(crate) fn get_mat(&self, window: (f32, f32)) -> Mat4 {
        let zoom = *self.zoom.lock();
        let canvas = vec2(50., 50.) / zoom;
        let window = vec2(window.0, window.1) / 2.;
        let aspect_window = window.x / window.y;
        let aspect_canvas = canvas.x / canvas.y;
        let scale = canvas.x / (aspect_canvas / aspect_window);
        let view = Mat4::orthographic_rh_gl(
            -scale,
            scale,
            -canvas.y,
            canvas.y,
            -1.,
            1.
        );

        let position = *self.position.lock();
        let position = Mat4::from_translation(-position);

        view * position
    }
    pub fn position(&self) -> MutexGuard<'_, Vec3> {
        self.position.lock()
    }
    pub fn zoom(&self) -> MutexGuard<'_, f32> {
        self.zoom.lock()
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
pub(crate) use glam::Mat4;
pub use glam::{Vec2, Vec3, Vec4, vec2, vec3, vec4};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Default, Clone)]
pub struct Data<T> {
    data: Arc<Mutex<T>>,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.data.lock().unwrap()
    }

    pub fn set(&self, v: T) {
        *self.lock() = v;
    }

    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.lock().clone()
    }
}

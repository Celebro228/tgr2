pub(crate) use glam::Mat4;
pub use glam::{Vec2, Vec3, Vec4, vec2, vec3, vec4};
use std::sync::{atomic::*, Arc, Mutex, MutexGuard, OnceLock};


/// Local Data
#[derive(Default, Clone)]
pub struct LData<T> {
    data: Arc<Mutex<T>>,
}
impl<T> LData<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
        }
    }
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.data.lock().unwrap()
    }
}

/// Global Data
#[derive(Default)]
pub struct GData<T> {
    data: OnceLock<Mutex<T>>,
    buffer: T,
}
impl<T: Clone> GData<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: OnceLock::new(),
            buffer: data,
        }
    }
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.data.get_or_init(|| {
            Mutex::new(self.buffer.clone())
        }).lock().unwrap()
    }
}


const ORD_STATUS: Ordering = Ordering::Relaxed;

/// Atomic Count
#[derive(Default)]
pub struct ACount {
    data: AtomicIsize,
}
impl ACount {
    pub const fn new(data: isize) -> Self {
        Self {
            data: AtomicIsize::new(data),
        }
    }
    pub fn get(&self) -> isize {
        self.data.load(ORD_STATUS)
    }
    pub fn set(&self, v: isize) {
        self.data.store(v, ORD_STATUS);
    }
    pub fn add(&self, v: isize) {
        self.data.fetch_add(v, ORD_STATUS);
    }
    pub fn sub(&self, v: isize) {
        self.data.fetch_sub(v, ORD_STATUS);
    }
}

/// Atomic Bool
#[derive(Default)]
pub struct ABool {
    data: AtomicBool,
}
impl ABool {
    pub const fn new(data: bool) -> Self {
        Self {
            data: AtomicBool::new(data),
        }
    }
    pub fn get(&self) -> bool {
        self.data.load(ORD_STATUS)
    }
    pub fn set(&self, v: bool) {
        self.data.store(v, ORD_STATUS);
    }
    pub fn and(&self, v: bool) {
        self.data.fetch_and(v, ORD_STATUS);
    }
    pub fn or(&self, v: bool) {
        self.data.fetch_or(v, ORD_STATUS);
    }
    pub fn not(&self) {
        self.data.fetch_not(ORD_STATUS);
    }
}
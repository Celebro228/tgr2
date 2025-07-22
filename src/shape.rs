use std::sync::{Mutex, Arc};
use glam::Vec2;

use crate::object::Object;
use crate::draw::Draw;


pub fn circle(r: f32) -> Object<Shape> {
    Object::new(Shape::default(), Draw{})
}

pub fn rect(w: f32, h: f32) -> Object<Shape> {
    Object::new(Shape::default(), Draw{})
}


#[derive(Default)]
pub struct Shape {
    // Transform
    position: Arc<Mutex<Vec2>>,
    rotation: Arc<Mutex<Vec2>>,
    scale: Arc<Mutex<Vec2>>,
}

impl Shape {
    pub fn position_set(&self, v: Vec2) {
        let mut position_lock = self.position
            .lock()
            .expect("Failed set position of object2d");
        *position_lock = v;
    }

    pub fn position_get(&self) -> Vec2 {
        let position_lock = self.position
            .lock()
            .expect("Failed get position of object2d");
        position_lock.clone()
    }

    pub fn rotation_set(&self, v: Vec2) {
        let mut rotation_lock = self.rotation
            .lock()
            .expect("Failed set rotation of object2d");
        *rotation_lock = v;
    }

    pub fn rotation_get(&self) -> Vec2 {
        let rotation_lock = self.rotation
            .lock()
            .expect("Failed get rotation of object2d");
        rotation_lock.clone()
    }

    pub fn scale_set(&self, v: Vec2) {
        let mut scale_lock = self.scale
            .lock()
            .expect("Failed set scale of object2d");
        *scale_lock = v;
    }

    pub fn scale_get(&self) -> Vec2 {
        let scale_lock = self.scale
            .lock()
            .expect("Failed get scale of object2d");
        scale_lock.clone()
    }
}
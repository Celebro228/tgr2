use glam::Mat4;

use crate::draw::Vertex;
use crate::object2d::Objects2d;
use crate::info::Info;


pub struct App {
    pub objects2d: Objects2d,
    pub info: Info,
}

impl App {
    pub(crate) fn new(time: f64) -> Self {
        Self {
            objects2d: Objects2d::default(),
            info: Info::new(time),
        }
    }

    pub(crate) fn pre_update(&mut self, time: f64) {
        self.info.update(time);
    }

    pub(crate) fn post_update(&mut self) {
        self.objects2d.update();
    }

    pub(crate) fn draw(&self) -> Vec<(&Vec<Vertex>, &Vec<u16>, Mat4)> {
        self.objects2d.get_draw()
    }
}
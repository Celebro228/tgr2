use crate::info::Info;
use crate::object::Object;
use crate::object2d::Group2d;
use crate::cross::*;


pub struct App {
    pub info: Info,
    pub objects2d: Group2d,
}

impl App {
    pub(crate) fn new(time: f64) -> Self {
        Self {
            info: Info::new(time),
            objects2d: Group2d::default(),
        }
    }

    pub(crate) fn pre_update(&mut self, time: f64) {
        self.info.update(time);
    }

    pub(crate) fn post_update(&mut self) {
        let mut objects2d = take(&mut self.objects2d);
        objects2d.update(&self);
        self.objects2d = objects2d;
    }
}
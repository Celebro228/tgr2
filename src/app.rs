use crate::info::Info;
use crate::object2d::Group2d;


pub struct App {
    pub info: Info,
    pub objects2d: Group2d,
}

impl App {
    pub(crate) fn new(time: f64) -> Self {
        Self {
            info: Info::new(time),
            objects2d: Group2d::new(),
        }
    }

    pub(crate) fn pre_update(&mut self, time: f64) {
        self.info.update(time);
    }

    pub(crate) fn post_update(&mut self) {
        
    }
}
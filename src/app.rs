use crate::info::Info;


pub struct App {
    pub info: Info,
}

impl App {
    pub(crate) fn new(time: f64) -> Self {
        Self {
            info: Info::new(time),
        }
    }

    pub(crate) fn pre_update(&mut self, time: f64) {
        self.info.update(time);
    }

    pub(crate) fn post_update(&mut self) {
        
    }
}
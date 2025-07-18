use std::any::Any;
use std::mem;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::app::App;


pub(crate) struct Modules {
    module_list: Vec<Box<dyn Module>>,
}

impl Modules {
    pub(crate) fn new() -> Self {
        Self {
            module_list: Vec::new(),
        }
    }

    pub(crate) fn add_module(&mut self, app: &App, mut module: impl Module) {
        module.ready(app);
        self.module_list.push(Box::new(module));
    }

    pub(crate) fn update(&mut self, app: &mut App) -> App {
        let app = mem::take(app);

        self.module_list.par_iter_mut().for_each(|module| {
            module.procces(&app);
        });

        app
    }
}


pub trait Module: Any + Sync + Send {
    fn ready(&mut self, _app: &App) {}
    fn procces(&mut self, _app: &App) {}
}
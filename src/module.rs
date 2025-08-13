use std::any::Any;

use crate::app::App;
use crate::cross::*;

/*#[derive(Default)]
pub(crate) struct Modules {
    module_list: Vec<Box<dyn Module>>,
}

impl Modules {
    pub(crate) fn add_module(&mut self, app: &App, mut module: impl Module) {
        module.ready(app);
        self.module_list.push(Box::new(module));
    }

    pub(crate) fn update(&mut self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        let module_list_iter = self.module_list.par_iter_mut();
        #[cfg(target_arch = "wasm32")]
        let module_list_iter = self.module_list.iter_mut();

        module_list_iter.for_each(|module| {
            module.procces(&app);
        });
    }
}*/


#[derive(Default)]
pub struct ModulesEngine {
    module_list: Vec<Box<dyn ModuleEngine>>,
    module_list_len: usize,
}
impl ModulesEngine {
    pub(crate) fn update(&mut self, app: &mut App) {
        for module in &mut self.module_list[self.module_list_len..] {
            module.ready(app);
        }
        self.module_list_len = self.module_list.len();

        for module in &mut self.module_list {
            module.procces(app);
        }
    }
    pub fn add(&mut self, module: impl ModuleEngine) {
        self.module_list.push(Box::new(module));
    }
}

pub trait ModuleEngine: Any + Sync + Send {
    fn ready(&mut self, _app: &mut App) {}
    fn procces(&mut self, _app: &mut App) {}
}


#[derive(Default)]
pub struct ModulesObject {
    module_list: Vec<Box<dyn Module>>,
}
impl ModulesObject {
    pub fn add(&mut self, module: impl Module) {
        self.module_list.push(Box::new(module));
    }
    pub(crate) fn update(&mut self, app: &App) {
        cross_iter(&mut self.module_list).for_each(|module| {
            module.procces(app);
        });
    }
}

pub trait Module: Any + Sync + Send {
    fn ready(&mut self, _app: &App) {}
    fn procces(&mut self, _app: &App) {}
}
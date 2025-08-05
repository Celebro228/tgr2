#[cfg(not(target_arch = "wasm32"))]
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::app::App;

#[derive(Default)]
pub(crate) struct Modules {
    module_list: Vec<Box<dyn Module>>,
}

impl Modules {
    pub(crate) fn add_module(&mut self, app: &App, mut module: impl Module) {
        module.ready(app);
        self.module_list.push(Box::new(module));
    }

    pub(crate) fn update(&mut self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        let module_list_iter = self.module_list.iter_mut();
        #[cfg(not(target_arch = "wasm32"))]
        let module_list_iter = self.module_list.par_iter_mut();

        module_list_iter.for_each(|module| {
            module.procces(&app);
        });
    }
}

use std::any::Any;
pub trait Module: Any + Sync + Send {
    fn ready(&mut self, _app: &App) {}
    fn procces(&mut self, _app: &App) {}
}

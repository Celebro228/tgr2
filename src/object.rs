use std::any::Any;
use crate::render::Ctx;
use crate::app::App;


pub trait Object: Any + Sync + Send {
    fn update(&mut self, app: &App);
    fn draw(&mut self, ctx: &mut Ctx);
}


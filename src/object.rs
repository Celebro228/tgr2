use std::any::Any;
use crate::render::Ctx;


pub trait Object: Any + Sync + Send {
    fn update(&mut self);
    fn draw(&mut self, ctx: &mut Ctx);
}


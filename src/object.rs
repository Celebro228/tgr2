use std::any::Any;
use crate::render::Ctx;
use crate::app::App;
use crate::cross::Mat4;


pub trait Object: Any + Sync + Send {
    fn update(&mut self, app: &App);
    fn draw(&mut self, ctx: &mut Ctx, mvp: &Mat4);
}


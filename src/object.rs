use crate::render::Ctx;


pub(crate) trait Object {
    fn update(&mut self);
    fn draw(&mut self, ctx: &mut Ctx);
}


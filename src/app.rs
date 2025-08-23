use crate::info::Info;
use crate::node::*;
use crate::cross::*;
use crate::event::*;
use crate::render::Ctx;


pub struct App {
    pub info: Info,
    pub events: Events,
    pub objects2d: Group,
    pub objects3d: Group,
}

impl App {
    pub(crate) fn new(time: f64) -> Self {
        Self {
            info: Info::new(time),
            events: Events::default(),
            objects2d: Group::default(),
            objects3d: Group::default(),
        }
    }

    pub(crate) fn pre_update(&mut self, time: f64) {
        self.info.update(time);
    }

    pub(crate) fn post_update(&mut self) {
        let mut objects2d = take(&mut self.objects2d);
        objects2d.update(&self);
        self.objects2d = objects2d;

        let mut objects3d = take(&mut self.objects3d);
        objects3d.update(&self);
        self.objects3d = objects3d;

        self.events.clear_active_events();
    }

    pub(crate) fn draw_3d(&mut self, ctx: &mut Ctx) {
        let mvp = ctx.mvp_3d();
        self.objects3d.draw(ctx, &mvp);
    }

    pub(crate) fn draw_2d(&mut self, ctx: &mut Ctx) {
        let mvp = ctx.mvp_2d();
        self.objects2d.draw(ctx, &mvp);
    }

    pub(crate) fn event_send_set(&mut self, event: EventChange) {
        self.events.event_set(event);
    }
}
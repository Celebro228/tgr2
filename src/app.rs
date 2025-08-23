use crate::info::Info;
use crate::node::*;
use crate::cross::*;
use crate::event::*;
use crate::render::Ctx;


pub struct App {
    pub info: Info,
    pub events: Events,
    pub node2d: Group,
    pub node3d: Group,
}

impl App {
    pub(crate) fn new(time: f64) -> Self {
        Self {
            info: Info::new(time),
            events: Events::default(),
            node2d: Group::default(),
            node3d: Group::default(),
        }
    }

    pub(crate) fn pre_update(&mut self, time: f64) {
        self.info.update(time);
    }

    pub(crate) fn post_update(&mut self) {
        let mut node2d = take(&mut self.node2d);
        node2d.update(&self);
        self.node2d = node2d;

        let mut node3d = take(&mut self.node3d);
        node3d.update(&self);
        self.node3d = node3d;

        self.events.clear_active_events();
    }

    pub(crate) fn draw_3d(&mut self, ctx: &mut Ctx) {
        let mvp = ctx.mvp_3d();
        self.node3d.draw(ctx, &mvp);
    }

    pub(crate) fn draw_2d(&mut self, ctx: &mut Ctx) {
        let mvp = ctx.mvp_2d();
        self.node2d.draw(ctx, &mvp);
    }

    pub(crate) fn event_send_set(&mut self, event: EventChange) {
        self.events.event_set(event);
    }
}
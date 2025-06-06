use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::Wgpu;


pub(crate) struct Window {
    window_atr: WindowAttributes,
    wgpu: Option<Wgpu>,
}
impl Window {
    pub fn new(window_atr: WindowAttributes) -> Self {
        Self {
            window_atr,
            wgpu: None,
        }
    }
}
impl ApplicationHandler for Window {
    // При старте
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(self.window_atr.clone()).unwrap();
        self.wgpu = Some(Wgpu::new(window));
    }

    // События (клавищи, системные кнопки и т. д.)
    fn window_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            _window_id: WindowId,
            event: WindowEvent,
        ) {
            let wgpu = self.wgpu.as_mut().unwrap();

            match &event {
                // Каждый кадр
                WindowEvent::RedrawRequested => {
                    // Новый кадр
                    wgpu.window.request_redraw();

                    wgpu.render();
                }

                WindowEvent::Resized(phys_size) => {
                    wgpu.resize(*phys_size);
                }

                _ => {}
            }
    }
}
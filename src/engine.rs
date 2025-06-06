use winit::{
    window::WindowAttributes,
    event_loop::EventLoop,
};

use crate::Window;


pub struct Engine {
    title: String,
}
impl Engine {
    pub fn new(title: &str) -> Self {
        Self { 
            title: title.to_string(),
        }
    }

    pub fn run(self) {
        // Настройки окна
        let window_atr = WindowAttributes::default().with_title(self.title);

        // Создание окна
        let mut window = Window::new(window_atr);
        EventLoop::new().unwrap().run_app(&mut window).unwrap();
    }
}
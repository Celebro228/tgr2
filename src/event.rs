pub use miniquad::KeyCode;
use miniquad::window;
use hashbrown::HashSet;
use crate::cross::Vec2;


#[derive(Default)]
pub struct Events {
    // Keyboard
    key_pressed: HashSet<KeyCode>,
    key_press: HashSet<KeyCode>,
    key_release: HashSet<KeyCode>,

    // Mouse
    pub mouse: Vec2,
    pub mouse_delta: Vec2,
}
impl Events {
    pub(crate) fn event_set(&mut self, event: EventChange) {
        match event {
            EventChange::Press(keycode) => {
                self.key_press.insert(keycode);
                self.key_pressed.insert(keycode);
            }
            EventChange::Release(keycode) => {
                self.key_release.insert(keycode);
                self.key_pressed.remove(&keycode);
            }
            EventChange::Mouse(mouse) => {
                self.mouse = mouse;
            }
            EventChange::MouseDelta(mouse_delta) => {
                self.mouse_delta = mouse_delta;
            }
        }
    }

    pub(crate) fn clear_active_events(&mut self) {
        self.key_press.clear();
        self.key_release.clear();
        self.mouse_delta = Vec2::ZERO;
    }

    pub fn is_key_press(&self, keycode: KeyCode) -> bool {
        self.key_press.contains(&keycode)
    }

    pub fn is_key_release(&self, keycode: KeyCode) -> bool {
        self.key_release.contains(&keycode)
    }

    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.key_pressed.contains(&keycode)
    }

    /// Show screen keyboard (more often for mobile device)
    pub fn keyboard_show(&self, shown: bool) {
        window::show_keyboard(shown);
    }

    pub fn mouse_show(&self, shown: bool) {
        window::show_mouse(shown);
        window::set_cursor_grab(!shown);
    }
}


pub(crate) enum EventChange {
    Press(KeyCode),
    Release(KeyCode),
    Mouse(Vec2),
    MouseDelta(Vec2),
}
pub use miniquad::KeyCode;
use hashbrown::HashSet;


#[derive(Default)]
pub struct Events {
    key_pressed: HashSet<KeyCode>,
    key_press: HashSet<KeyCode>,
    key_release: HashSet<KeyCode>,
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
        }
    }

    pub(crate) fn clear_active_events(&mut self) {
        self.key_press.clear();
        self.key_release.clear();
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
}


pub(crate) enum EventChange {
    Press(KeyCode),
    Release(KeyCode),
}
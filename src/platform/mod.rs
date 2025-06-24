use crate::resources::input::Input;
use std::sync::{Arc, Mutex};

/// Định nghĩa wrapper để triển khai InputCallback cho Arc<Mutex<Input>>
pub struct InputCallbackArc {
    pub inner: Arc<Mutex<Input>>,
}

impl minifb::InputCallback for InputCallbackArc {
    fn add_char(&mut self, uni_char: u32) {
        self.inner.lock().unwrap().add_char(uni_char);
    }
} 
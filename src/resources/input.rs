use crate::components::core::Status;
use crate::resources::font::FontResource;

#[derive(Default)]
pub struct Mouse {
    pub position: (f32, f32),
    pub pressed: bool,
}

#[derive(Default)]
pub struct Keyboard {
    pub key: Option<char>, // phím ký tự vừa được nhấn
    pub chars: String, // chuỗi ký tự nhập vào frame này
    pub enter: bool,
    pub escape: bool,
    pub backspace: bool,
    pub e: bool,
}

#[derive(Default)]
pub struct Time {
    pub now: u64,
}

#[derive(Default)]
pub struct Filter {
    pub text: Option<String>,
    pub status: Option<Status>,
    pub overdue: bool,
}

pub struct Input {
    pub chars: Vec<char>,
}

unsafe impl Send for Input {}
unsafe impl Sync for Input {}

impl Input {
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }
    pub fn take_chars(&mut self) -> Vec<char> {
        let chars = self.chars.clone();
        self.chars.clear();
        chars
    }
}

impl minifb::InputCallback for Input {
    fn add_char(&mut self, uni_char: u32) {
        if let Some(character) = std::char::from_u32(uni_char) {
            self.chars.push(character);
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

pub mod mod_rs {
    use super::*;
    #[derive(Default)]
    pub struct Resources {
        pub mouse: Mouse,
        pub keyboard: Keyboard,
        pub time: Time,
        pub filter: Filter,
        pub framebuffer: Option<(*mut u32, usize, usize)>,
        pub font: Option<FontResource>,
    }
}

use crate::task::components::Status;
use crate::resources::font::FontResource;

#[allow(dead_code)]
pub struct Mouse {
    pub position: (f32, f32),
    pub pressed: bool,
}

#[allow(dead_code)]
pub struct Keyboard {
    pub key: Option<char>, // phím ký tự vừa được nhấn
    pub chars: String, // chuỗi ký tự nhập vào frame này
    pub enter: bool,
    pub escape: bool,
    pub backspace: bool,
    pub e: bool,
}

#[allow(dead_code)]
pub struct Time {
    pub now: u64,
}

#[allow(dead_code)]
pub struct Filter {
    pub text: Option<String>,
    pub status: Option<Status>,
    pub overdue: bool,
    pub owner: Option<usize>, // Entity ID của owner để lọc
}

/// Input handler cho minifb, lưu trữ các ký tự được nhập vào.
pub struct Input {
    chars: Vec<char>,
}

unsafe impl Send for Input {}
unsafe impl Sync for Input {}

impl Input {
    pub fn new() -> Self {
        Self { chars: vec![] }
    }
    
    pub fn add_char(&mut self, uni_char: u32) {
        if let Some(ch) = char::from_u32(uni_char) {
            self.chars.push(ch);
        }
    }
    
    pub fn take_chars(&mut self) -> Vec<char> {
        let chars = self.chars.clone();
        self.chars.clear();
        chars
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl minifb::InputCallback for Input {
    fn add_char(&mut self, uni_char: u32) {
        if let Some(character) = std::char::from_u32(uni_char) {
            self.chars.push(character);
        }
    }
}

pub mod mod_rs {
    use super::*;
    #[allow(dead_code)]
    pub struct Resources {
        pub mouse: Mouse,
        pub keyboard: Keyboard,
        pub time: Time,
        pub filter: Filter,
        pub framebuffer: Option<(*mut u32, usize, usize)>,
        pub font: Option<FontResource>,
        pub session: Option<Session>,
    }
}

/// Resource quản lý phiên làm việc của người dùng hiện tại.
#[allow(dead_code)]
pub struct Session {
    pub user: usize, // Entity ID của người dùng hiện tại
}

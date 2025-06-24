use crate::user::components::User;
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
    pub status: Option<crate::task::components::Status>,
    pub overdue: bool,
    pub owner: Option<usize>, // Entity ID của owner để lọc
}

/// Resource quản lý phiên làm việc của người dùng hiện tại.
pub struct Session {
    pub user: usize, // Entity ID của người dùng hiện tại
}

#[derive(Default)]
pub struct Resources {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
    pub time: Time,
    pub filter: Filter,
    pub framebuffer: Option<(*mut u32, usize, usize)>,
    pub font: Option<FontResource>,
    pub session: Option<Session>,
}

pub mod font;
pub mod input; 
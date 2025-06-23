#[derive(Default)]
pub struct Mouse {
    pub position: (f32, f32),
    pub pressed: bool,
}

#[derive(Default)]
pub struct Keyboard {
    pub key: Option<char>, // phím ký tự vừa được nhấn
    pub enter: bool,
    pub escape: bool,
    pub backspace: bool,
    pub e: bool,
}

#[derive(Default)]
pub struct Time {
    pub now: u64,
}

pub mod mod_rs {
    use super::*;
    #[derive(Default)]
    pub struct Resources {
        pub mouse: Mouse,
        pub keyboard: Keyboard,
        pub time: Time,
    }
}

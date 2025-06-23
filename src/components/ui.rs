use crate::components::traits::Layoutable;

/// Component lưu trữ thông tin vị trí và kích thước của entity trên UI.
#[derive(Clone, Copy)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Layoutable for Bounds {
    type Node = Self;
    fn node(&self) -> &Self::Node {
        self
    }
}

// pub struct Hover;
// pub use Hover as UiHover;

/// Component lưu trữ thông tin style (màu sắc) cho entity trên UI.
pub struct Style {
    pub color: &'static str,
}

/// Tag component đánh dấu một entity là container layout
pub struct Container;

/// Dòng/chữ dọc cho container
#[derive(Clone, Copy)]
pub enum Flow {
    Row,
    Column,
}

/// Căn chỉnh theo trục phụ
#[derive(Clone, Copy)]
pub enum Align {
    Start,
    Center,
    End,
}

/// Phân phối không gian theo trục chính
#[derive(Clone, Copy)]
pub enum Justify {
    Start,
    Center,
    End,
    Between,
}

/// Component Button, chứa nhãn nút bấm
pub struct Button(pub String);

use crate::components::traits::Layoutable;

/// Component lưu trữ thông tin vị trí và kích thước của entity trên UI.
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

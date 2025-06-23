use crate::components::traits::Layoutable;

pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Layoutable for Bounds {
    type Node = Self;
    fn node(&self) -> &Self::Node { self }
}

// pub struct Hover;
// pub use Hover as UiHover;

pub struct Style {
    pub color: &'static str,
}

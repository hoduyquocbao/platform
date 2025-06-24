// UI Components
/// Component lưu trữ kích thước và vị trí của entity.
#[derive(Clone, Copy)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Component lưu trữ style (màu sắc) của entity.
pub struct Style {
    pub color: String,
}

/// Component đánh dấu entity được hiển thị trên UI.
pub struct Visible;

/// Component đánh dấu entity đang được chọn.
pub struct Selected;

/// Component đánh dấu entity đang ở chế độ chỉnh sửa.
pub struct Editing;

// Interaction State Components
/// Component đánh dấu entity đang được hover chuột.
pub struct Hover;

/// Component đánh dấu entity đang active (tương tác).
pub struct Active;

/// Component đánh dấu entity vừa được click.
pub struct Click;

/// Component đánh dấu entity bị vô hiệu hóa (disabled).
pub struct Disabled;

/// Component đánh dấu entity là container.
pub struct Container;

/// Component định nghĩa hướng sắp xếp của container.
pub enum Flow {
    Row,
    Column,
}

/// Component định nghĩa căn chỉnh theo trục chính.
pub enum Align {
    Start,
    Center,
    End,
}

/// Component định nghĩa căn chỉnh theo trục phụ.
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

/// Component đánh dấu entity là button.
pub struct Button;

// Traits
/// Trait cho các component có thể render.
pub trait Renderable {
    type Object;
    fn object(&self) -> &Self::Object;
}

/// Trait cho các component có thể tương tác.
pub trait Interactable {
    type Target;
    fn target(&self) -> &Self::Target;
}

// Implement traits for UI components
impl Renderable for Visible {
    type Object = Self;
    fn object(&self) -> &Self::Object {
        self
    }
}

impl Interactable for Selected {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

impl Interactable for Editing {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

impl Interactable for Hover {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

impl Interactable for Active {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

impl Interactable for Click {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

impl Interactable for Disabled {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
} 
use crate::components::traits::{Interactable, Renderable};

// Core & State Components
/// Component lưu trữ văn bản cho một entity (ví dụ: tên task).
pub struct Text {
    pub value: String,
}

impl Renderable for Text {
    type Object = Self;
    fn object(&self) -> &Self::Object {
        self
    }
}

/// Component đánh dấu trạng thái TODO/DONE cho entity.
pub struct Status;
impl Renderable for Status {
    type Object = Self;
    fn object(&self) -> &Self::Object {
        self
    }
}

/// Component đánh dấu mức độ ưu tiên cho entity.
pub struct Priority;
impl Renderable for Priority {
    type Object = Self;
    fn object(&self) -> &Self::Object {
        self
    }
}

/// Component đánh dấu entity đang được chọn.
pub struct Selected;
impl Interactable for Selected {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

/// Component đánh dấu entity đang ở chế độ chỉnh sửa.
pub struct Editing;
impl Interactable for Editing {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

/// Component đánh dấu entity được hiển thị trên UI.
pub struct Visible;
impl Renderable for Visible {
    type Object = Self;
    fn object(&self) -> &Self::Object {
        self
    }
}

/// Component đánh dấu entity bị thu gọn (collapsed) trong cây phân cấp.
pub struct Collapsed;

// Interaction State Components
/// Component đánh dấu entity đang được hover chuột.
pub struct Hover;
impl Interactable for Hover {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

/// Component đánh dấu entity đang active (tương tác).
pub struct Active;
impl Interactable for Active {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

/// Component đánh dấu entity vừa được click.
pub struct Click;
impl Interactable for Click {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

/// Component đánh dấu entity có thay đổi cần lưu (dirty).
pub struct Dirty;
impl Interactable for Dirty {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

/// Component đánh dấu entity bị vô hiệu hóa (disabled).
pub struct Disabled;
impl Interactable for Disabled {
    type Target = Self;
    fn target(&self) -> &Self::Target {
        self
    }
}

/// Component đánh dấu entity cần được tạo mới (dùng cho hệ thống Create).
pub struct Create; // trùng với Command::Create
/// Component đánh dấu entity cần được xóa (dùng cho hệ thống Delete).
pub struct Delete; // trùng với Command::Delete

/// Component lưu entity cha của một entity (nếu có).
pub struct Parent(pub usize);
/// Component lưu danh sách entity con của một entity.
pub struct Children(pub Vec<usize>);

/// Component lưu timestamp ngày hết hạn của entity (epoch seconds).
pub struct Due(pub u64);
/// Component đánh dấu entity đang ở chế độ nhập ngày hết hạn.
pub struct Scheduling;

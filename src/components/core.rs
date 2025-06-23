// Core & State Components
pub struct Text {
    pub value: String,
}

pub struct Status;

pub struct Priority;

pub struct Selected;

pub struct Editing;

pub struct Visible;

// Interaction State Components
pub struct Hover;

pub struct Active;

pub struct Click;

pub struct Dirty;

pub struct Disabled;

pub struct Create; // trùng với Command::Create
pub struct Delete; // trùng với Command::Delete

/// Component lưu entity cha của một entity (nếu có).
pub struct Parent(pub usize);
/// Component lưu danh sách entity con của một entity.
pub struct Children(pub Vec<usize>);

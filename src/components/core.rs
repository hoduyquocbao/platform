use crate::components::traits::{Renderable, Interactable};

// Core & State Components
pub struct Text {
    pub value: String,
}

impl Renderable for Text {
    type Object = Self;
    fn object(&self) -> &Self::Object { self }
}

pub struct Status;
impl Renderable for Status {
    type Object = Self;
    fn object(&self) -> &Self::Object { self }
}

pub struct Priority;
impl Renderable for Priority {
    type Object = Self;
    fn object(&self) -> &Self::Object { self }
}

pub struct Selected;
impl Interactable for Selected {
    type Target = Self;
    fn target(&self) -> &Self::Target { self }
}

pub struct Editing;
impl Interactable for Editing {
    type Target = Self;
    fn target(&self) -> &Self::Target { self }
}

pub struct Visible;
impl Renderable for Visible {
    type Object = Self;
    fn object(&self) -> &Self::Object { self }
}

pub struct Collapsed;

// Interaction State Components
pub struct Hover;
impl Interactable for Hover {
    type Target = Self;
    fn target(&self) -> &Self::Target { self }
}

pub struct Active;
impl Interactable for Active {
    type Target = Self;
    fn target(&self) -> &Self::Target { self }
}

pub struct Click;
impl Interactable for Click {
    type Target = Self;
    fn target(&self) -> &Self::Target { self }
}

pub struct Dirty;
impl Interactable for Dirty {
    type Target = Self;
    fn target(&self) -> &Self::Target { self }
}

pub struct Disabled;
impl Interactable for Disabled {
    type Target = Self;
    fn target(&self) -> &Self::Target { self }
}

pub struct Create; // trùng với Command::Create
pub struct Delete; // trùng với Command::Delete

/// Component lưu entity cha của một entity (nếu có).
pub struct Parent(pub usize);
/// Component lưu danh sách entity con của một entity.
pub struct Children(pub Vec<usize>);

pub struct Due(pub u64);
pub struct Scheduling;

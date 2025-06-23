/// Trait cho phép một đối tượng có thể được layout (bố trí vị trí trên UI).
pub trait Layoutable {
    type Node;
    #[allow(dead_code)]
    fn node(&self) -> &Self::Node;
}

/// Trait cho phép một đối tượng có thể được render (hiển thị trên UI).
pub trait Renderable {
    type Object;
    fn object(&self) -> &Self::Object;
}

/// Trait cho phép một đối tượng có thể được tương tác (chọn, chỉnh sửa, v.v.).
pub trait Interactable {
    type Target;
    #[allow(dead_code)]
    fn target(&self) -> &Self::Target;
}

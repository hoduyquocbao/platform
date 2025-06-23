/// Trait cho phép một đối tượng có thể được layout.
pub trait Layoutable {
    type Node;
    #[allow(dead_code)]
    fn node(&self) -> &Self::Node;
}

/// Trait cho phép một đối tượng có thể được render.
pub trait Renderable {
    type Object;
    fn object(&self) -> &Self::Object;
}

/// Trait cho phép một đối tượng có thể được tương tác.
pub trait Interactable {
    type Target;
    #[allow(dead_code)]
    fn target(&self) -> &Self::Target;
} 
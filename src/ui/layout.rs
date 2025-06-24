use crate::World;
use crate::engine::System;
use crate::resources::Resources;
use crate::ui::components::Visible;

/// System chịu trách nhiệm tính toán layout cho các entity có component Container.
pub struct LayoutSystem;

impl System for LayoutSystem {
    /// Thực thi tính toán layout cho mỗi entity có component Container, cập nhật Bounds cho các con.
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        // TODO: Implement layout logic
        // Hiện tại chỉ đơn giản là đánh dấu Visible cho các entity có Container
        for id in 0..world.entity_count {
            if world.container[id].is_some() {
                world.visibles[id] = Some(Visible);
            }
        }
    }
}

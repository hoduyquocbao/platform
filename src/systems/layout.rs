use crate::World;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

/// Hệ thống sắp xếp các entity có Visible và Bounds thành danh sách dọc.
pub struct Layout;

impl System for Layout {
    /// Cập nhật thuộc tính y cho mỗi entity để xếp dọc, mỗi mục cách nhau một khoảng.
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        let mut y = 0.0;
        let spacing = 40.0;
        for id in 0..world.entity_count {
            if world.visibles[id].is_some() && world.bounds[id].is_some() {
                if let Some(bounds) = &mut world.bounds[id] {
                    bounds.y = y;
                    y += spacing;
                }
            }
        }
    }
}

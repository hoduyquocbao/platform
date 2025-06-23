use crate::World;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;
use crate::components::core::*;

/// Hệ thống sắp xếp các entity theo cấu trúc phân cấp, cập nhật x (thụt lề) và y.
pub struct Layout;

impl Layout {
    /// Duyệt cây entity theo quan hệ Parent/Children, cập nhật x/y theo depth và thứ tự.
    fn layout_entity(world: &mut World, id: usize, depth: usize, y: &mut f32, spacing: f32, indent: f32) {
        if world.visibles[id].is_some() && world.bounds[id].is_some() {
            if let Some(bounds) = &mut world.bounds[id] {
                bounds.x = indent * depth as f32;
                bounds.y = *y;
                *y += spacing;
            }
        }
        // Thu thập children trước để tránh borrow lỗi
        let children_ids = if let Some(children) = &world.childrens[id] {
            children.0.clone()
        } else {
            vec![]
        };
        for child_id in children_ids {
            Self::layout_entity(world, child_id, depth + 1, y, spacing, indent);
        }
    }
}

impl System for Layout {
    /// Cập nhật vị trí x/y cho tất cả entity theo cấu trúc phân cấp.
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        let spacing = 40.0;
        let indent = 32.0;
        let mut y = 0.0;
        // Duyệt tất cả entity không có Parent (gốc)
        for id in 0..world.entity_count {
            if world.parents[id].is_none() {
                Self::layout_entity(world, id, 0, &mut y, spacing, indent);
            }
        }
    }
}

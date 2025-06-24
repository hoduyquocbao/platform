use crate::World;
use crate::ui::components::Visible;
use crate::engine::System;
use crate::resources::Resources;

/// System chịu trách nhiệm lọc entity theo text, status, overdue và cập nhật trạng thái Visible.
pub struct FilterSystem;

impl System for FilterSystem {
    /// Lọc entity theo các tiêu chí trong resource Filter, cập nhật trạng thái Visible cho entity phù hợp.
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let filter = &resources.filter;
        let now = resources.time.now;
        // Xóa Visible khỏi tất cả entity
        for id in 0..world.entity_count {
            world.visibles[id] = None;
        }
        // Lọc entity
        for id in 0..world.entity_count {
            // Lọc theo text
            if let Some(ref text) = filter.text {
                let t = world.texts[id]
                    .as_ref()
                    .map(|t| t.value.as_str())
                    .unwrap_or("");
                if !t.contains(text) {
                    continue;
                }
            }
            // Lọc theo status
            if filter.status.is_some() && world.statuses[id].is_none() {
                continue;
            }
            // Lọc overdue
            if filter.overdue {
                if let Some(due) = world.dues[id].as_ref() {
                    if due.0 >= now || world.statuses[id].is_none() {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            // Lọc theo owner
            if let Some(owner_id) = filter.owner {
                if let Some(owner) = world.owners[id].as_ref() {
                    if owner.0 != owner_id {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            // Nếu qua hết các filter, cho Visible
            world.visibles[id] = Some(Visible);
        }
    }
}

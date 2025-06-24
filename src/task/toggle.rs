use crate::World;
use crate::components::{Status, Selected, Dirty};
use crate::engine::System;
use crate::resources::Resources;

/// System xử lý toggle trạng thái Status (TODO/DONE) dựa trên sự kiện Click.
pub struct ToggleSystem;

impl System for ToggleSystem {
    /// Thực thi toggle Status cho mỗi entity có cả Click và Status, sau đó đánh dấu Dirty.
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.clicks[id].is_some() && world.statuses[id].is_some() {
                // Toggle Status: nếu có Status thì xóa, nếu không có thì thêm
                if world.statuses[id].is_some() {
                    world.statuses[id] = None; // Chuyển từ DONE về TODO
                } else {
                    world.statuses[id] = Some(Status); // Chuyển từ TODO sang DONE
                }
                world.dirties[id] = Some(Dirty);
                world.clicks[id] = None; // Xóa sự kiện Click
            }
        }
    }
}

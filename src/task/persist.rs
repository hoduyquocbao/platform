use crate::World;
use crate::components::Dirty;
use crate::engine::System;
use crate::resources::Resources;

/// System xử lý lưu trữ dữ liệu cho các entity có component Dirty.
pub struct PersistSystem;

impl System for PersistSystem {
    /// Thực thi lưu trữ cho mỗi entity có component Dirty, sau đó xóa component Dirty.
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.dirties[id].is_some() {
                // TODO: Thực hiện logic lưu trữ thực tế (database, file, network, etc.)
                // Hiện tại chỉ đơn giản là xóa component Dirty
                world.dirties[id] = None;
            }
        }
    }
}

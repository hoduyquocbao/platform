use crate::World;
use crate::components::core::*;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

/// Hệ thống xử lý lệnh tạo task mới dựa trên component Create.
pub struct CreateSystem;

impl System for CreateSystem {
    /// Thực thi tạo task mới cho mỗi entity có component Create.
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.creates[id].is_some() {
                let e = world.spawn();
                world.texts[e] = Some(Text {
                    value: "New Task".to_string(),
                });
                world.statuses[e] = Some(Status);
                world.priorities[e] = Some(Priority);
                world.visibles[e] = Some(Visible);
                world.bounds[e] = Some(crate::components::ui::Bounds {
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 30.0,
                });
                world.styles[e] = Some(crate::components::ui::Style { color: "gray" });
                world.creates[id] = None;
            }
        }
    }
}

/// Hệ thống xử lý lệnh xóa task dựa trên component Delete.
pub struct DeleteSystem;

impl System for DeleteSystem {
    /// Thực thi xóa entity cho mỗi entity có component Delete.
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.deletes[id].is_some() {
                world.mark_for_delete(id);
                world.deletes[id] = None;
            }
        }
        world.sweep();
    }
}

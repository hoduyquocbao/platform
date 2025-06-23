use crate::World;
use crate::components::core::*;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;
use crate::components::core::Delete as CoreDelete;

/// Hệ thống xử lý lệnh tạo task mới dựa trên component Create.
pub struct Create;

impl System for Create {
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
                // Nếu có entity đang Selected, tạo quan hệ cha-con
                let mut parent_id = None;
                for sid in 0..world.entity_count {
                    if world.selecteds[sid].is_some() {
                        parent_id = Some(sid);
                        break;
                    }
                }
                if let Some(pid) = parent_id {
                    world.parents[e] = Some(Parent(pid));
                    if let Some(children) = &mut world.childrens[pid] {
                        children.0.push(e);
                    } else {
                        world.childrens[pid] = Some(Children(vec![e]));
                    }
                }
                world.creates[id] = None;
            }
        }
    }
}

/// Hệ thống xử lý lệnh xóa task dựa trên component Delete.
pub struct Delete;

impl System for Delete {
    /// Thực thi xóa entity cho mỗi entity có component Delete, bao gồm xóa theo tầng (cascading deletes).
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        // Đánh dấu Delete cho toàn bộ cây con trước khi xóa entity cha
        fn mark_cascade(world: &mut World, root_id: usize) {
            let mut stack = vec![root_id];
            while let Some(id) = stack.pop() {
                if let Some(children) = world.childrens[id].as_ref() {
                    for &child_id in &children.0 {
                        if world.deletes[child_id].is_none() {
                            world.deletes[child_id] = Some(CoreDelete);
                            stack.push(child_id);
                        }
                    }
                }
            }
        }
        for id in 0..world.entity_count {
            if world.deletes[id].is_some() {
                mark_cascade(world, id);
            }
        }
        for id in 0..world.entity_count {
            if world.deletes[id].is_some() {
                world.mark_for_delete(id);
                world.deletes[id] = None;
            }
        }
        world.sweep();
    }
}

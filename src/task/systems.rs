use crate::World;
use crate::task::components::{Delete as CoreDelete, Parent, Children, Dirty};
use crate::engine::System;
use crate::resources::Resources;
use crate::user::components::Owner;

/// System xử lý lệnh tạo task mới dựa trên component Create.
pub struct CreateSystem;

impl System for CreateSystem {
    /// Thực thi tạo task mới cho mỗi entity có component Create, tự động gán parent nếu có entity đang Selected.
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.creates[id].is_some() {
                let e = world.spawn();
                world.texts[e] = Some(crate::task::components::Text {
                    value: "New Task".to_string(),
                });
                world.statuses[e] = Some(crate::task::components::Status);
                world.priorities[e] = Some(crate::task::components::Priority);
                world.visibles[e] = Some(crate::ui::components::Visible);
                world.bounds[e] = Some(crate::ui::components::Bounds {
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 30.0,
                });
                world.styles[e] = Some(crate::ui::components::Style { color: "gray".to_string() });
                
                // Tự động gán Owner từ session hiện tại
                if let Some(session) = &resources.session {
                    world.owners[e] = Some(Owner(session.user));
                }
                
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

/// System xử lý lệnh xóa task dựa trên component Delete, hỗ trợ xóa theo tầng (cascading deletes).
pub struct DeleteSystem;

impl System for DeleteSystem {
    /// Thực thi xóa entity cho mỗi entity có component Delete, bao gồm xóa theo tầng (cascading deletes) bằng stack.
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
                    world.statuses[id] = Some(crate::task::components::Status); // Chuyển từ TODO sang DONE
                }
                world.dirties[id] = Some(Dirty);
                world.clicks[id] = None; // Xóa sự kiện Click
            }
        }
    }
}

/// System xử lý nhập liệu văn bản cho entity đang ở chế độ Editing.
pub struct TextSystem;

impl System for TextSystem {
    /// Thực thi cập nhật văn bản cho mỗi entity có component Editing và có ký tự mới từ keyboard.
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let chars = &resources.keyboard.chars;
        if chars.is_empty() {
            return;
        }
        for id in 0..world.entity_count {
            if world.editings[id].is_some() {
                if let Some(text) = &mut world.texts[id] {
                    text.value.push_str(chars);
                    world.dirties[id] = Some(Dirty);
                }
            }
        }
    }
}

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

use crate::World;
use crate::components::{Text, Editing, Dirty};
use crate::engine::System;
use crate::resources::Resources;

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

use crate::components::core::*;
use crate::World;

pub fn toggle(world: &mut World) {
    for id in 0..world.entity_count {
        if world.selecteds[id].is_some() && world.clicks[id].is_some() {
            // Đổi trạng thái Status (giả lập: nếu có thì xóa, nếu không thì thêm)
            if world.statuses[id].is_some() {
                world.statuses[id] = None;
                println!("[Toggle] Đã bỏ trạng thái Status cho entity {}", id);
            } else {
                world.statuses[id] = Some(Status);
                println!("[Toggle] Đã thêm trạng thái Status cho entity {}", id);
            }
            world.dirties[id] = Some(Dirty);
        }
    }
} 
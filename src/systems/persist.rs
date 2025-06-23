use crate::components::core::*;
use crate::World;
use crate::resources::input::*;

pub fn persist(world: &mut World, _mouse: &Mouse) {
    for id in 0..world.entity_count {
        if world.dirties[id].is_some() {
            println!("[Persist] Đã lưu các thay đổi cho Entity {}", id);
            world.dirties[id] = None;
        }
    }
} 
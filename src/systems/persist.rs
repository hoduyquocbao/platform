use crate::World;

pub fn persist(world: &mut World, _mouse: &crate::resources::input::Mouse) {
    for id in 0..world.entity_count {
        if world.dirties[id].is_some() {
            println!("[Persist] Đã lưu các thay đổi cho Entity {}", id);
            world.dirties[id] = None;
        }
    }
} 
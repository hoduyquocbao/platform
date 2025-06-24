use crate::World;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

pub struct Persist;

impl System for Persist {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.dirties[id].is_some() {
                println!("[Persist] Đã lưu các thay đổi cho Entity {}", id);
                world.dirties[id] = None;
            }
        }
    }
}

use crate::components::core::*;
use crate::resources::input::mod_rs::Resources;
use crate::engine::System;
use crate::World;

pub struct Toggle;

impl System for Toggle {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.selecteds[id].is_some() && world.clicks[id].is_some() {
                // Đảo trạng thái Status
                if world.statuses[id].is_some() {
                    world.statuses[id] = None;
                } else {
                    world.statuses[id] = Some(Status);
                }
                world.dirties[id] = Some(Dirty);
            }
        }
    }
} 
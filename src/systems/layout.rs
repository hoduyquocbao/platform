use crate::World;
use crate::resources::input::mod_rs::Resources;
use crate::engine::System;

pub struct Layout;

impl System for Layout {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        let mut y = 0.0;
        let spacing = 40.0;
        for id in 0..world.entity_count {
            if world.visibles[id].is_some() && world.bounds[id].is_some() {
                if let Some(bounds) = &mut world.bounds[id] {
                    bounds.y = y;
                    y += spacing;
                }
            }
        }
    }
} 
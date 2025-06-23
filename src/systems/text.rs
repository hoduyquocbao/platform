use crate::resources::input::mod_rs::Resources;
use crate::engine::System;
use crate::World;

pub struct TextSystem;

impl System for TextSystem {
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let keyboard = &resources.keyboard;
        for id in 0..world.entity_count {
            if world.editings[id].is_some() {
                if let Some(text) = &mut world.texts[id] {
                    if let Some(c) = keyboard.key {
                        text.value.push(c);
                    }
                    if keyboard.backspace {
                        text.value.pop();
                    }
                }
            }
        }
    }
} 
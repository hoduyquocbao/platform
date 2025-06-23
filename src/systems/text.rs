use crate::components::core::*;
use crate::World;
use crate::systems::edit::{Keyboard, Key};

pub fn text(world: &mut World, keyboard: &Keyboard) {
    for id in 0..world.entity_count {
        if world.editings[id].is_some() {
            if let Some(Key::Char(c)) = keyboard.key {
                if let Some(text) = &mut world.texts[id] {
                    text.value.push(c);
                }
            }
        }
    }
} 
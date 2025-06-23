use crate::components::core::*;
use crate::resources::input::*;
use crate::World;

pub fn text(world: &mut World, keyboard: &Keyboard) {
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
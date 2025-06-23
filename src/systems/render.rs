use crate::World;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

pub struct Render;

impl System for Render {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        println!("--- FRAME START ---");
        for id in 0..world.entity_count {
            if world.visibles[id].is_some()
                && world.texts[id].is_some()
                && world.bounds[id].is_some()
            {
                let text = world.texts[id]
                    .as_ref()
                    .map(|t| t.value.as_str())
                    .unwrap_or("");
                let bounds = world.bounds[id].as_ref().unwrap();
                let status = if world.statuses[id].is_some() {
                    "[TODO]"
                } else {
                    "[DONE]"
                };
                let style = world.styles[id].as_ref().map(|s| s.color).unwrap_or("");
                let mut prefix = " ";
                if world.editings[id].is_some() {
                    prefix = "[EDITING]";
                } else if world.selecteds[id].is_some() {
                    prefix = "*";
                } else if world.hovers[id].is_some() {
                    prefix = ">";
                }
                let mut display_text = text.to_string();
                if world.editings[id].is_some() {
                    display_text.push('|');
                }
                println!(
                    "{} {} ({}, {}) {} {} {}",
                    prefix, status, bounds.x, bounds.y, display_text, style, id
                );
            }
        }
    }
}

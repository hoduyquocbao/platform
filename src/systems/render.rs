use crate::World;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;
use crate::components::traits::Renderable;

pub struct Render;

impl System for Render {
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        println!("--- FRAME START ---");
        for id in 0..world.entity_count {
            if world.visibles[id].is_some()
                && world.texts[id].is_some()
                && world.bounds[id].is_some()
            {
                let text = world.texts[id].as_ref().map(|t| t.object().value.as_str()).unwrap_or("");
                let bounds = world.bounds[id].as_ref().unwrap();
                let status = if let Some(s) = world.statuses[id].as_ref() {
                    s.object();
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
                let mut icon = "";
                if world.childrens[id].is_some() {
                    if world.collapseds[id].is_some() {
                        icon = "[+]";
                    } else {
                        icon = "[-]";
                    }
                }
                let mut display_text = text.to_string();
                if world.editings[id].is_some() {
                    display_text.push('|');
                }
                let indent = (bounds.x / 8.0) as usize;
                let indent_str = "  ".repeat(indent);
                println!(
                    "{}{}{} {} ({}, {}) {} {} {}",
                    indent_str, icon, prefix, status, bounds.x, bounds.y, display_text, style, id
                );
            }
        }
    }
}

use crate::World;
use crate::components::traits::Renderable;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

/// System chịu trách nhiệm render (hiển thị) toàn bộ entity ra UI, bao gồm trạng thái, style, due date, filter.
pub struct Render;

impl System for Render {
    /// Render toàn bộ entity Visible ra UI, hiển thị trạng thái, style, due date, filter.
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let now = resources.time.now;
        println!("--- FRAME START ---");
        for id in 0..world.entity_count {
            if world.visibles[id].is_some()
                && world.texts[id].is_some()
                && world.bounds[id].is_some()
            {
                let text = world.texts[id]
                    .as_ref()
                    .map(|t| t.object().value.as_str())
                    .unwrap_or("");
                let bounds = world.bounds[id].as_ref().unwrap();
                let status = if let Some(s) = world.statuses[id].as_ref() {
                    s.object();
                    "[TODO]"
                } else {
                    "[DONE]"
                };
                let mut style = world.styles[id].as_ref().map(|s| s.color).unwrap_or("");
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
                let mut due_str = String::new();
                if let Some(due) = world.dues[id].as_ref() {
                    due_str = format!(" [Due: {}]", due.0);
                    if due.0 < now && world.statuses[id].is_some() {
                        style = "red";
                    }
                }
                let indent = (bounds.x / 8.0) as usize;
                let indent_str = "  ".repeat(indent);
                println!(
                    "{}{}{} {} ({}, {}) {}{} {} {}",
                    indent_str,
                    icon,
                    prefix,
                    status,
                    bounds.x,
                    bounds.y,
                    display_text,
                    due_str,
                    style,
                    id
                );
            }
        }
        let filter = &resources.filter;
        let mut filter_str = String::from("FILTERING:");
        if let Some(ref t) = filter.text {
            filter_str.push_str(&format!(" text=\"{}\"", t));
        }
        if filter.status.is_some() {
            filter_str.push_str(" status=TODO");
        }
        if filter.overdue {
            filter_str.push_str(" overdue=true");
        }
        if filter_str == "FILTERING:" {
            filter_str.push_str(" (none)");
        }
        println!("{}", filter_str);
    }
}

use crate::components::core::*;
use crate::components::ui::*;
use crate::World;
use crate::resources::input::*;

pub fn render(world: &mut World, _mouse: &Mouse) {
    println!("--- FRAME START ---");
    for id in 0..world.entity_count {
        if world.visibles[id].is_some() && world.texts[id].is_some() && world.statuses[id].is_some() && world.styles[id].is_some() {
            let text = world.texts[id].as_ref().map(|t| t.value.as_str()).unwrap_or("");
            let status = if world.statuses[id].is_some() { "[TODO]" } else { "[DONE]" };
            let style = world.styles[id].as_ref().map(|s| s.color).unwrap_or("");
            let mut prefix = " ";
            if world.selecteds[id].is_some() {
                prefix = "*";
            } else if world.hovers[id].is_some() {
                prefix = ">";
            }
            println!("{} {} {} {}", prefix, status, text, style);
        }
    }
} 
use crate::components::core::*;
use crate::World;

pub fn render(world: &mut World) {
    println!("--- FRAME START ---");
    for id in 0..world.entity_count {
        if world.visibles[id].is_some() {
            let text = world.texts[id].as_ref().map(|t| t.value.as_str()).unwrap_or("");
            let status = if world.statuses[id].is_some() { "[TODO]" } else { "[DONE]" };
            println!("{} {}", status, text);
        }
    }
} 
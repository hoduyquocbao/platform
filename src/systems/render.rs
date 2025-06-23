use crate::World;

pub fn render(world: &mut World, _mouse: &crate::resources::input::Mouse) {
    println!("--- FRAME START ---");
    for id in 0..world.entity_count {
        if world.visibles[id].is_some() && world.texts[id].is_some() && world.statuses[id].is_some() && world.styles[id].is_some() && world.bounds[id].is_some() {
            let text = world.texts[id].as_ref().map(|t| t.value.as_str()).unwrap_or("");
            let status = if world.statuses[id].is_some() { "[TODO]" } else { "[DONE]" };
            let style = world.styles[id].as_ref().map(|s| s.color).unwrap_or("");
            let bounds = world.bounds[id].as_ref().unwrap();
            let mut prefix = " ";
            if world.selecteds[id].is_some() {
                prefix = "*";
            } else if world.hovers[id].is_some() {
                prefix = ">";
            }
            if world.editings[id].is_some() {
                println!("{:>4.0},{:>4.0} {} [EDITING] |{}| {}", bounds.x, bounds.y, prefix, text, style);
            } else {
                println!("{:>4.0},{:>4.0} {} {} {} {}", bounds.x, bounds.y, prefix, status, text, style);
            }
        }
    }
} 
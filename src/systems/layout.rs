use crate::World;

pub fn layout(world: &mut World, _mouse: &crate::resources::input::Mouse) {
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
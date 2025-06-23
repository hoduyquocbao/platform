use crate::components::core::*;
use crate::World;

pub fn input(world: &mut World) {
    if world.entity_count > 0 {
        let id = 0;
        world.selecteds[id] = Some(Selected);
        world.clicks[id] = Some(Click);
        println!("[Input] Đã chọn và click entity 0");
    }
} 
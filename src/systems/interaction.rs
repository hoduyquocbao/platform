use crate::components::core::*;

use crate::resources::input::*;
use crate::World;

static mut LAST_PRESSED: bool = false;

pub fn interact(world: &mut World, mouse: &Mouse) {
    // Xóa Click ở đầu frame
    for id in 0..world.entity_count {
        world.clicks[id] = None;
        world.hovers[id] = None;
    }
    // Hit detection
    for id in 0..world.entity_count {
        if let Some(bounds) = &world.bounds[id] {
            let (mx, my) = mouse.position;
            if mx >= bounds.x && mx <= bounds.x + bounds.width && my >= bounds.y && my <= bounds.y + bounds.height {
                world.hovers[id] = Some(Hover);
                if mouse.pressed {
                    // Chỉ cho phép một entity được chọn
                    for i in 0..world.entity_count {
                        world.selecteds[i] = None;
                    }
                    world.selecteds[id] = Some(Selected);
                }
                // Phát hiện click (pressed chuyển từ true sang false)
                unsafe {
                    if LAST_PRESSED && !mouse.pressed {
                        world.clicks[id] = Some(Click);
                    }
                }
            }
        }
    }
    // Lưu trạng thái mouse.pressed cho frame sau
    unsafe {
        LAST_PRESSED = mouse.pressed;
    }
} 
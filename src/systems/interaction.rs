use crate::components::core::*;
use crate::resources::input::mod_rs::Resources;
use crate::engine::System;
use crate::World;

pub struct Interact;

static mut LAST_PRESSED: bool = false;

impl System for Interact {
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let mouse = &resources.mouse;
        let keyboard = &resources.keyboard;
        // Reset Click và Hover ở đầu frame
        for id in 0..world.entity_count {
            world.clicks[id] = None;
            world.hovers[id] = None;
        }
        // Chế độ vào/ra Editing
        for id in 0..world.entity_count {
            // Vào chế độ Editing
            if world.selecteds[id].is_some() && keyboard.e && world.editings[id].is_none() {
                world.editings[id] = Some(Editing);
            }
            // Thoát Editing
            if world.editings[id].is_some() && (keyboard.enter || keyboard.escape) {
                world.editings[id] = None;
                if keyboard.enter {
                    world.dirties[id] = Some(Dirty);
                }
            }
        }
        // Tạo task mới khi nhấn 'n'
        if keyboard.key == Some('n') {
            let e = world.spawn();
            world.creates[e] = Some(Create);
        }
        // Xóa task đang chọn khi nhấn 'd'
        if keyboard.key == Some('d') {
            for id in 0..world.entity_count {
                if world.selecteds[id].is_some() {
                    world.deletes[id] = Some(Delete);
                }
            }
        }
        // Hit detection và phát hiện click (chỉ khi không Editing)
        for id in 0..world.entity_count {
            if world.editings[id].is_some() {
                continue;
            }
            if let Some(bounds) = &world.bounds[id] {
                let (mx, my) = mouse.position;
                if mx >= bounds.x && mx <= bounds.x + bounds.width && my >= bounds.y && my <= bounds.y + bounds.height {
                    world.hovers[id] = Some(Hover);
                    if mouse.pressed {
                        for i in 0..world.entity_count {
                            world.selecteds[i] = None;
                        }
                        world.selecteds[id] = Some(Selected);
                    }
                    // Phát hiện click (pressed true->false)
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
} 
use crate::World;
use crate::components::core::*;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;
use crate::components::traits::Interactable;

/// Hệ thống xử lý tương tác chuột và bàn phím, bao gồm chọn, chỉnh sửa, tạo, xóa task.
pub struct Interact;

static mut LAST_PRESSED: bool = false;

impl Interact {
    /// Đặt lại trạng thái Click và Hover cho tất cả entity.
    fn reset_hover_click(world: &mut World) {
        for id in 0..world.entity_count {
            world.clicks[id] = None;
            world.hovers[id] = None;
        }
    }
    /// Xử lý vào/ra chế độ Editing dựa trên bàn phím.
    fn handle_editing(world: &mut World, keyboard: &crate::resources::input::Keyboard) {
        for id in 0..world.entity_count {
            if world.selecteds[id].as_ref().map(|s| s.target()).is_some() && keyboard.e && world.editings[id].is_none() {
                world.editings[id] = Some(Editing);
            }
            if world.editings[id].as_ref().map(|e| e.target()).is_some() && (keyboard.enter || keyboard.escape) {
                world.editings[id] = None;
                if keyboard.enter {
                    world.dirties[id] = Some(Dirty);
                }
            }
        }
    }
    /// Xử lý tạo task mới khi nhấn 'n'.
    fn handle_create(world: &mut World, keyboard: &crate::resources::input::Keyboard) {
        if keyboard.key == Some('n') {
            let e = world.spawn();
            world.creates[e] = Some(Create);
        }
    }
    /// Xử lý xóa task đang chọn khi nhấn 'd'.
    fn handle_delete(world: &mut World, keyboard: &crate::resources::input::Keyboard) {
        if keyboard.key == Some('d') {
            for id in 0..world.entity_count {
                if world.selecteds[id].as_ref().map(|s| s.target()).is_some() {
                    world.deletes[id] = Some(Delete);
                }
            }
        }
    }
    /// Xử lý phát hiện hover, click, chọn entity bằng chuột.
    fn handle_mouse_interaction(world: &mut World, mouse: &crate::resources::input::Mouse) {
        for id in 0..world.entity_count {
            if world.editings[id].as_ref().map(|e| e.target()).is_some() {
                continue;
            }
            if let Some(bounds) = &world.bounds[id] {
                let (mx, my) = mouse.position;
                let icon_width = 24.0; // vùng biểu tượng mở/đóng bên trái
                let in_icon = mx >= bounds.x && mx <= bounds.x + icon_width && my >= bounds.y && my <= bounds.y + bounds.height;
                let in_main = mx > bounds.x + icon_width && mx <= bounds.x + bounds.width && my >= bounds.y && my <= bounds.y + bounds.height;
                if in_icon && world.childrens[id].is_some() {
                    world.hovers[id] = Some(Hover);
                    if mouse.pressed {
                        // Không select, chỉ toggle collapsed
                        // Toggle Collapsed
                        if world.collapseds[id].is_some() {
                            world.collapseds[id] = None;
                        } else {
                            world.collapseds[id] = Some(Collapsed);
                        }
                        world.dirties[id] = Some(Dirty);
                    }
                    unsafe {
                        if LAST_PRESSED && !mouse.pressed {
                            // Không cần click event riêng cho icon
                        }
                    }
                } else if in_main {
                    world.hovers[id] = Some(Hover);
                    if mouse.pressed {
                        for i in 0..world.entity_count {
                            world.selecteds[i] = None;
                        }
                        world.selecteds[id] = Some(Selected);
                    }
                    unsafe {
                        if LAST_PRESSED && !mouse.pressed {
                            world.clicks[id] = Some(Click);
                        }
                    }
                }
            }
        }
    }
    /// Lưu trạng thái mouse.pressed cho frame sau.
    fn update_last_pressed(mouse: &crate::resources::input::Mouse) {
        unsafe {
            LAST_PRESSED = mouse.pressed;
        }
    }
}

impl System for Interact {
    /// Hàm chính thực thi hệ thống tương tác mỗi frame.
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let mouse = &resources.mouse;
        let keyboard = &resources.keyboard;
        Self::reset_hover_click(world);
        Self::handle_editing(world, keyboard);
        Self::handle_create(world, keyboard);
        Self::handle_delete(world, keyboard);
        Self::handle_mouse_interaction(world, mouse);
        Self::update_last_pressed(mouse);
    }
}

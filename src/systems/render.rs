use crate::World;
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

/// System chịu trách nhiệm render (hiển thị) toàn bộ entity ra UI, bao gồm trạng thái, style, due date, filter.
pub struct Render;

fn color_to_u32(color: &str) -> u32 {
    match color {
        "blue" => 0xFF2196F3,
        "green" => 0xFF4CAF50,
        "red" => 0xFFF44336,
        "gray" => 0xFFBDBDBD,
        "yellow" => 0xFFFFEB3B,
        "orange" => 0xFFFF9800,
        "purple" => 0xFF9C27B0,
        "black" => 0xFF000000,
        "white" => 0xFFFFFFFF,
        _ => 0xFF757575,
    }
}

impl System for Render {
    /// Render toàn bộ entity Visible ra UI, vẽ hình chữ nhật màu sắc lên framebuffer.
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let (framebuffer, width, height) = match resources.framebuffer {
            Some((ptr, w, h)) => unsafe { (std::slice::from_raw_parts_mut(ptr, w * h), w, h) },
            None => return,
        };
        // Clear background
        for px in framebuffer.iter_mut() {
            *px = 0xFF222222;
        }
        for id in 0..world.entity_count {
            if world.visibles[id].is_some()
                && world.bounds[id].is_some()
                && world.styles[id].is_some()
            {
                let bounds = world.bounds[id].as_ref().unwrap();
                let style = world.styles[id].as_ref().unwrap();
                let mut color = color_to_u32(style.color);
                // Highlight nếu selected/hover
                if world.selecteds[id].is_some() {
                    color = 0xFF1976D2; // darker blue
                } else if world.hovers[id].is_some() {
                    color = 0xFF90CAF9; // light blue
                }
                let x0 = bounds.x as usize;
                let y0 = bounds.y as usize;
                let w = bounds.width as usize;
                let h = bounds.height as usize;
                for y in y0..(y0 + h).min(height) {
                    for x in x0..(x0 + w).min(width) {
                        framebuffer[y * width + x] = color;
                    }
                }
            }
        }
    }
}

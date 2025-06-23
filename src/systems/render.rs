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

#[allow(clippy::too_many_arguments)]
fn blit_glyph(
    framebuffer: &mut [u32],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    bitmap: &[u8],
    glyph_w: usize,
    glyph_h: usize,
    color: u32,
) {
    for row in 0..glyph_h {
        for col in 0..glyph_w {
            let alpha = bitmap[row * glyph_w + col] as f32 / 255.0;
            let px = x + col;
            let py = y + row;
            if px < width && py < height {
                let idx = py * width + px;
                let bg = framebuffer[idx];
                // Simple alpha blend
                let r = ((color >> 16) & 0xFF) as f32;
                let g = ((color >> 8) & 0xFF) as f32;
                let b = (color & 0xFF) as f32;
                let br = ((bg >> 16) & 0xFF) as f32;
                let bg_ = ((bg >> 8) & 0xFF) as f32;
                let bb = (bg & 0xFF) as f32;
                let nr = (r * alpha + br * (1.0 - alpha)) as u32;
                let ng = (g * alpha + bg_ * (1.0 - alpha)) as u32;
                let nb = (b * alpha + bb * (1.0 - alpha)) as u32;
                framebuffer[idx] = (0xFF << 24) | (nr << 16) | (ng << 8) | nb;
            }
        }
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
        let font = match &resources.font {
            Some(f) => &f.0,
            None => return,
        };
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
                // Vẽ text nếu có
                if let Some(text) = world.texts[id].as_ref() {
                    let text_color = 0xFFFFFFFF;
                    let mut cursor_x = x0 + 8;
                    let baseline = y0 + h / 2;
                    let font_size = (h as f32 * 0.5).max(12.0);
                    for ch in text.value.chars() {
                        let (metrics, bitmap) = font.rasterize(ch, font_size);
                        let glyph_y = baseline.saturating_sub(metrics.height / 2);
                        blit_glyph(
                            framebuffer,
                            width,
                            height,
                            cursor_x,
                            glyph_y,
                            &bitmap,
                            metrics.width,
                            metrics.height,
                            text_color,
                        );
                        cursor_x += metrics.advance_width as usize;
                    }
                }
            }
        }
    }
}

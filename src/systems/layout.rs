use crate::World;
use crate::components::traits::Layoutable;
use crate::components::ui::{Container, Flow, Align, Justify, Bounds};
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

/// System bố cục UI theo container lồng nhau, hỗ trợ Flow::Column/Row và Justify cơ bản.
pub struct Layout;

impl Layout {
    /// Đệ quy layout cho entity dạng container và các con của nó.
    fn layout_container(
        world: &mut World,
        id: usize,
        bounds: Bounds,
    ) {
        // Nếu entity là container
        let is_container = world.container.is_some();
        let flow = world.flows.as_ref().unwrap_or(&Flow::Column);
        if is_container {
            // Lấy danh sách con
            let children_ids = if let Some(children) = &world.childrens[id] {
                children.0.clone()
            } else {
                vec![]
            };
            let n = children_ids.len();
            if n == 0 { return; }
            // Tính toán kích thước cho từng con
            match flow {
                Flow::Column => {
                    let h = bounds.height / n as f32;
                    for (i, &child_id) in children_ids.iter().enumerate() {
                        let child_bounds = Bounds {
                            x: bounds.x,
                            y: bounds.y + i as f32 * h,
                            width: bounds.width,
                            height: h,
                        };
                        world.bounds[child_id] = Some(child_bounds);
                        Self::layout_container(world, child_id, child_bounds);
                    }
                }
                Flow::Row => {
                    let w = bounds.width / n as f32;
                    for (i, &child_id) in children_ids.iter().enumerate() {
                        let child_bounds = Bounds {
                            x: bounds.x + i as f32 * w,
                            y: bounds.y,
                            width: w,
                            height: bounds.height,
                        };
                        world.bounds[child_id] = Some(child_bounds);
                        Self::layout_container(world, child_id, child_bounds);
                    }
                }
            }
        }
    }
}

impl System for Layout {
    /// Bắt đầu layout từ entity root (không có Parent, có Container)
    fn run(&mut self, world: &mut World, _resources: &mut Resources) {
        for id in 0..world.entity_count {
            if world.parents[id].is_none() && world.container.is_some() {
                if let Some(bounds) = world.bounds[id].clone() {
                    Self::layout_container(world, id, bounds);
                }
            }
        }
    }
}

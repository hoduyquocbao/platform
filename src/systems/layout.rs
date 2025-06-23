use crate::World;
use crate::components::traits::Layoutable;
use crate::components::ui::{Container, Flow, Align, Justify, Bounds};
use crate::engine::System;
use crate::resources::input::mod_rs::Resources;

/// System bố cục UI theo container lồng nhau, hỗ trợ Flow, Align, Justify.
pub struct Layout;

impl Layout {
    /// Đệ quy layout cho entity dạng container và các con của nó.
    fn layout_container(
        world: &mut World,
        id: usize,
        bounds: Bounds,
    ) {
        // Nếu entity là container
        let is_container = world.container[id].is_some();
        let flow = world.flows.get(id).and_then(|f| f.as_ref()).cloned().unwrap_or(Flow::Column);
        let align = world.aligns.get(id).and_then(|a| a.as_ref()).cloned().unwrap_or(Align::Start);
        let justify = world.justifys.get(id).and_then(|j| j.as_ref()).cloned().unwrap_or(Justify::Start);
        if is_container {
            // Lấy danh sách con
            let children_ids = if let Some(children) = &world.childrens[id] {
                children.0.clone()
            } else {
                vec![]
            };
            let n = children_ids.len();
            if n == 0 { return; }
            // clone children_ids để không borrow world khi vào vòng lặp
            let children_ids = children_ids.clone();
            // Tính toán kích thước cho từng con
            match flow {
                Flow::Column => {
                    let total_height: f32 = bounds.height;
                    let child_height = total_height / n as f32;
                    let mut y = bounds.y;
                    let extra_space = total_height - child_height * n as f32;
                    let (start_offset, space_between) = match justify {
                        Justify::Start => (0.0, 0.0),
                        Justify::Center => (extra_space / 2.0, 0.0),
                        Justify::End => (extra_space, 0.0),
                        Justify::Between => {
                            let gaps = if n > 1 { n - 1 } else { 1 } as f32;
                            (0.0, if n > 1 { extra_space / gaps } else { 0.0 })
                        }
                    };
                    y += start_offset;
                    for (i, &child_id) in children_ids.iter().enumerate() {
                        let mut child_bounds = Bounds {
                            x: bounds.x,
                            y,
                            width: bounds.width,
                            height: child_height,
                        };
                        // Align trên trục phụ (x)
                        match align {
                            Align::Start => {},
                            Align::Center => {
                                child_bounds.x += (bounds.width - child_bounds.width) / 2.0;
                            },
                            Align::End => {
                                child_bounds.x += bounds.width - child_bounds.width;
                            },
                        }
                        world.bounds[child_id] = Some(child_bounds);
                        Self::layout_container(world, child_id, child_bounds);
                        y += child_height + space_between;
                    }
                }
                Flow::Row => {
                    let total_width: f32 = bounds.width;
                    let child_width = total_width / n as f32;
                    let mut x = bounds.x;
                    let extra_space = total_width - child_width * n as f32;
                    let (start_offset, space_between) = match justify {
                        Justify::Start => (0.0, 0.0),
                        Justify::Center => (extra_space / 2.0, 0.0),
                        Justify::End => (extra_space, 0.0),
                        Justify::Between => {
                            let gaps = if n > 1 { n - 1 } else { 1 } as f32;
                            (0.0, if n > 1 { extra_space / gaps } else { 0.0 })
                        }
                    };
                    x += start_offset;
                    for (i, &child_id) in children_ids.iter().enumerate() {
                        let mut child_bounds = Bounds {
                            x,
                            y: bounds.y,
                            width: child_width,
                            height: bounds.height,
                        };
                        // Align trên trục phụ (y)
                        match align {
                            Align::Start => {},
                            Align::Center => {
                                child_bounds.y += (bounds.height - child_bounds.height) / 2.0;
                            },
                            Align::End => {
                                child_bounds.y += bounds.height - child_bounds.height;
                            },
                        }
                        world.bounds[child_id] = Some(child_bounds);
                        Self::layout_container(world, child_id, child_bounds);
                        x += child_width + space_between;
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
            if world.parents[id].is_none() && world.container[id].is_some() {
                if let Some(bounds) = world.bounds[id].clone() {
                    Self::layout_container(world, id, bounds);
                }
            }
        }
    }
}

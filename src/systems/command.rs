use crate::components::core::*;
use crate::components::ui::*;
use crate::World;
use crate::resources::input::Mouse;

pub fn process(world: &mut World, _mouse: &Mouse) {
    // Xử lý Create
    let mut create_ids = vec![];
    for id in 0..world.entity_count {
        if world.creates.get(id).is_some_and(|c| c.is_some()) {
            create_ids.push(id);
        }
    }
    for _ in &create_ids {
        // Tạo task mới
        let e = world.spawn();
        world.texts[e] = Some(Text { value: "New Task".to_string() });
        world.statuses[e] = Some(Status);
        world.visibles[e] = Some(Visible);
        world.bounds[e] = Some(Bounds { x: 0.0, y: 0.0, width: 100.0, height: 30.0 });
        world.styles[e] = Some(Style { color: "gray" });
    }
    // Xóa entity lệnh Create
    for id in create_ids {
        world.mark_for_delete(id);
    }
    // Xử lý Delete
    let mut delete_ids = vec![];
    for id in 0..world.entity_count {
        if world.deletes.get(id).is_some_and(|d| d.is_some()) {
            delete_ids.push(id);
        }
    }
    for id in delete_ids {
        world.mark_for_delete(id);
    }
    // Thực hiện xóa entity đã đánh dấu
    world.sweep();
} 
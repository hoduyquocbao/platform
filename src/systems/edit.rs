use crate::components::core::*;
use crate::World;

// Giả lập phím nhấn: e để vào Editing, Enter để lưu, Escape để hủy
pub enum Key {
    E,
    Enter,
    Escape,
    Char(char),
}

pub struct Keyboard {
    pub key: Option<Key>,
}

pub fn edit(world: &mut World, keyboard: &Keyboard) {
    for id in 0..world.entity_count {
        // Vào chế độ Editing
        if world.selecteds[id].is_some() && keyboard.key == Some(Key::E) {
            world.editings[id] = Some(Editing);
        }
        // Thoát chế độ Editing
        if world.editings[id].is_some() {
            if keyboard.key == Some(Key::Enter) || keyboard.key == Some(Key::Escape) {
                world.editings[id] = None;
                if keyboard.key == Some(Key::Enter) {
                    world.dirties[id] = Some(Dirty);
                }
            }
        }
    }
} 
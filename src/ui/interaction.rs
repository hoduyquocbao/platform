use crate::World;
use crate::ui::components::{Selected, Editing, Hover, Click, Interactable};
use crate::task::components::{Collapsed, Due, Scheduling, Create};
use crate::engine::System;
use crate::resources::Resources;

/// System xử lý toàn bộ logic tương tác chuột và bàn phím: chọn, chỉnh sửa, tạo, xóa, đặt ngày hết hạn, lọc, v.v.
pub struct InteractSystem;

static mut LAST_PRESSED: bool = false;

impl InteractSystem {
    /// Đặt lại trạng thái Click và Hover cho tất cả entity (reset mỗi frame).
    fn reset_hover_click(world: &mut World) {
        for id in 0..world.entity_count {
            world.clicks[id] = None;
            world.hovers[id] = None;
        }
    }
    /// Xử lý vào/ra chế độ Editing dựa trên bàn phím (E, Enter, Escape).
    fn handle_editing(world: &mut World, keyboard: &crate::resources::Keyboard) {
        for id in 0..world.entity_count {
            if world.selecteds[id].as_ref().map(|s| s.target()).is_some()
                && keyboard.e
                && world.editings[id].is_none()
            {
                world.editings[id] = Some(Editing);
            }
            if world.editings[id].as_ref().map(|e| e.target()).is_some()
                && (keyboard.enter || keyboard.escape)
            {
                world.editings[id] = None;
                if keyboard.enter {
                    world.dirties[id] = Some(crate::task::components::Dirty);
                }
            }
        }
    }
    /// Xử lý tạo task mới khi nhấn 'n'.
    fn handle_create(world: &mut World, keyboard: &crate::resources::Keyboard) {
        if keyboard.key == Some('n') {
            let e = world.spawn();
            world.creates[e] = Some(Create);
        }
    }
    /// Xử lý xóa task đang chọn khi nhấn 'd'.
    fn handle_delete(world: &mut World, keyboard: &crate::resources::Keyboard) {
        if keyboard.key == Some('d') {
            for id in 0..world.entity_count {
                if world.selecteds[id].as_ref().map(|s| s.target()).is_some() {
                    world.deletes[id] = Some(crate::task::components::Delete);
                }
            }
        }
    }
    /// Xử lý đặt/sửa ngày hết hạn (Due date) cho entity Selected (t, nhập số, Enter, Escape).
    fn handle_due(world: &mut World, keyboard: &crate::resources::Keyboard) {
        // Bước 1: Nếu có entity Selected và nhấn 't', thêm Scheduling
        for id in 0..world.entity_count {
            if world.selecteds[id].is_some()
                && keyboard.key == Some('t')
                && world.schedulings[id].is_none()
            {
                world.schedulings[id] = Some(Scheduling);
            }
        }
        // Bước 2: Nếu entity đang Scheduling, nhập số để build timestamp, Enter để lưu
        for id in 0..world.entity_count {
            if world.schedulings[id].is_some() {
                // Sử dụng tạm trường editings để lưu chuỗi số nhập vào (hoặc có thể mở rộng thêm trường riêng)
                // Đơn giản: mỗi lần nhập số, append vào Due tạm (dùng Option<u64> hoặc String)
                // Ở đây ta dùng một mảng tạm static để lưu input cho demo
                use std::cell::RefCell;
                thread_local! {
                    static DUE_INPUT: RefCell<[Option<String>; 128]> = RefCell::new([(); 128].map(|_| None));
                }
                // Nhận số
                if let Some(c) = keyboard.key {
                    if c.is_ascii_digit() {
                        DUE_INPUT.with(|arr| {
                            let mut arr = arr.borrow_mut();
                            let s = arr[id].get_or_insert(String::new());
                            s.push(c);
                        });
                    }
                }
                // Backspace
                if keyboard.backspace {
                    DUE_INPUT.with(|arr| {
                        let mut arr = arr.borrow_mut();
                        if let Some(s) = &mut arr[id] {
                            s.pop();
                        }
                    });
                }
                // Enter: lưu Due
                if keyboard.enter {
                    DUE_INPUT.with(|arr| {
                        let mut arr = arr.borrow_mut();
                        if let Some(s) = arr[id].take() {
                            if let Ok(ts) = s.parse::<u64>() {
                                world.dues[id] = Some(Due(ts));
                                world.dirties[id] = Some(crate::task::components::Dirty);
                            }
                        }
                    });
                    world.schedulings[id] = None;
                }
                // Escape: hủy
                if keyboard.escape {
                    DUE_INPUT.with(|arr| arr.borrow_mut()[id] = None);
                    world.schedulings[id] = None;
                }
            }
        }
    }
    /// Xử lý tìm kiếm/lọc: nhấn '/' vào chế độ search, nhập ký tự cập nhật Filter.text, 's' lọc Status, 'o' lọc overdue.
    fn handle_filter(
        filter: &mut crate::resources::Filter,
        keyboard: &crate::resources::Keyboard,
    ) {
        use std::cell::RefCell;
        thread_local! {
            static SEARCH_MODE: RefCell<bool> = const { RefCell::new(false) };
        }
        // Bật/tắt search mode
        if keyboard.key == Some('/') {
            SEARCH_MODE.with(|m| *m.borrow_mut() = true);
            filter.text = Some(String::new());
        }
        // Nếu đang search mode, nhập ký tự vào filter.text
        let mut in_search = false;
        SEARCH_MODE.with(|m| in_search = *m.borrow());
        if in_search {
            if let Some(c) = keyboard.key {
                if c.is_ascii_alphanumeric() || c == ' ' {
                    if let Some(s) = &mut filter.text {
                        s.push(c);
                    }
                }
            }
            if keyboard.backspace {
                if let Some(s) = &mut filter.text {
                    s.pop();
                }
            }
            if keyboard.enter || keyboard.escape {
                SEARCH_MODE.with(|m| *m.borrow_mut() = false);
                if let Some(s) = &filter.text {
                    if s.is_empty() {
                        filter.text = None;
                    }
                }
            }
        }
        // Phím nóng: 's' lọc Status, 'o' lọc overdue
        if keyboard.key == Some('s') {
            if filter.status.is_some() {
                filter.status = None;
            } else {
                filter.status = Some(crate::task::components::Status);
            }
        }
        if keyboard.key == Some('o') {
            filter.overdue = !filter.overdue;
        }
        // Phím nóng: 'u' lọc theo owner (chỉ hiển thị task của người dùng hiện tại)
        if keyboard.key == Some('u') {
            if filter.owner.is_some() {
                filter.owner = None;
            } else {
                // Sẽ được set trong run() method khi có session
            }
        }
    }
    /// Xử lý phát hiện hover, click, chọn entity bằng chuột (bao gồm toggle collapsed).
    fn handle_mouse_interaction(world: &mut World, mouse: &crate::resources::Mouse) {
        for id in 0..world.entity_count {
            if world.editings[id].as_ref().map(|e| e.target()).is_some() {
                continue;
            }
            if let Some(bounds) = &world.bounds[id] {
                let (mx, my) = mouse.position;
                let icon_width = 24.0; // vùng biểu tượng mở/đóng bên trái
                let in_icon = mx >= bounds.x
                    && mx <= bounds.x + icon_width
                    && my >= bounds.y
                    && my <= bounds.y + bounds.height;
                let in_main = mx > bounds.x + icon_width
                    && mx <= bounds.x + bounds.width
                    && my >= bounds.y
                    && my <= bounds.y + bounds.height;
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
                        world.dirties[id] = Some(crate::task::components::Dirty);
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
    /// Lưu trạng thái mouse.pressed cho frame sau (phục vụ phát hiện click).
    fn update_last_pressed(mouse: &crate::resources::Mouse) {
        unsafe {
            LAST_PRESSED = mouse.pressed;
        }
    }
    /// Xử lý click trên Button: nếu entity có Click và Button, tạo entity lệnh Create
    fn handle_button_click(world: &mut World) {
        for id in 0..world.entity_count {
            if world.clicks[id].is_some() && world.buttons[id].is_some() {
                let e = world.spawn();
                world.creates[e] = Some(Create);
            }
        }
    }
}

impl System for InteractSystem {
    /// Hàm chính thực thi hệ thống tương tác mỗi frame (gọi lần lượt các handler phụ trợ).
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let keyboard = &resources.keyboard;
        Self::handle_filter(&mut resources.filter, keyboard);
        
        // Xử lý phím 'u' để set owner filter từ session
        if keyboard.key == Some('u') && resources.filter.owner.is_none() {
            if let Some(session) = &resources.session {
                resources.filter.owner = Some(session.user);
            }
        }
        
        let mouse = &resources.mouse;
        Self::reset_hover_click(world);
        Self::handle_editing(world, keyboard);
        Self::handle_create(world, keyboard);
        Self::handle_delete(world, keyboard);
        Self::handle_due(world, keyboard);
        Self::handle_mouse_interaction(world, mouse);
        Self::handle_button_click(world);
        Self::update_last_pressed(mouse);
    }
}

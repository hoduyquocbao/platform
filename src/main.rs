mod components {
    pub mod core;
    pub mod traits;
    pub mod ui;
}
mod resources {
    pub mod input;
    pub mod font;
}
mod systems {
    pub mod command;
    pub mod filter;
    pub mod interaction;
    pub mod layout;
    pub mod persist;
    pub mod render;
    pub mod text;
    pub mod toggle;
}
mod engine;
use engine::System;

use components::core::*;
use components::ui::*;
use resources::input::mod_rs::Resources;
use resources::input::Input;
use resources::font::FontResource;
use minifb::{Key, Window, WindowOptions};
use std::sync::{Arc, Mutex};

type Entity = usize;

/// ECS World lưu trữ tất cả các component cho mọi entity.
#[derive(Default)]
pub struct World {
    pub texts: Vec<Option<Text>>,
    pub statuses: Vec<Option<Status>>,
    pub priorities: Vec<Option<Priority>>,
    pub selecteds: Vec<Option<Selected>>,
    pub editings: Vec<Option<Editing>>,
    pub visibles: Vec<Option<Visible>>,
    pub hovers: Vec<Option<Hover>>,
    pub actives: Vec<Option<Active>>,
    pub clicks: Vec<Option<Click>>,
    pub dirties: Vec<Option<Dirty>>,
    pub disableds: Vec<Option<Disabled>>,
    pub bounds: Vec<Option<Bounds>>,
    pub styles: Vec<Option<Style>>,
    pub creates: Vec<Option<Create>>,
    pub deletes: Vec<Option<Delete>>,
    pub parents: Vec<Option<Parent>>,
    pub childrens: Vec<Option<Children>>,
    pub to_delete: Vec<Option<()>>,
    pub collapseds: Vec<Option<Collapsed>>,
    pub dues: Vec<Option<Due>>,
    pub schedulings: Vec<Option<Scheduling>>,
    pub entity_count: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            texts: vec![],
            statuses: vec![],
            priorities: vec![],
            selecteds: vec![],
            editings: vec![],
            visibles: vec![],
            hovers: vec![],
            actives: vec![],
            clicks: vec![],
            dirties: vec![],
            disableds: vec![],
            bounds: vec![],
            styles: vec![],
            creates: vec![],
            deletes: vec![],
            parents: vec![],
            childrens: vec![],
            to_delete: vec![],
            collapseds: vec![],
            dues: vec![],
            schedulings: vec![],
            entity_count: 0,
        }
    }
    pub fn spawn(&mut self) -> Entity {
        let id = self.entity_count;
        self.entity_count += 1;
        self.texts.push(None);
        self.statuses.push(None);
        self.priorities.push(None);
        self.selecteds.push(None);
        self.editings.push(None);
        self.visibles.push(None);
        self.hovers.push(None);
        self.actives.push(None);
        self.clicks.push(None);
        self.dirties.push(None);
        self.disableds.push(None);
        self.bounds.push(None);
        self.styles.push(None);
        self.creates.push(None);
        self.deletes.push(None);
        self.parents.push(None);
        self.childrens.push(None);
        self.to_delete.push(None);
        self.collapseds.push(None);
        self.dues.push(None);
        self.schedulings.push(None);
        id
    }
    pub fn mark_for_delete(&mut self, id: usize) {
        self.to_delete[id] = Some(());
    }
    /// Xóa tất cả entity đã được đánh dấu to_delete và thu gọn các vector component.
    pub fn sweep(&mut self) {
        let mut keep = vec![];
        for id in 0..self.entity_count {
            if self.to_delete[id].is_none() {
                keep.push(id);
            }
        }
        macro_rules! sweep_vec {
            ($vec:expr) => {
                let mut new_vec = vec![];
                for &id in &keep {
                    new_vec.push($vec[id].take());
                }
                $vec = new_vec;
            };
        }
        sweep_vec!(self.texts);
        sweep_vec!(self.statuses);
        sweep_vec!(self.priorities);
        sweep_vec!(self.selecteds);
        sweep_vec!(self.editings);
        sweep_vec!(self.visibles);
        sweep_vec!(self.hovers);
        sweep_vec!(self.actives);
        sweep_vec!(self.clicks);
        sweep_vec!(self.dirties);
        sweep_vec!(self.disableds);
        sweep_vec!(self.bounds);
        sweep_vec!(self.styles);
        sweep_vec!(self.creates);
        sweep_vec!(self.deletes);
        sweep_vec!(self.parents);
        sweep_vec!(self.childrens);
        sweep_vec!(self.to_delete);
        sweep_vec!(self.collapseds);
        sweep_vec!(self.dues);
        sweep_vec!(self.schedulings);
        self.entity_count = keep.len();
    }
}

/// Bộ điều phối hệ thống, quản lý và thực thi các system.
#[derive(Default)]
pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
    pub fn add(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for system in self.systems.iter_mut() {
            system.run(world, resources);
        }
    }
}

/// Ứng dụng chính, chứa World, Scheduler và Resources.
#[derive(Default)]
pub struct App {
    world: World,
    scheduler: Scheduler,
    resources: Resources,
}

impl App {
    pub fn new() -> Result<Self, Error> {
        let mut app = Self {
            world: World::new(),
            scheduler: Scheduler::new(),
            resources: Resources::default(),
        };
        // Load font
        let font_bytes = std::fs::read("assets/Roboto-Regular.ttf").map_err(|_| Error::FontLoad)?;
        let font = fontdue::Font::from_bytes(font_bytes, fontdue::FontSettings::default()).map_err(|_| Error::FontLoad)?;
        app.resources.font = Some(FontResource(font));
        app.initialize();
        Ok(app)
    }

    fn initialize(&mut self) {
        use systems::{
            command::{Create, Delete},
            filter::FilterSystem,
            interaction::Interact,
            layout::Layout,
            persist::Persist,
            render::Render,
            text::TextSystem,
            toggle::Toggle,
        };
        self.scheduler.add(Box::new(Interact));
        self.scheduler.add(Box::new(FilterSystem));
        self.scheduler.add(Box::new(Layout));
        self.scheduler.add(Box::new(Create));
        self.scheduler.add(Box::new(Delete));
        self.scheduler.add(Box::new(Render));
        self.scheduler.add(Box::new(Persist));
        self.scheduler.add(Box::new(Toggle));
        self.scheduler.add(Box::new(TextSystem));
        // Khởi tạo entity mẫu với Bounds và Style
        let e0 = self.world.spawn();
        self.world.texts[e0] = Some(Text {
            value: "Task 1".to_string(),
        });
        self.world.statuses[e0] = Some(Status);
        self.world.priorities[e0] = Some(Priority);
        self.world.selecteds[e0] = None;
        self.world.visibles[e0] = Some(Visible);
        self.world.bounds[e0] = Some(Bounds {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 30.0,
        });
        self.world.styles[e0] = Some(Style { color: "blue" });
        let e1 = self.world.spawn();
        self.world.texts[e1] = Some(Text {
            value: "Task 2".to_string(),
        });
        self.world.statuses[e1] = Some(Status);
        self.world.priorities[e1] = Some(Priority);
        self.world.editings[e1] = Some(Editing);
        self.world.visibles[e1] = Some(Visible);
        self.world.bounds[e1] = Some(Bounds {
            x: 0.0,
            y: 40.0,
            width: 100.0,
            height: 30.0,
        });
        self.world.styles[e1] = Some(Style { color: "green" });
        let e2 = self.world.spawn();
        self.world.texts[e2] = Some(Text {
            value: "Task 3".to_string(),
        });
        self.world.statuses[e2] = Some(Status);
        self.world.priorities[e2] = Some(Priority);
        self.world.dirties[e2] = Some(Dirty);
        self.world.visibles[e2] = Some(Visible);
        self.world.bounds[e2] = Some(Bounds {
            x: 0.0,
            y: 80.0,
            width: 100.0,
            height: 30.0,
        });
        self.world.styles[e2] = Some(Style { color: "red" });
    }

    pub fn run_with_framebuffer(&mut self, framebuffer: &mut [u32], width: usize, height: usize) {
        self.resources.time.now += 1;
        self.resources.framebuffer = Some((framebuffer.as_mut_ptr(), width, height));
        self.scheduler.run(&mut self.world, &mut self.resources);
        self.resources.framebuffer = None;
    }

    pub fn run(&mut self) {
        self.resources.time.now += 1;
        self.scheduler.run(&mut self.world, &mut self.resources);
    }
}

// Định nghĩa wrapper để triển khai InputCallback cho Arc<Mutex<Input>>
pub struct InputCallbackArc {
    pub inner: Arc<Mutex<Input>>,
}

impl minifb::InputCallback for InputCallbackArc {
    fn add_char(&mut self, uni_char: u32) {
        self.inner.lock().unwrap().add_char(uni_char);
    }
}

#[derive(Debug)]
pub enum Error {
    FontLoad,
    // ... các loại lỗi khác trong tương lai
}

fn main() {
    let width = 800;
    let height = 600;
    let mut window = Window::new(
        "ECS Platform - Cycle 4",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open window: {}", e);
    });
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Lỗi khởi tạo ứng dụng: {:?}", e);
            std::process::exit(1);
        }
    };
    // Tạo input handler và đăng ký callback
    let input_shared = Arc::new(Mutex::new(Input::new()));
    let input_for_callback = Arc::clone(&input_shared);
    window.set_input_callback(Box::new(InputCallbackArc { inner: input_for_callback }));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Lấy input chuột
        if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            app.resources.mouse.position = (mx, my);
        }
        app.resources.mouse.pressed = window.get_mouse_down(minifb::MouseButton::Left);
        // Lấy input ký tự từ input_shared
        let chars: String = {
            let mut input = input_shared.lock().unwrap();
            let chars: String = input.take_chars().into_iter().collect();
            chars
        };
        app.resources.keyboard.chars = chars;
        // Các phím đặc biệt
        app.resources.keyboard.enter = window.is_key_down(Key::Enter);
        app.resources.keyboard.escape = window.is_key_down(Key::Escape);
        app.resources.keyboard.backspace = window.is_key_down(Key::Backspace);
        app.resources.keyboard.e = window.is_key_down(Key::E);
        app.run_with_framebuffer(&mut buffer, width, height);
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

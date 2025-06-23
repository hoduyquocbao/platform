mod components {
    pub mod core;
}
mod systems {
    pub mod interaction;
    pub mod toggle;
    pub mod layout;
    pub mod render;
    pub mod persist;
}

use components::core::*;
use systems::*;

// Entity struct
#[derive(Default)]
pub struct Entity {
    pub text: Option<Text>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub selected: Option<Selected>,
    pub editing: Option<Editing>,
    pub visible: Option<Visible>,
    pub hover: Option<Hover>,
    pub active: Option<Active>,
    pub click: Option<Click>,
    pub dirty: Option<Dirty>,
    pub disabled: Option<Disabled>,
}

pub struct World {
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }
}

pub struct Scheduler {
    systems: Vec<fn(&mut World)>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { systems: Vec::new() }
    }
    pub fn add(&mut self, system: fn(&mut World)) {
        self.systems.push(system);
    }
    pub fn run(&self, world: &mut World) {
        for system in &self.systems {
            system(world);
        }
    }
}

pub struct App {
    world: World,
    scheduler: Scheduler,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            world: World::new(),
            scheduler: Scheduler::new(),
        };
        app.initialize();
        app
    }

    fn initialize(&mut self) {
        // Đăng ký các system theo thứ tự: interaction, toggle, layout, render, persist
        self.scheduler.add(|_| interaction::interact());
        self.scheduler.add(|_| toggle::toggle());
        self.scheduler.add(|_| layout::layout());
        self.scheduler.add(|_| render::render());
        self.scheduler.add(|_| persist::persist());
        // Khởi tạo entity mẫu
        self.world.entities.push(Entity {
            text: Some(Text { value: "Task 1".to_string() }),
            status: Some(Status),
            priority: Some(Priority),
            selected: Some(Selected),
            visible: Some(Visible),
            ..Default::default()
        });
        self.world.entities.push(Entity {
            text: Some(Text { value: "Task 2".to_string() }),
            status: Some(Status),
            priority: Some(Priority),
            editing: Some(Editing),
            visible: Some(Visible),
            ..Default::default()
        });
        self.world.entities.push(Entity {
            text: Some(Text { value: "Task 3".to_string() }),
            status: Some(Status),
            priority: Some(Priority),
            dirty: Some(Dirty),
            visible: Some(Visible),
            ..Default::default()
        });
    }

    pub fn run(&mut self) {
        self.scheduler.run(&mut self.world);
    }
}

fn main() {
    let mut app = App::new();
    // Vòng lặp ứng dụng chính
    loop {
        app.run();
        std::thread::sleep(std::time::Duration::from_millis(16));
        break; // Chạy 1 frame để demo, bỏ break để chạy liên tục
    }
}

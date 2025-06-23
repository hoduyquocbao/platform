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

type Entity = usize;

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
        id
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
        self.scheduler.add(interaction::input);
        self.scheduler.add(toggle::toggle);
        self.scheduler.add(layout::layout);
        self.scheduler.add(render::render);
        self.scheduler.add(persist::persist);
        // Khởi tạo entity mẫu
        let e0 = self.world.spawn();
        self.world.texts[e0] = Some(Text { value: "Task 1".to_string() });
        self.world.statuses[e0] = Some(Status);
        self.world.priorities[e0] = Some(Priority);
        self.world.selecteds[e0] = None;
        self.world.visibles[e0] = Some(Visible);
        let e1 = self.world.spawn();
        self.world.texts[e1] = Some(Text { value: "Task 2".to_string() });
        self.world.statuses[e1] = Some(Status);
        self.world.priorities[e1] = Some(Priority);
        self.world.editings[e1] = Some(Editing);
        self.world.visibles[e1] = Some(Visible);
        let e2 = self.world.spawn();
        self.world.texts[e2] = Some(Text { value: "Task 3".to_string() });
        self.world.statuses[e2] = Some(Status);
        self.world.priorities[e2] = Some(Priority);
        self.world.dirties[e2] = Some(Dirty);
        self.world.visibles[e2] = Some(Visible);
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

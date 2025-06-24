mod engine;
mod error;
mod platform;
mod resources;
mod task;
mod ui;
mod user;

use engine::{World, Scheduler};
use resources::{Resources, Session};
use resources::font::{FontResource, FONT_DATA};
use platform::InputCallbackArc;
use task::{CreateSystem, DeleteSystem, ToggleSystem, TextSystem, PersistSystem};
use ui::{LayoutSystem, RenderSystem, InteractSystem, FilterSystem};
use user::{User, Owner};
use minifb::{Key, Window, WindowOptions};
use std::sync::{Arc, Mutex};

/// Ứng dụng chính, chứa World, Scheduler và Resources.
#[derive(Default)]
pub struct App {
    world: World,
    scheduler: Scheduler,
    resources: Resources,
}

impl App {
    pub fn new() -> Result<Self, error::Error> {
        let mut app = Self {
            world: World::new(),
            scheduler: Scheduler::new(),
            resources: Resources::default(),
        };
        // Load font từ dữ liệu nhúng
        let font = fontdue::Font::from_bytes(FONT_DATA, fontdue::FontSettings::default()).map_err(|_| error::Error::FontLoad)?;
        app.resources.font = Some(FontResource(font));
        app.initialize();
        Ok(app)
    }

    fn initialize(&mut self) {
        self.scheduler.add(Box::new(InteractSystem));
        self.scheduler.add(Box::new(FilterSystem));
        self.scheduler.add(Box::new(LayoutSystem));
        self.scheduler.add(Box::new(CreateSystem));
        self.scheduler.add(Box::new(DeleteSystem));
        self.scheduler.add(Box::new(RenderSystem));
        self.scheduler.add(Box::new(PersistSystem));
        self.scheduler.add(Box::new(ToggleSystem));
        self.scheduler.add(Box::new(TextSystem));
        
        // Tạo các user mẫu
        let user_a = self.world.spawn();
        self.world.users[user_a] = Some(User { name: "User A".to_string() });
        
        let user_b = self.world.spawn();
        self.world.users[user_b] = Some(User { name: "User B".to_string() });
        
        // Khởi tạo session với User A làm người dùng hiện tại
        self.resources.session = Some(Session { user: user_a });
        
        // Tái cấu trúc layout chính thành Master-Detail
        let root = self.world.spawn();
        self.world.bounds[root] = Some(ui::components::Bounds { x: 0.0, y: 0.0, width: 800.0, height: 600.0 });
        self.world.styles[root] = Some(ui::components::Style { color: "#f0f0f0".to_string() });
        self.world.visibles[root] = Some(ui::components::Visible);
        self.world.childrens[root] = Some(task::components::Children(vec![]));
        self.world.container[root] = Some(ui::components::Container);
        self.world.flows[root] = Some(ui::components::Flow::Row);
        // Master panel (danh sách task)
        let master_panel = self.world.spawn();
        self.world.bounds[master_panel] = Some(ui::components::Bounds { x: 0.0, y: 0.0, width: 400.0, height: 600.0 });
        self.world.styles[master_panel] = Some(ui::components::Style { color: "#ffffff".to_string() });
        self.world.visibles[master_panel] = Some(ui::components::Visible);
        self.world.childrens[master_panel] = Some(task::components::Children(vec![]));
        self.world.container[master_panel] = Some(ui::components::Container);
        self.world.flows[master_panel] = Some(ui::components::Flow::Column);
        self.world.parents[master_panel] = Some(task::components::Parent(root));
        if let Some(children) = &mut self.world.childrens[root] { children.0.push(master_panel); }
        // Detail panel (hiện thông tin chi tiết)
        let detail_panel = self.world.spawn();
        self.world.bounds[detail_panel] = Some(ui::components::Bounds { x: 400.0, y: 0.0, width: 400.0, height: 600.0 });
        self.world.styles[detail_panel] = Some(ui::components::Style { color: "#e3e3e3".to_string() });
        self.world.visibles[detail_panel] = Some(ui::components::Visible);
        self.world.childrens[detail_panel] = Some(task::components::Children(vec![]));
        self.world.container[detail_panel] = Some(ui::components::Container);
        self.world.flows[detail_panel] = Some(ui::components::Flow::Column);
        self.world.parents[detail_panel] = Some(task::components::Parent(root));
        if let Some(children) = &mut self.world.childrens[root] { children.0.push(detail_panel); }
        // Thêm các task mẫu vào master_panel với owner
        for i in 0..3 {
            let task = self.world.spawn();
            self.world.bounds[task] = Some(ui::components::Bounds { x: 0.0, y: 0.0, width: 380.0, height: 40.0 });
            self.world.styles[task] = Some(ui::components::Style { color: "#e3f2fd".to_string() });
            self.world.visibles[task] = Some(ui::components::Visible);
            self.world.texts[task] = Some(task::components::Text { value: format!("Task {}", i + 1) });
            self.world.parents[task] = Some(task::components::Parent(master_panel));
            // Gán owner cho task (luân phiên giữa User A và User B)
            let owner = if i % 2 == 0 { user_a } else { user_b };
            self.world.owners[task] = Some(Owner(owner));
            if let Some(children) = &mut self.world.childrens[master_panel] { children.0.push(task); }
        }
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

fn main() {
    let width = 800;
    let height = 600;
    let mut window = Window::new(
        "ECS Platform - Cycle 5",
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
    let input_shared = Arc::new(Mutex::new(resources::input::Input::new()));
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
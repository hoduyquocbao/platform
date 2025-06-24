mod engine;
mod error;
mod platform;
mod resources;
mod task;
mod ui;
mod user;

use engine::{World, Scheduler, Plugin};
use resources::{Resources};
use resources::font::{FontResource, FONT_DATA};
use platform::InputCallbackArc;
use minifb::{Key, Window, WindowOptions};
use std::sync::{Arc, Mutex};

/// Ứng dụng chính, chứa World, Scheduler và Resources.
#[derive(Default)]
pub struct App {
    pub world: World,
    pub scheduler: Scheduler,
    pub resources: Resources,
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
        Ok(app)
    }

    // Phương thức để thêm một plugin
    pub fn add<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(self);
        self
    }

    // Phương thức để thêm một system trực tiếp
    pub fn system<S: engine::System + 'static>(&mut self, system: S) -> &mut Self {
        self.scheduler.add(Box::new(system));
        self
    }

    // Phương thức để chạy ứng dụng
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
    
    // Lắp ráp ứng dụng từ các plugin
    app.add(user::UserPlugin)
       .add(task::Task)
       .add(ui::Ui);
    
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
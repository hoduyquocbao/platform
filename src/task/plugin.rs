use crate::engine::Plugin;
use crate::App;
use crate::task::systems::{CreateSystem, DeleteSystem, ToggleSystem, TextSystem, PersistSystem};
use crate::user::{User, Owner};
use crate::resources::Session;
use crate::ui::components::{Bounds, Style, Visible, Container, Flow};
use crate::task::components::{Children, Parent};

pub struct Task;

impl Plugin for Task {
    fn build(&self, app: &mut App) {
        // Đăng ký tất cả các system liên quan đến Task
        app.system(CreateSystem)
           .system(DeleteSystem)
           .system(ToggleSystem)
           .system(PersistSystem)
           .system(TextSystem);
        
        // Tạo các user mẫu
        let user_a = app.world.spawn();
        app.world.users[user_a] = Some(User { name: "User A".to_string() });
        
        let user_b = app.world.spawn();
        app.world.users[user_b] = Some(User { name: "User B".to_string() });
        
        // Khởi tạo session với User A làm người dùng hiện tại
        app.resources.session = Some(Session { user: user_a });
        
        // Tái cấu trúc layout chính thành Master-Detail
        let root = app.world.spawn();
        app.world.bounds[root] = Some(Bounds { x: 0.0, y: 0.0, width: 800.0, height: 600.0 });
        app.world.styles[root] = Some(Style { color: "#f0f0f0".to_string() });
        app.world.visibles[root] = Some(Visible);
        app.world.childrens[root] = Some(Children(vec![]));
        app.world.container[root] = Some(Container);
        app.world.flows[root] = Some(Flow::Row);
        
        // Master panel (danh sách task)
        let master_panel = app.world.spawn();
        app.world.bounds[master_panel] = Some(Bounds { x: 0.0, y: 0.0, width: 400.0, height: 600.0 });
        app.world.styles[master_panel] = Some(Style { color: "#ffffff".to_string() });
        app.world.visibles[master_panel] = Some(Visible);
        app.world.childrens[master_panel] = Some(Children(vec![]));
        app.world.container[master_panel] = Some(Container);
        app.world.flows[master_panel] = Some(Flow::Column);
        app.world.parents[master_panel] = Some(Parent(root));
        if let Some(children) = &mut app.world.childrens[root] { children.0.push(master_panel); }
        
        // Detail panel (hiện thông tin chi tiết)
        let detail_panel = app.world.spawn();
        app.world.bounds[detail_panel] = Some(Bounds { x: 400.0, y: 0.0, width: 400.0, height: 600.0 });
        app.world.styles[detail_panel] = Some(Style { color: "#e3e3e3".to_string() });
        app.world.visibles[detail_panel] = Some(Visible);
        app.world.childrens[detail_panel] = Some(Children(vec![]));
        app.world.container[detail_panel] = Some(Container);
        app.world.flows[detail_panel] = Some(Flow::Column);
        app.world.parents[detail_panel] = Some(Parent(root));
        if let Some(children) = &mut app.world.childrens[root] { children.0.push(detail_panel); }
        
        // Thêm các task mẫu vào master_panel với owner
        for i in 0..3 {
            let task = app.world.spawn();
            app.world.bounds[task] = Some(Bounds { x: 0.0, y: 0.0, width: 380.0, height: 40.0 });
            app.world.styles[task] = Some(Style { color: "#e3f2fd".to_string() });
            app.world.visibles[task] = Some(Visible);
            app.world.texts[task] = Some(crate::task::components::Text { value: format!("Task {}", i + 1) });
            app.world.parents[task] = Some(Parent(master_panel));
            // Gán owner cho task (luân phiên giữa User A và User B)
            let owner = if i % 2 == 0 { user_a } else { user_b };
            app.world.owners[task] = Some(Owner(owner));
            if let Some(children) = &mut app.world.childrens[master_panel] { children.0.push(task); }
        }
        
        println!("Task Plugin loaded.");
    }
} 
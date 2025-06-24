use crate::engine::Plugin;
use crate::App;

pub struct UserPlugin;

impl Plugin for UserPlugin {
    fn build(&self, _app: &mut App) {
        // Hiện tại UserPlugin không cần đăng ký system nào
        // Logic khởi tạo user đã được chuyển vào TaskPlugin
        println!("User Plugin loaded.");
    }
} 
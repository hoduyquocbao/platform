use crate::engine::Plugin;
use crate::App;
use crate::ui::{Layout, Render, Interact, Filter};

pub struct Ui;

impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        // Đăng ký tất cả các system liên quan đến UI
        app.system(Layout)
           .system(Filter)
           .system(Interact)
           .system(Render);
        
        println!("UI Plugin loaded.");
    }
} 
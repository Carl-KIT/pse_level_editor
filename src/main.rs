use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::macroquad;

mod tile;
mod level;
mod camera;
mod editor;
mod ui;
mod history;
mod brush;

use editor::LevelEditor;
use ui::UI;

// Constants
const LEVEL_WIDTH: usize = 20;
const LEVEL_HEIGHT: usize = 15;

#[macroquad::main("Level Editor")]
async fn main() {
    let mut editor = LevelEditor::new(LEVEL_WIDTH, LEVEL_HEIGHT);

    loop {
        clear_background(LIGHTGRAY);
        
        // Handle input
        editor.handle_input();
        
        // Set camera
        editor.setup_camera();
        
        // Draw level
        editor.draw_level();
        
        // Clear highlights after drawing
        editor.clear_highlights();
        
        // Reset camera for UI
        set_default_camera();
        
        // Draw UI
        egui_macroquad::ui(|egui_ctx| {
            UI::draw_all(&mut editor, egui_ctx);
        });
        
        // Draw egui
        egui_macroquad::draw();
        
        next_frame().await;
    }
}
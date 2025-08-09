use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::macroquad;

mod tile;
mod level;
mod camera;
mod editor;
mod ui;
mod history;
mod brush;
mod theme;

use editor::LevelEditor;
use ui::UI;
use theme::Theme;

// Constants
const LEVEL_WIDTH: usize = 20;
const LEVEL_HEIGHT: usize = 15;

#[macroquad::main("Level Editor")]
async fn main() {
    let mut editor = LevelEditor::new(LEVEL_WIDTH, LEVEL_HEIGHT);
    let theme = Theme::default();

    loop {
        clear_background(theme.to_macroquad_color(theme.background));
        
        // Handle input
        editor.handle_input();
        
        // Set camera
        editor.setup_camera();
        
        // Draw level
        editor.draw_level(&theme);
        
        // Clear highlights after drawing
        editor.clear_highlights();
        
        // Reset camera for UI
        set_default_camera();
        
        // Draw UI
        egui_macroquad::ui(|ctx| {
            // Apply theme to egui
            theme.apply_egui_theme(ctx);
            UI::draw_all(&mut editor, ctx, &theme);
        });
        
        // Draw egui
        egui_macroquad::draw();
        
        next_frame().await;
    }
}
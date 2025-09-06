use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::macroquad;

mod level;
mod camera;
mod editor;
mod ui;
mod tile;
mod tile_type_system;
mod tile_types;

use editor::LevelEditor;
use ui::UI;

// Removed unused imports

// Constants
const LEVEL_WIDTH: usize = 20;
const LEVEL_HEIGHT: usize = 15;

#[macroquad::main("Level Editor")]
async fn main() {
    let mut editor = LevelEditor::new(LEVEL_WIDTH, LEVEL_HEIGHT).await;

    egui_macroquad::cfg(|ctx| {
        ctx.style_mut(|style| {
            style.visuals = egui_macroquad::egui::Visuals::light();
        });
    });

    loop {

        
        clear_background(LIGHTGRAY);

        // Ensure UI uses default camera
        set_default_camera();

        // Build egui UI first to query input capture
        let mut ui_focused = false;

        egui_macroquad::ui(|ctx| {
            // Draw UI
            UI::draw_all(&mut editor, ctx);

            // After building UI, query whether egui wants the pointer/keyboard this frame
            ui_focused = ctx.wants_keyboard_input() || ctx.wants_pointer_input();
        });

        // Handle input with respect to egui capture

        if !ui_focused {
            editor.handle_input();
        }
        
        // Set camera
        editor.setup_camera();
        
        // Draw level
        editor.draw_level();
        
        // Clear highlights after drawing
        editor.clear_highlights();
        
        // Draw egui
        egui_macroquad::draw();
        
        next_frame().await;
    }
}
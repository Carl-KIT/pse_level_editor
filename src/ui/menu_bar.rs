use egui_macroquad::egui::{self, Context};
use crate::{editor::{BrushType, LevelEditor}, tile::TileType};

pub fn show_menu_bar(egui_ctx: &Context, editor: &mut LevelEditor) {
    egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Tile Selector").clicked() {
                let current_state = editor.show_tile_selector();
                editor.set_show_tile_selector(!current_state);
            }

            if ui.button("Modules View").clicked() { editor.toggle_modules_view(); }
            if ui.button("Export JSON").clicked() {
                let destination = rfd::FileDialog::new().save_file();
                
                if destination.is_none() {
                    eprintln!("Invalid destination folder");
                }

                let destination = destination.unwrap();

                if let Ok(json) = editor.level_export_json() {
                    let _ = std::fs::write(destination, json);
                } else {
                    eprintln!("Failure trying to export json");
                }
            }
            
            ui.separator();
            
            // Undo/Redo buttons
            ui.add_enabled_ui(editor.can_undo(), |ui| {
                if ui.button("Undo (Ctrl+Z)").clicked() {
                    editor.undo();
                }
            });
            
            ui.add_enabled_ui(editor.can_redo(), |ui| {
                if ui.button("Redo (Ctrl+Y)").clicked() {
                    editor.redo();
                }
            });
        });
    });
}
use egui_macroquad::egui::{self, Context};
use crate::editor::LevelEditor;

pub fn show_menu_bar(egui_ctx: &Context, editor: &mut LevelEditor) {
    egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Tile Selector").clicked() {
                let current_state = editor.show_tile_selector();
                editor.set_show_tile_selector(!current_state);
            }

            if ui.button("Modules View").clicked() { editor.toggle_modules_view(); }
            if ui.button("Export JSON").clicked() {
                let destination = rfd::FileDialog::new().add_filter("json", &["json"]).save_file();
                
                if destination.is_none() {
                    eprintln!("Invalid destination folder");
                    return;
                }

                let mut destination = destination.unwrap();
                let name = destination.file_stem();

                if name.is_none() {
                    eprintln!("Invalid destination file name");
                    return;
                }

                let name = name.unwrap().to_str().unwrap().to_string();

                if !destination.ends_with(".json") {
                    destination.set_extension("json");
                }

                if let Ok(json) = editor.level_export_json(name) {
                    println!("destination: {:?}", destination);
                    let _ = std::fs::write(destination, json);
                } else {
                    eprintln!("Failure trying to export json");
                }
            }

            if ui.button("Import JSON").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("json", &["json"]).pick_file() {
                    match std::fs::read_to_string(&path) {
                        Ok(contents) => {
                            if let Err(e) = editor.level_import_json(&contents) {
                                eprintln!("Failed to import json: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Failed to read file: {}", e),
                    }
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
use egui_macroquad::egui::{self, Context};

use crate::{editor::{BrushType, LevelEditor}, tile::TileType};

pub fn show_tiles(egui_ctx: &Context, editor: &mut LevelEditor) {
    egui::SidePanel::left("tile_selector_panel")
        .resizable(true)
        .default_width(200.0)
        .show(egui_ctx, |ui| {
            ui.heading("Select Tile");
            ui.separator();
            // Dynamic list from registry with folder-like grouping
            let selected_type = editor.selected_tile();
            let mut enemies: Vec<(String, String)> = Vec::new();
            let mut tiles_all: Vec<(String, String)> = Vec::new();
            let mut others: Vec<(String, String)> = Vec::new();

            for kind in editor.registry().kinds().iter() {
                if kind.key == "air" {
                    continue;
                }
                let key_l = kind.key.to_lowercase();
                let item = (kind.key.clone(), kind.display_name.clone());
                if key_l.contains("pig")
                    || key_l.contains("snail")
                    || key_l.contains("bird")
                    || ((key_l.contains("bear") && key_l.contains("trap"))
                        || key_l.contains("beartrap"))
                {
                    enemies.push(item);
                } else if key_l.contains("powerup")
                    || key_l.contains("ground")
                    || key_l.contains("wall")
                    || key_l.contains("ice")
                    || key_l.contains("mud")
                {
                    tiles_all.push(item);
                } else {
                    others.push(item);
                }
            }

            ui.vertical(|ui| {
                egui::CollapsingHeader::new("Enemies")
                    .default_open(true)
                    .show(ui, |ui| {
                        for (key, display_name) in &enemies {
                            let is_selected = match &selected_type {
                                TileType::Air => false,
                                TileType::Custom(k) => k == key,
                            };
                            if ui.selectable_label(is_selected, display_name).clicked() {
                                editor.set_selected_tile(TileType::Custom(key.clone()));
                            }
                        }
                    });

                egui::CollapsingHeader::new("Tiles")
                    .default_open(true)
                    .show(ui, |ui| {
                        for (key, display_name) in &tiles_all {
                            let is_selected = match &selected_type {
                                TileType::Air => false,
                                TileType::Custom(k) => k == key,
                            };
                            if ui.selectable_label(is_selected, display_name).clicked() {
                                editor.set_selected_tile(TileType::Custom(key.clone()));
                            }
                        }
                    });

                egui::CollapsingHeader::new("Other")
                    .default_open(false)
                    .show(ui, |ui| {
                        for (key, display_name) in &others {
                            let is_selected = match &selected_type {
                                TileType::Air => false,
                                TileType::Custom(k) => k == key,
                            };
                            if ui.selectable_label(is_selected, display_name).clicked() {
                                editor.set_selected_tile(TileType::Custom(key.clone()));
                            }
                        }
                    });

                ui.separator();
                // Air option (no category)
                let is_air = matches!(selected_type, TileType::Air);
                if ui.selectable_label(is_air, "Air").clicked() {
                    editor.set_selected_tile(TileType::Air);
                }
            });

            ui.separator();
            ui.heading("Brush Type");
            ui.separator();

            for brush_type in [
                BrushType::Single,
                BrushType::Fill,
                BrushType::Selector,
                BrushType::Structure,
            ] {
                let is_selected = editor.brush_type() == brush_type;

                if ui
                    .selectable_label(is_selected, brush_type.name())
                    .clicked()
                {
                    editor.set_brush_type(brush_type);
                }
            }
        });
}

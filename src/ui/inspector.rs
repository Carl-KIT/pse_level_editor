use egui_macroquad::egui::{self, Context};
use crate::editor::LevelEditor;

pub fn show_inspector(egui_ctx: &Context, editor: &mut LevelEditor) {
    let coords = editor.get_selected_tile_coords();
    egui::SidePanel::right("tile_inspector_panel")
        .resizable(true)
        .default_width(250.0)
        .show(egui_ctx, |ui| {
            ui.heading("Tile Inspector");
            ui.separator();
            
            // Tile position
            if let Some((x, y)) = coords {
                ui.label(format!("Position: ({}, {})", x, y));
            }

            // Platform info if present
            if let Some((_ptype, min_x, min_y, max_x, max_y)) = editor.get_selected_platform_info() {
                ui.separator();
                ui.label("Platform:");
                // Type display removed; dynamic types have no static name
                ui.label(format!("Bounds: ({}, {}) - ({}, {})", min_x, min_y, max_x, max_y));
                ui.label(format!("Size: {} x {}", max_x - min_x + 1, max_y - min_y + 1));
            }
            
            
            ui.separator();
            
            // Metadata UI: show structure metadata (platform or stairs) if present, else tile metadata
            if let Some((x, y)) = coords {
                // Check if this is a stairs first, then platform, then regular tile
                if let Some(stairs) = editor.level().stairs_at(x, y) {
                    // This is a stairs - show stairs metadata directly
                    ui.label("Type: Stairs");
                    ui.label(format!("Size: {} x {}", stairs.max_x - stairs.min_x + 1, stairs.max_y - stairs.min_y + 1));
                    // Edit stairs metadata directly
                    if let Some(stairs_mut) = editor.level_mut().stairs_at_mut(x, y) {
                        for field in &mut stairs_mut.metadata {
                            field.ui(ui);
                        }
                    }
                } else if let Some(_platform) = editor.level().platform_at(x, y) {
                    // This is a platform - show platform metadata directly
                    ui.label("Type: Platform");
                    // Edit platform metadata directly
                    if let Some(platform_mut) = editor.level_mut().platform_at_mut(x, y) {
                        for field in &mut platform_mut.metadata {
                            field.ui(ui);
                        }
                    }
                } else if let Some(tile) = editor.level().get_tile(x, y) {
                    // This is a regular tile
                    let tile_type_string = tile.tile_type.to_string();
                    if tile_type_string != "air" { // Skip metadata for air tiles
                        if let Some(tile_type) = editor.tile_type_registry().get(&tile_type_string) {
                            ui.label(format!("Type: {}", tile_type.display_name()));
                            // Edit tile metadata directly
                            if let Some(tile_mut) = editor.level_mut().get_tile_mut(x, y) {
                                for field in &mut tile_mut.metadata {
                                    field.ui(ui);
                                }
                            }
                        }
                    } else {
                        ui.label("Type: Air (no metadata)");
                    }
                }
            }
            
            ui.separator();
        });
}
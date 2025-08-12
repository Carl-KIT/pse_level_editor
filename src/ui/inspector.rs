use egui_macroquad::egui::{self, Context};
use crate::{editor::{BrushType, LevelEditor}, tile::{Platform, SelectableMeta, Tile, TileType}};

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
                if let Some(p) = editor.level_mut().platform_at_mut(x, y) {
                    p.metadata_ui(ui);
                } else if let Some(s) = editor.level_mut().stairs_at_mut(x, y) {
                    s.metadata_ui(ui);
                } else {
                    let mut_tile = editor.level_mut().get_tile_mut(x, y).unwrap();
                    mut_tile.metadata_ui(ui);
                }
            }
            
            ui.separator();
        });
}
use egui_macroquad::egui::{self, Context};

use crate::{editor::{Mode, LevelEditor}, tile::TileType as OldTileType};
use crate::tile_type_system::*;

pub fn show_tiles(egui_ctx: &Context, editor: &mut LevelEditor) {
    // Collect tile types first to avoid borrowing conflicts
    let all_tile_types: Vec<&TileType> = editor.tile_type_registry().all_tile_types();
    let mut tiles_by_category: std::collections::HashMap<TileCategory, Vec<(String, String)>> = std::collections::HashMap::new();
    
    for tile_type in all_tile_types {
        if tile_type.id() != "air" { // Skip air tiles
            let tile_info = (tile_type.id().to_string(), tile_type.display_name().to_string());
            tiles_by_category.entry(tile_type.category()).or_insert_with(Vec::new).push(tile_info);
        }
    }
    
    egui::SidePanel::left("tile_selector_panel")
        .resizable(true)
        .default_width(200.0)
        .show(egui_ctx, |ui| {
            ui.heading("Select Tile");
            ui.separator();
            // Use new tile type system with categories
            let selected_type = editor.selected_tile();

            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 100.0) // Reserve space for brush selection
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Display tiles by category
                        for category in [TileCategory::Tiles, TileCategory::Enemies, TileCategory::Collectables, TileCategory::Structures] {
                            if let Some(tiles) = tiles_by_category.get(&category) {
                                egui::CollapsingHeader::new(category.display_name())
                                    .default_open(true)
                                    .show(ui, |ui| {
                                        for (tile_id, tile_name) in tiles {
                                            let is_selected = match &selected_type {
                                                OldTileType::Air => false,
                                                OldTileType::Custom(k) => k == tile_id,
                                            };
                                            
                                            if ui.selectable_label(is_selected, tile_name).clicked() {
                                                editor.set_selected_tile(OldTileType::Custom(tile_id.clone()));
                                                
                                        // Automatically switch to drawing mode when selecting any tile
                                        editor.set_mode(Mode::Drawing);
                                            }
                                        }
                                    });
                            }
                        }

                        ui.separator();
                        // Air option (no category)
                        let is_air = matches!(selected_type, OldTileType::Air);
                        if ui.selectable_label(is_air, "Air").clicked() {
                            editor.set_selected_tile(OldTileType::Air);
                        }
                    });
                });

            ui.separator();
            ui.heading("Mode");
            ui.separator();

            for mode in [
                Mode::Drawing,
                Mode::Selector,
            ] {
                let is_selected = editor.mode() == mode;

                if ui
                    .selectable_label(is_selected, mode.name())
                    .clicked()
                {
                    editor.set_mode(mode);
                }
            }
        });
}

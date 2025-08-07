use crate::editor::LevelEditor;
use crate::tile::TileType;
use crate::brush::BrushType;
use egui_macroquad::egui;

pub struct UI;

impl UI {
    pub fn draw_all(editor: &mut LevelEditor, egui_ctx: &egui::Context) {
        // Draw menu bar
        egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Tile Selector").clicked() {
                    let current_state = editor.show_tile_selector();
                    editor.set_show_tile_selector(!current_state);
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

        // Draw tile selector panel
        if editor.show_tile_selector() {
            egui::SidePanel::left("tile_selector_panel")
                .resizable(true)
                .default_width(200.0)
                .show(egui_ctx, |ui| {
                    ui.heading("Select Tile");
                    ui.separator();
                    
                    for tile_type in [TileType::Air, TileType::Grass, TileType::Ground] {
                        let is_selected = editor.selected_tile() == tile_type;
                        
                        ui.horizontal(|ui| {
                            // Color preview
                            let color = tile_type.color();
                            let egui_color = egui::Color32::from_rgb(
                                (color.r * 255.0) as u8,
                                (color.g * 255.0) as u8,
                                (color.b * 255.0) as u8,
                            );
                            
                            ui.colored_label(egui_color, "■");
                            
                            // Tile name and selection
                            if ui.selectable_label(is_selected, tile_type.name()).clicked() {
                                editor.set_selected_tile(tile_type);
                            }
                        });
                    }
                    
                    ui.separator();
                    ui.label(format!("Selected: {}", editor.selected_tile().name()));
                    
                    ui.separator();
                    ui.heading("Brush Type");
                    ui.separator();
                    
                    for brush_type in [BrushType::Single, BrushType::Fill, BrushType::Selector] {
                        let is_selected = editor.brush_type() == brush_type;
                        
                        if ui.selectable_label(is_selected, brush_type.name()).clicked() {
                            editor.set_brush_type(brush_type);
                        }
                    }
                    
                    ui.separator();
                    ui.label(format!("Brush: {}", editor.brush_type().name()));
                    
                    // Show brush instructions
                    ui.separator();
                    ui.heading("Instructions");
                    match editor.brush_type() {
                        BrushType::Single => {
                            ui.label("• Click and drag to place tiles");
                        },
                        BrushType::Fill => {
                            ui.label("• Click and drag to create fill area");
                            ui.label("• Right-click to cancel");
                            ui.label("• Release to apply fill");
                        },
                        BrushType::Selector => {
                            ui.label("• Click to select a tile");
                            ui.label("• Edit tile attributes in inspector");
                        },
                    }
                });
        }

        // Draw tile inspector panel
        if let Some(tile) = editor.get_selected_tile() {
            let coords = editor.get_selected_tile_coords();
            let tile_type = tile.tile_type;
            let tile_name = tile.name.clone();
            let tile_color = tile.color();
            
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
                    
                    ui.separator();
                    
                    // Tile type selection
                    ui.label("Tile Type:");
                    for tile_type_option in [TileType::Air, TileType::Grass, TileType::Ground] {
                        let is_selected = tile_type == tile_type_option;
                        if ui.selectable_label(is_selected, tile_type_option.name()).clicked() {
                            editor.update_selected_tile(tile_type_option);
                        }
                    }
                    
                    ui.separator();
                    
                    // Tile name editing
                    ui.label("Name:");
                    let mut name = tile_name;
                    if ui.text_edit_singleline(&mut name).changed() {
                        editor.update_selected_tile_name(name);
                    }
                    
                    ui.separator();
                    
                    // Color preview
                    ui.label("Color Preview:");
                    let egui_color = egui::Color32::from_rgb(
                        (tile_color.r * 255.0) as u8,
                        (tile_color.g * 255.0) as u8,
                        (tile_color.b * 255.0) as u8,
                    );
                    ui.colored_label(egui_color, "■");
                });
        }
    }
} 
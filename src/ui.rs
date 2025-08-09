use crate::editor::LevelEditor;
use crate::tile::TileType;
use crate::brush::BrushType;
use crate::theme::Theme;
use egui_macroquad::egui;

pub struct UI;

impl UI {
    pub fn draw_all(editor: &mut LevelEditor, egui_ctx: &egui::Context, theme: &Theme) {
        // Draw menu bar
        egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                // Apply custom styling to buttons to ensure proper text color

                if ui.button("Tile Selector").clicked() {
                    let current_state = editor.show_tile_selector();
                    editor.set_show_tile_selector(!current_state);
                }
                
                ui.separator();
                
                // Undo/Redo buttons with proper text color
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
                            // Color preview using theme colors
                            let color = theme.tile_color(tile_type);
                            
                            ui.colored_label(color, "■");
                            
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
                });
        }

        // Draw tile inspector panel
        if let Some(tile) = editor.get_selected_tile() {
            let coords = editor.get_selected_tile_coords();
            let tile_type = tile.tile_type;
            let tile_name = tile.name.clone();
            let tile_color = theme.tile_color(tile.tile_type);
            
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
                    
                    // Tile name editing with proper styling
                    ui.label("Name:");
                    let mut name = tile_name;
                    let text_edit = egui::TextEdit::singleline(&mut name)
                        .text_color(theme.text_primary);
                    if ui.add(text_edit).changed() {
                        editor.update_selected_tile_name(name);
                    }
                    
                    ui.separator();
                    
                    // Color preview using theme colors
                    ui.label("Color Preview:");
                    ui.colored_label(tile_color, "■");
                });
        }
    }
} 
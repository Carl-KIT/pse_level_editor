use crate::editor::LevelEditor;
use crate::tile::{TileType, SelectableMeta};
use crate::editor::BrushType;
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

        // Draw tile selector panel, hidden when Structure brush is active
        if editor.show_tile_selector() && !matches!(editor.brush_type(), BrushType::Structure) {
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
                        if kind.key == "air" { continue; }
                        let key_l = kind.key.to_lowercase();
                        let item = (kind.key.clone(), kind.display_name.clone());
                        if key_l.contains("pig") || key_l.contains("snail") || key_l.contains("bird") ||
                           ((key_l.contains("bear") && key_l.contains("trap")) || key_l.contains("beartrap")) {
                            enemies.push(item);
                        } else if key_l.contains("powerup") || key_l.contains("ground") || key_l.contains("wall") {
                            tiles_all.push(item);
                        } else {
                            others.push(item);
                        }
                    }

                    ui.vertical(|ui| {
                        egui::CollapsingHeader::new("Enemies").default_open(true).show(ui, |ui| {
                            for (key, display_name) in &enemies {
                                let is_selected = match &selected_type { TileType::Air => false, TileType::Custom(k) => k == key };
                                if ui.selectable_label(is_selected, display_name).clicked() {
                                    editor.set_selected_tile(TileType::Custom(key.clone()));
                                }
                            }
                        });

                        egui::CollapsingHeader::new("Tiles").default_open(true).show(ui, |ui| {
                            for (key, display_name) in &tiles_all {
                                let is_selected = match &selected_type { TileType::Air => false, TileType::Custom(k) => k == key };
                                if ui.selectable_label(is_selected, display_name).clicked() {
                                    editor.set_selected_tile(TileType::Custom(key.clone()));
                                }
                            }
                        });

                        egui::CollapsingHeader::new("Other").default_open(false).show(ui, |ui| {
                            for (key, display_name) in &others {
                                let is_selected = match &selected_type { TileType::Air => false, TileType::Custom(k) => k == key };
                                if ui.selectable_label(is_selected, display_name).clicked() {
                                    editor.set_selected_tile(TileType::Custom(key.clone()));
                                }
                            }
                        });

                        ui.separator();
                        // Air option (no category)
                        let is_air = matches!(selected_type, TileType::Air);
                        if ui.selectable_label(is_air, "Air").clicked() { editor.set_selected_tile(TileType::Air); }
                    });
                    
                    ui.separator();
                    ui.heading("Brush Type");
                    ui.separator();
                    
                    for brush_type in [BrushType::Single, BrushType::Fill, BrushType::Selector, BrushType::Structure] {
                        let is_selected = editor.brush_type() == brush_type;
                        
                        if ui.selectable_label(is_selected, brush_type.name()).clicked() {
                            editor.set_brush_type(brush_type);
                        }
                    }
                
                });
        }

        // Structure panel when Structure brush active
        if matches!(editor.brush_type(), BrushType::Structure) {
            egui::SidePanel::left("structure_panel")
                .resizable(true)
                .default_width(200.0)
                .show(egui_ctx, |ui| {
                    ui.heading("Structures");
                    ui.separator();
                    let modes = [
                        ("Platform", crate::editor::StructureMode::PlatformRect),
                        ("Stairs", crate::editor::StructureMode::Stairs),
                    ];
                    for (label, mode) in modes {
                        let is_selected = editor.structure_mode() == mode;
                        if ui.selectable_label(is_selected, label).clicked() {
                            editor.set_structure_mode(mode);
                        }
                    }

                    ui.separator();
                    ui.heading("Brush Type");
                    ui.separator();
                    for brush_type in [BrushType::Single, BrushType::Fill, BrushType::Selector, BrushType::Structure] {
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
                    
                    // Metadata UI: show platform metadata if belongs to a platform, else tile metadata
                    if let Some((x, y)) = coords {
                        if let Some(p) = editor.level_mut().platform_at_mut(x, y) {
                            let mut proxy = p; // mutable access
                            proxy.metadata_ui(ui);
                        } else {
                            let mut_tile = editor.level_mut().get_tile_mut(x, y).unwrap();
                            mut_tile.metadata_ui(ui);
                        }
                    }
                    
                    ui.separator();
                });
        }
    }
} 
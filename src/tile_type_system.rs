use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Categories for organizing tile types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileCategory {
    Tiles,
    Structures,
    Enemies,
    Collectables,
}

impl TileCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            TileCategory::Tiles => "Tiles",
            TileCategory::Structures => "Structures", 
            TileCategory::Enemies => "Enemies",
            TileCategory::Collectables => "Collectables",
        }
    }
}

// Brush types that can be used with tile types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrushType {
    Drawing,
    Selector,
}

impl BrushType {
    pub fn name(&self) -> &'static str {
        match self {
            BrushType::Drawing => "Drawing",
            BrushType::Selector => "Selector",
        }
    }
}

// Metadata field types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MetaField {
    Number { 
        key: String, 
        label: String, 
        value: f32, 
        min: f32, 
        max: f32,
        editable: bool,
    },
    Text { 
        key: String, 
        label: String, 
        value: String,
        editable: bool,
    },
    Bool { 
        key: String, 
        label: String, 
        value: bool, 
        editable: bool,
    },
    Label { 
        label: String, 
        value: String,
    },
    Choice { 
        key: String, 
        label: String, 
        options: Vec<String>, 
        selected: usize,
        editable: bool,
    },
}

impl MetaField {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            MetaField::Number { label, value, min, max, editable, .. } => {
                ui.horizontal(|ui| {
                    ui.label(label.clone());
                    let mut v = *value;
                    if ui.add_enabled(*editable, egui::Slider::new(&mut v, *min..=*max)).changed() { 
                        *value = v; 
                    }
                });
            }
            MetaField::Text { label, value, editable, .. } => {
                ui.horizontal(|ui| {
                    ui.label(label.clone());
                    let mut buf = value.clone();
                    if ui.add_enabled(*editable, egui::TextEdit::singleline(&mut buf)).changed() { 
                        *value = buf; 
                    }
                });
            }
            MetaField::Bool { label, value, editable, .. } => {
                ui.horizontal(|ui| {
                    ui.label(label.clone());
                    let mut b = *value;
                    ui.add_enabled(*editable, egui::Checkbox::new(&mut b, ""));
                    *value = b;
                });
            }
            MetaField::Label { label, value } => {
                ui.horizontal(|ui| {
                    ui.label(label.clone());
                    ui.label(value.clone());
                });
            }
            MetaField::Choice { label, options, selected, .. } => {
                ui.horizontal(|ui| {
                    ui.label(label.clone());
                    egui::ComboBox::new(egui::Id::new(label.clone()), "")
                        .selected_text(options.get(*selected).cloned().unwrap_or_else(|| "-".to_string()))
                        .show_ui(ui, |ui| {
                            for (i, opt) in options.iter().enumerate() {
                                if ui.selectable_label(*selected == i, opt.clone()).clicked() { 
                                    *selected = i; 
                                }
                            }
                        });
                });
            }
        }
    }
    
    pub fn key(&self) -> &str {
        match self {
            MetaField::Number { key, .. } => key,
            MetaField::Text { key, .. } => key,
            MetaField::Bool { key, .. } => key,
            MetaField::Label { .. } => "",
            MetaField::Choice { key, .. } => key,
        }
    }
    
    pub fn is_editable(&self) -> bool {
        match self {
            MetaField::Number { editable, .. } => *editable,
            MetaField::Text { editable, .. } => *editable,
            MetaField::Bool { editable, .. } => *editable,
            MetaField::Label { .. } => false,
            MetaField::Choice { editable, .. } => *editable,
        }
    }
}

// Common metadata fields that all tile types should have
pub fn create_common_metadata() -> Vec<MetaField> {
    vec![
        MetaField::Text { 
            key: "objectID".to_string(), 
            label: "Object ID".to_string(), 
            value: String::new(),
            editable: true,
        },
        MetaField::Label { 
            label: "type".to_string(), 
            value: String::new(), // Will be set by the tile type
        },
        MetaField::Bool { 
            key: "enabled".to_string(), 
            label: "Enabled".to_string(), 
            value: true, 
            editable: false,
        },
        MetaField::Bool { 
            key: "mutable".to_string(), 
            label: "Mutable".to_string(), 
            value: false, 
            editable: true,
        },
    ]
}

// Enum-based tile type system
#[derive(Clone, Debug)]
pub enum TileType {
    BasicTile {
        id: String,
        display_name: String,
        category: TileCategory,
        texture: Texture2D,
        metadata: Vec<MetaField>,
        position: Option<(u32, u32)>,
    },
    PlatformTile {
        id: String,
        display_name: String,
        min_x: usize,
        min_y: usize,
        max_x: usize,
        max_y: usize,
        metadata: Vec<MetaField>,
        texture: Texture2D,
    },
    StairsTile {
        id: String,
        display_name: String,
        min_x: usize,
        min_y: usize,
        max_x: usize,
        max_y: usize,
        orientation: i32,
        metadata: Vec<MetaField>,
        texture: Texture2D,
    },
}

impl TileType {
    pub fn id(&self) -> &str {
        match self {
            TileType::BasicTile { id, .. } => id,
            TileType::PlatformTile { id, .. } => id,
            TileType::StairsTile { id, .. } => id,
        }
    }
    
    pub fn display_name(&self) -> &str {
        match self {
            TileType::BasicTile { display_name, .. } => display_name,
            TileType::PlatformTile { display_name, .. } => display_name,
            TileType::StairsTile { display_name, .. } => display_name,
        }
    }
    
    pub fn category(&self) -> TileCategory {
        match self {
            TileType::BasicTile { category, .. } => *category,
            TileType::PlatformTile { .. } => TileCategory::Structures,
            TileType::StairsTile { .. } => TileCategory::Structures,
        }
    }
    
    pub fn texture(&self) -> &Texture2D {
        match self {
            TileType::BasicTile { texture, .. } => texture,
            TileType::PlatformTile { texture, .. } => texture,
            TileType::StairsTile { texture, .. } => texture,
        }
    }
    
    
    pub fn position(&self) -> Option<(u32, u32)> {
        match self {
            TileType::BasicTile { position, .. } => *position,
            TileType::PlatformTile { min_x, min_y, .. } => Some((*min_x as u32, *min_y as u32)),
            TileType::StairsTile { min_x, min_y, .. } => Some((*min_x as u32, *min_y as u32)),
        }
    }
    
    pub fn set_position(&mut self, pos: (u32, u32)) {
        match self {
            TileType::BasicTile { position, .. } => *position = Some(pos),
            TileType::PlatformTile { min_x, min_y, max_x, max_y, .. } => {
                let width = *max_x - *min_x + 1;
                let height = *max_y - *min_y + 1;
                *min_x = pos.0 as usize;
                *min_y = pos.1 as usize;
                *max_x = *min_x + width - 1;
                *max_y = *min_y + height - 1;
            },
            TileType::StairsTile { min_x, min_y, max_x, max_y, .. } => {
                let width = *max_x - *min_x + 1;
                let height = *max_y - *min_y + 1;
                *min_x = pos.0 as usize;
                *min_y = pos.1 as usize;
                *max_x = *min_x + width - 1;
                *max_y = *min_y + height - 1;
            },
        }
    }
    
    pub fn size(&self) -> Option<(u32, u32)> {
        match self {
            TileType::BasicTile { .. } => None,
            TileType::PlatformTile { min_x, min_y, max_x, max_y, .. } => {
                Some(((max_x - min_x + 1) as u32, (max_y - min_y + 1) as u32))
            },
            TileType::StairsTile { min_x, min_y, max_x, max_y, .. } => {
                let size = (max_x - min_x + 1).max(max_y - min_y + 1);
                Some((size as u32, size as u32))
            },
        }
    }
    
    pub fn can_use_brush(&self, brush_type: BrushType) -> bool {
        match self.category() {
            TileCategory::Tiles => matches!(brush_type, BrushType::Drawing | BrushType::Selector),
            TileCategory::Enemies => matches!(brush_type, BrushType::Drawing | BrushType::Selector),
            TileCategory::Collectables => matches!(brush_type, BrushType::Drawing | BrushType::Selector),
            TileCategory::Structures => matches!(brush_type, BrushType::Drawing | BrushType::Selector),
        }
    }
    
    pub fn metadata(&self) -> &Vec<MetaField> {
        match self {
            TileType::BasicTile { metadata, .. } => metadata,
            TileType::PlatformTile { metadata, .. } => metadata,
            TileType::StairsTile { metadata, .. } => metadata,
        }
    }
    
    pub fn metadata_mut(&mut self) -> &mut Vec<MetaField> {
        match self {
            TileType::BasicTile { metadata, .. } => metadata,
            TileType::PlatformTile { metadata, .. } => metadata,
            TileType::StairsTile { metadata, .. } => metadata,
        }
    }
    
    pub fn editable_metadata(&self) -> Vec<MetaField> {
        self.metadata().iter().filter(|f| f.is_editable()).cloned().collect()
    }
    
    pub fn render_metadata_ui(&mut self, ui: &mut egui::Ui) {
        // Default implementation: show type as title and render all metadata fields
        ui.label(format!("{} Metadata:", self.display_name()));
        
        // Show position if available
        if let Some(pos) = self.position() {
            ui.label(format!("Position: ({}, {})", pos.0, pos.1));
        }
        
        // Show size if available
        if let Some(size) = self.size() {
            ui.label(format!("Size: {} x {}", size.0, size.1));
        }
        
        // Render all metadata fields
        for field in self.metadata_mut() {
            field.ui(ui);
        }
    }
}

// Registry for managing all tile types
pub struct TileTypeRegistry {
    tile_types: HashMap<String, TileType>,
    category_index: HashMap<TileCategory, Vec<String>>,
}

impl TileTypeRegistry {
    pub fn new() -> Self {
        Self {
            tile_types: HashMap::new(),
            category_index: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, tile_type: TileType) {
        let id = tile_type.id().to_string();
        let category = tile_type.category();
        
        self.tile_types.insert(id.clone(), tile_type);
        self.category_index.entry(category).or_insert_with(Vec::new).push(id);
    }
    
    pub fn get(&self, id: &str) -> Option<&TileType> {
        self.tile_types.get(id)
    }
    
    pub fn get_mut(&mut self, id: &str) -> Option<&mut TileType> {
        self.tile_types.get_mut(id)
    }
    
    pub fn get_by_category(&self, category: TileCategory) -> Vec<&TileType> {
        self.category_index
            .get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.tile_types.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    pub fn all_tile_types(&self) -> Vec<&TileType> {
        self.tile_types.values().collect()
    }
}

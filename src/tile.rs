use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Platform {
    pub tile_type: TileType,
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
    pub metadata: Vec<MetaField>,
}

impl Platform {
    pub fn width(&self) -> usize { self.max_x - self.min_x + 1 }
    pub fn height(&self) -> usize { self.max_y - self.min_y + 1 }
}

#[derive(Clone, Debug)]
pub struct Stairs {
    pub tile_type: TileType,
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
    pub metadata: Vec<MetaField>,
}

pub trait SelectableMeta {
    fn metadata_ui(&mut self, ui: &mut egui::Ui);
}

#[derive(Clone, Debug)]
pub enum MetaField {
    Number { key: &'static str, label: &'static str, value: f32, min: f32, max: f32 },
    Text { key: &'static str, label: &'static str, value: String },
    Bool { key: &'static str, label: &'static str, value: bool, editable: bool },
    Label { label: &'static str, value: String },
    Choice { key: &'static str, label: &'static str, options: Vec<&'static str>, selected: usize },
}

impl MetaField {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            MetaField::Number { label, value, min, max, .. } => {
                ui.horizontal(|ui| {
                    ui.label(*label);
                    let mut v = *value;
                    if ui.add(egui::Slider::new(&mut v, *min..=*max)).changed() { *value = v; }
                });
            }
            MetaField::Text { label, value, .. } => {
                ui.horizontal(|ui| {
                    ui.label(*label);
                    let mut buf = value.clone();
                    if ui.text_edit_singleline(&mut buf).changed() { *value = buf; }
                });
            }
            MetaField::Bool { label, value, editable, .. } => {
                ui.horizontal(|ui| {
                    ui.label(*label);
                    let mut b = *value;
                    ui.add_enabled(*editable, egui::Checkbox::new(&mut b, ""));
                    *value = b;
                });
            }
            MetaField::Label { label, value } => {
                ui.horizontal(|ui| {
                    ui.label(*label);
                    ui.label(value.clone());
                });
            }
            MetaField::Choice { label, options, selected, .. } => {
                ui.horizontal(|ui| {
                    ui.label(*label);
                    egui::ComboBox::from_id_source(label.to_string())
                        .selected_text(options.get(*selected).copied().unwrap_or("-"))
                        .show_ui(ui, |ui| {
                            for (i, opt) in options.iter().enumerate() {
                                if ui.selectable_label(*selected == i, *opt).clicked() { *selected = i; }
                            }
                        });
                });
            }
        }
    }
}


// Tile type abstraction: keep Air as a special, everything else is dynamic by key
#[derive(Clone, PartialEq, Debug)]
pub enum TileType {
    Air,
    Custom(String),
}

impl Default for TileType { fn default() -> Self { TileType::Air } }

// Complete tile with editable attributes
#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub name: String,
    pub metadata: Vec<MetaField>,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        let name = match &tile_type { TileType::Air => "Air".to_string(), TileType::Custom(k) => k.clone() };
        let metadata = default_tile_metadata_for(&tile_type);
        Self { tile_type, name, metadata }
    }

    pub fn color(&self) -> Color { WHITE }

    pub fn set_tile_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
        self.name = match &self.tile_type { TileType::Air => "Air".to_string(), TileType::Custom(k) => k.clone() };
        self.metadata = default_tile_metadata_for(&self.tile_type);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl SelectableMeta for Tile {
    fn metadata_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Tile Metadata:");
        for field in &mut self.metadata { field.ui(ui); }
    }
}

impl SelectableMeta for Platform {
    fn metadata_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Platform Metadata:");
        // Computed fields: position and size
        ui.label(format!("Position: ({}, {})", self.min_x, self.min_y));
        ui.label(format!("Size: {} x {}", self.width(), self.height()));
        for field in &mut self.metadata { field.ui(ui); }
    }
}

impl SelectableMeta for Stairs {
    fn metadata_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Stairs Metadata:");
        // Computed fields: position and size (single value since square-ish by design)
        ui.label(format!("Position: ({}, {})", self.min_x, self.min_y));
        let size = (self.max_x - self.min_x + 1).max(self.max_y - self.min_y + 1);
        ui.label(format!("Size: {}", size));
        for field in &mut self.metadata { field.ui(ui); }
    }
}

pub fn default_tile_metadata_for(tile_type: &TileType) -> Vec<MetaField> {
    let mut fields: Vec<MetaField> = Vec::new();
    // Common fields
    fields.push(MetaField::Text { key: "object_id", label: "Object ID", value: String::new() });
    let type_label = match tile_type { TileType::Air => "Air".to_string(), TileType::Custom(k) => k.clone() };
    fields.push(MetaField::Label { label: "Type", value: type_label });
    fields.push(MetaField::Bool { key: "enabled", label: "Enabled", value: true, editable: false });
    fields.push(MetaField::Bool { key: "mutable", label: "Mutable", value: false, editable: true });

    // Special-case: powerup tile
    if let TileType::Custom(k) = tile_type {
        let kl = k.to_lowercase();
        if kl.contains("powerup") {
            fields.push(MetaField::Text { key: "powerup", label: "Powerup", value: String::new() });
        }
        // Special-case: pig and bird speed
        if kl.contains("pig") || kl.contains("bird") {
            fields.push(MetaField::Number { key: "speed", label: "Speed", value: 1.0, min: 0.0, max: 10.0 });
        }
    }
    fields
}

pub fn default_platform_metadata_for(_tile_type: TileType) -> Vec<MetaField> {
    vec![
        MetaField::Text { key: "object_id", label: "Object ID", value: String::new() },
        MetaField::Label { label: "Type", value: "Platform".to_string() },
        MetaField::Bool { key: "enabled", label: "Enabled", value: true, editable: false },
        MetaField::Bool { key: "mutable", label: "Mutable", value: false, editable: true },
    ]
}

pub fn default_stairs_metadata_for(_tile_type: TileType) -> Vec<MetaField> {
    vec![
        MetaField::Text { key: "object_id", label: "Object ID", value: String::new() },
        MetaField::Label { label: "Type", value: "Stairs".to_string() },
        MetaField::Bool { key: "enabled", label: "Enabled", value: true, editable: false },
        MetaField::Bool { key: "mutable", label: "Mutable", value: false, editable: true },
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlatformGroup { Grass, Ground, Wall }

#[derive(Clone)]
pub struct TileKind {
    pub key: String,
    pub display_name: String,
    pub texture: Option<Texture2D>,
    pub platform_group: Option<PlatformGroup>,
}

pub struct TileRegistry {
    kinds: Vec<TileKind>,
    name_to_index: HashMap<String, usize>,
}

impl TileRegistry {
    pub async fn load_from_dir(dir: &str) -> TileRegistry {
        let mut kinds: Vec<TileKind> = Vec::new();
        let mut name_to_index: HashMap<String, usize> = HashMap::new();

        // Always include Air as index 0
        kinds.push(TileKind { key: "air".into(), display_name: "Air".into(), texture: None, platform_group: None });
        name_to_index.insert("air".into(), 0);

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext = ext.to_lowercase();
                        if ext == "png" || ext == "jpg" || ext == "jpeg" {
                            let key = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                            let display_name = key.replace('_', " ");
                            let tex_path = path.to_string_lossy().to_string();
                            let texture = load_texture(&tex_path).await.ok();
                            let platform_group = infer_platform_group_from_key(&key);
                            let idx = kinds.len();
                            kinds.push(TileKind { key: key.clone(), display_name, texture, platform_group });
                            name_to_index.insert(key, idx);
                        }
                    }
                }
            }
        }

        TileRegistry { kinds, name_to_index }
    }

    pub fn kinds(&self) -> &[TileKind] { &self.kinds }
    pub fn get(&self, key: &str) -> Option<&TileKind> { self.name_to_index.get(key).and_then(|&i| self.kinds.get(i)) }
    pub fn texture_for(&self, tile_type: &TileType) -> Option<&Texture2D> {
        match tile_type { TileType::Air => None, TileType::Custom(k) => self.get(k).and_then(|t| t.texture.as_ref()) }
    }
    pub fn platform_group_for(&self, tile_type: &TileType) -> Option<PlatformGroup> {
        match tile_type { TileType::Air => None, TileType::Custom(k) => self.get(k).and_then(|t| t.platform_group) }
    }
}

fn infer_platform_group_from_key(key: &str) -> Option<PlatformGroup> {
    let k = key.to_lowercase();
    if k.contains("grass") { Some(PlatformGroup::Grass) }
    else if k.contains("ground") { Some(PlatformGroup::Ground) }
    else if k.contains("wall") { Some(PlatformGroup::Wall) }
    else { None }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new(TileType::Air)
    }
} 
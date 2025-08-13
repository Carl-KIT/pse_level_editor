pub mod history;
use history::*;
use crate::tile::*;

use egui_macroquad::macroquad::prelude::*;
use serde::Serialize;
use serde_json::json;
mod platform_ext;

// Constants
const GRID_LINE_WIDTH: f32 = 0.05;
const TILE_SIZE: f32 = 1.0;

// Level structure
pub struct Level {
    pub(crate) tiles: Vec<Vec<Tile>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    history: HistoryManager,
    pub(crate) current_operation: Option<TileOperation>,
    pub(crate) highlighted_tiles: Vec<(usize, usize)>, // (x, y) coordinates of tiles to highlight
    pub(crate) platforms: Vec<Platform>,
    pub(crate) platform_map: Vec<Vec<Option<usize>>>, // index into platforms
    pub(crate) stairs: Vec<Stairs>,
    pub(crate) stairs_map: Vec<Vec<Option<usize>>>, // index into stairs
    // Modules: sequence of x-spans. Borders are cumulative sums starting at 0
    pub(crate) modules: Vec<usize>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::default(); width]; height];
        Self {
            tiles,
            width,
            height,
            history: HistoryManager::new(100), // Allow up to 100 operations
            current_operation: None,
            highlighted_tiles: Vec::new(),
            platforms: Vec::new(),
            platform_map: vec![vec![None; width]; height],
            stairs: Vec::new(),
            stairs_map: vec![vec![None; width]; height],
            modules: Vec::new(),
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile_type: TileType) {
        if x < self.width && y < self.height {
            let old_tile_type = self.tiles[y][x].tile_type.clone();
            if old_tile_type != tile_type {
                // Start a new operation if we don't have one
                if self.current_operation.is_none() {
                    self.current_operation = Some(TileOperation::new("Tile Placement".to_string()));
                }
                
                // Add the change to the current operation
                if let Some(ref mut operation) = self.current_operation {
                    operation.add_change(x, y, old_tile_type, tile_type.clone());
                }
                
                self.tiles[y][x].set_tile_type(tile_type);

                // Opportunistically update platforms locally for maintainability
                self.try_update_platforms_locally(x, y);
                // When changing a tile type, clear its stairs membership. Creators will reassign.
                self.stairs_map[y][x] = None;
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < self.width && y < self.height {
            Some(&mut self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn fill_rectangle(&mut self, min_x: usize, min_y: usize, max_x: usize, max_y: usize, tile_type: TileType) {
        // Start a new operation for the fill
        self.current_operation = Some(TileOperation::new("Fill Area".to_string()));
        
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x < self.width && y < self.height {
                    let old_tile_type = self.tiles[y][x].tile_type.clone();
                    if old_tile_type != tile_type {
                        // Add the change to the current operation
                        if let Some(ref mut operation) = self.current_operation {
                            operation.add_change(x, y, old_tile_type, tile_type.clone());
                        }
                        
                        self.tiles[y][x].set_tile_type(tile_type.clone());
                    }
                }
            }
        }
    }

    pub fn set_highlighted_tiles(&mut self, tiles: Vec<(usize, usize)>) {
        self.highlighted_tiles = tiles;
    }

    pub fn add_highlighted_tile(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.highlighted_tiles.push((x, y));
        }
    }

    pub fn clear_highlights(&mut self) {
        self.highlighted_tiles.clear();
    }

    pub fn finish_operation(&mut self) {
        if let Some(operation) = self.current_operation.take() {
            if !operation.is_empty() {
                self.history.add_operation(operation);
                self.rebuild_platforms();
            }
        }
    }

    pub fn undo(&mut self) {
        if let Some(operation) = self.history.undo() {
            // Apply the reverse of the operation
            for change in &operation.changes {
                self.tiles[change.y][change.x].set_tile_type(change.old_tile.clone());
            }
            self.rebuild_platforms();
        }
    }

    pub fn redo(&mut self) {
        if let Some(operation) = self.history.redo() {
            // Apply the operation
            for change in &operation.changes {
                self.tiles[change.y][change.x].set_tile_type(change.new_tile.clone());
            }
            self.rebuild_platforms();
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    // Resize the level width (x-size). Y-size remains static
    pub fn resize_width(&mut self, new_width: usize) {
        if new_width == self.width { return; }
        if new_width > self.width {
            let extra = new_width - self.width;
            for y in 0..self.height {
                for _ in 0..extra { self.tiles[y].push(Tile::default()); }
            }
            for y in 0..self.height {
                for _ in 0..extra { self.platform_map[y].push(None); }
            }
            for y in 0..self.height {
                for _ in 0..extra { self.stairs_map[y].push(None); }
            }
            self.width = new_width;
        } else {
            // Shrink
            for y in 0..self.height {
                self.tiles[y].truncate(new_width);
                self.platform_map[y].truncate(new_width);
                self.stairs_map[y].truncate(new_width);
            }
            self.width = new_width;
            // Cleanup stairs vector to remove any entries no longer referenced
            self.compact_stairs_after_resize();
        }
        // Rebuild platforms to reflect new width
        self.rebuild_platforms();
    }

    fn compact_stairs_after_resize(&mut self) {
        use std::collections::BTreeSet;
        let mut used: BTreeSet<usize> = BTreeSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(idx) = self.stairs_map[y][x] { used.insert(idx); }
            }
        }
        if used.is_empty() {
            self.stairs.clear();
            return;
        }
        let mut old_to_new: Vec<Option<usize>> = vec![None; self.stairs.len()];
        let mut new_vec: Vec<Stairs> = Vec::with_capacity(used.len());
        for (old_idx, s) in self.stairs.iter().enumerate() {
            if used.contains(&old_idx) {
                let new_idx = new_vec.len();
                new_vec.push(s.clone());
                old_to_new[old_idx] = Some(new_idx);
            }
        }
        for y in 0..self.height { for x in 0..self.width { if let Some(old_idx) = self.stairs_map[y][x] { self.stairs_map[y][x] = old_to_new[old_idx]; } } }
        self.stairs = new_vec;
    }

    pub fn platforms(&self) -> &Vec<Platform> { &self.platforms }

    pub fn platform_at(&self, x: usize, y: usize) -> Option<&Platform> {
        if x < self.width && y < self.height {
            if let Some(idx) = self.platform_map[y][x] { self.platforms.get(idx) } else { None }
        } else { None }
    }

    pub fn platform_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Platform> {
        if x < self.width && y < self.height {
            if let Some(idx) = self.platform_map[y][x] { self.platforms.get_mut(idx) } else { None }
        } else { None }
    }

    pub fn draw(&self, registry: &TileRegistry) {
        // Draw the base level
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y][x];
                let rect = Rect::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                );
                if let Some(tex) = registry.texture_for(&tile.tile_type) {
                    draw_texture_ex(tex, rect.x, rect.y, WHITE, DrawTextureParams { dest_size: Some(vec2(rect.w, rect.h)), ..Default::default() });
                } else {
                    // Fallback color for Air or missing textures
                    let color = match tile.tile_type { TileType::Air => WHITE, _ => GRAY };
                    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
                }
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, GRID_LINE_WIDTH, BLACK);
            }
        }

        // Draw highlighted tiles
        for &(x, y) in &self.highlighted_tiles {
            let rect = Rect::new(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
            );
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(1.0, 1.0, 0.0, 0.5)); // Semi-transparent yellow
        }

        // Draw module borders as vertical red lines at cumulative x positions
        let borders = self.module_borders();
        for bx in borders {
            let x = bx as f32 * TILE_SIZE;
            draw_line(x, 0.0, x, self.height as f32 * TILE_SIZE, 0.1, RED);
        }
    }

    pub fn draw_selection_indicator(&self, selected_coords: Option<(usize, usize)>) {
        if let Some((x, y)) = selected_coords {
            if x < self.width && y < self.height {
                if let Some(p) = self.platform_at(x, y) {
                    let rect = Rect::new(
                        p.min_x as f32 * TILE_SIZE,
                        p.min_y as f32 * TILE_SIZE,
                        (p.max_x - p.min_x + 1) as f32 * TILE_SIZE,
                        (p.max_y - p.min_y + 1) as f32 * TILE_SIZE,
                    );
                    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 0.1, RED);
                } else if let Some(s) = self.stairs_at(x, y) {
                    let rect = Rect::new(
                        s.min_x as f32 * TILE_SIZE,
                        s.min_y as f32 * TILE_SIZE,
                        (s.max_x - s.min_x + 1) as f32 * TILE_SIZE,
                        (s.max_y - s.min_y + 1) as f32 * TILE_SIZE,
                    );
                    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 0.1, RED);
                } else {
                    let rect = Rect::new(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    );
                    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 0.1, RED);
                }
            }
        }
    }

    pub fn stairs_at(&self, x: usize, y: usize) -> Option<&Stairs> {
        if x < self.width && y < self.height {
            if let Some(idx) = self.stairs_map[y][x] { self.stairs.get(idx) } else { None }
        } else { None }
    }

    pub fn stairs_at_mut(&mut self, x: usize, y: usize) -> Option<&mut Stairs> {
        if x < self.width && y < self.height {
            if let Some(idx) = self.stairs_map[y][x] { self.stairs.get_mut(idx) } else { None }
        } else { None }
    }

    pub fn assign_stairs_with_cells(&mut self, t: TileType, cells: &[(usize, usize)]) -> usize {
        if cells.is_empty() { return self.stairs.len(); }
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (usize::MAX, usize::MAX, 0usize, 0usize);
        for &(x, y) in cells {
            min_x = min_x.min(x); min_y = min_y.min(y); max_x = max_x.max(x); max_y = max_y.max(y);
        }
        let new_index = self.stairs.len();
        self.stairs.push(Stairs { tile_type: t.clone(), min_x, min_y, max_x, max_y, metadata: default_stairs_metadata_for(t) });
        for &(x, y) in cells { if x < self.width && y < self.height { self.stairs_map[y][x] = Some(new_index); } }
        new_index
    }

    pub fn modules(&self) -> &Vec<usize> { &self.modules }
    pub fn modules_mut(&mut self) -> &mut Vec<usize> { &mut self.modules }
    pub fn module_borders(&self) -> Vec<usize> {
        let mut res = vec![0usize];
        let mut acc = 0usize;
        for span in &self.modules {
            acc = acc.saturating_add(*span);
            if acc >= self.width { break; }
            res.push(acc);
        }
        res
    }

    // Recompute width = sum of module spans and resize level accordingly
    pub fn apply_modules_as_width(&mut self) {
        let new_width = self.modules.iter().copied().sum::<usize>().max(0);
        self.resize_width(new_width);
        self.enforce_module_boundaries_for_structures();
    }

    pub fn module_index_for_x(&self, x: usize) -> Option<usize> {
        if x >= self.width { return None; }
        let borders = self.module_borders();
        for i in 0..borders.len() {
            let start = borders[i];
            let end = if i + 1 < borders.len() { borders[i + 1] } else { self.width };
            if x >= start && x < end { return Some(i); }
        }
        None
    }

    pub fn module_end_for_x(&self, x: usize) -> usize {
        if x >= self.width { return self.width; }
        let borders = self.module_borders();
        for i in 0..borders.len() {
            let start = borders[i];
            let end = if i + 1 < borders.len() { borders[i + 1] } else { self.width };
            if x >= start && x < end { return end; }
        }
        self.width
    }

    fn enforce_module_boundaries_for_structures(&mut self) {
        use std::collections::BTreeSet;
        // Remove any stairs spanning multiple modules
        let mut to_remove: BTreeSet<usize> = BTreeSet::new();
        for (idx, s) in self.stairs.iter().enumerate() {
            let mi_start = self.module_index_for_x(s.min_x);
            let mi_end = self.module_index_for_x(s.max_x);
            if mi_start.is_some() && mi_end.is_some() && mi_start != mi_end { to_remove.insert(idx); }
        }
        if to_remove.is_empty() { return; }
        // Clear map cells for removed stairs
        for y in 0..self.height { for x in 0..self.width { if let Some(si) = self.stairs_map[y][x] { if to_remove.contains(&si) { self.stairs_map[y][x] = None; } } } }
        // Compact stairs vec and remap indices
        let mut new_vec: Vec<Stairs> = Vec::with_capacity(self.stairs.len() - to_remove.len());
        let mut old_to_new: Vec<Option<usize>> = vec![None; self.stairs.len()];
        for (old_idx, s) in self.stairs.iter().enumerate() {
            if to_remove.contains(&old_idx) { continue; }
            let new_idx = new_vec.len();
            new_vec.push(s.clone());
            old_to_new[old_idx] = Some(new_idx);
        }
        for y in 0..self.height { for x in 0..self.width { if let Some(old_idx) = self.stairs_map[y][x] { self.stairs_map[y][x] = old_to_new[old_idx]; } } }
        self.stairs = new_vec;
    }
} 

// ----- Export (serde) -----

#[derive(Serialize)]
struct ExportLevel {
    name: String,
    modules: Vec<ExportModule>,
}

#[derive(Serialize)]
struct ExportModule {
    #[serde(rename = "moduleID")]
    module_id: usize,
    #[serde(rename = "xSpan")]
    x_span: usize,
    #[serde(rename = "gameObjects")]
    game_objects: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct Position { x: usize, y: usize }

#[derive(Serialize)]
struct Size { x: usize, y: usize }

impl Level {
    pub fn export_to_json(&self, name: String, registry: &TileRegistry) -> serde_json::Result<String> {
        let borders = self.module_borders();
        let mut modules: Vec<ExportModule> = Vec::new();
        let mut start_x = 0usize;
        for (i, span) in self.modules.iter().copied().enumerate() {
            let end_x = start_x + span;
            let mut game_objects: Vec<serde_json::Value> = Vec::new();

            // Platforms fully contained within module
            for p in &self.platforms {
                if p.min_x >= start_x && p.max_x < end_x {
                    // Type should be the platform's tile type display name (e.g., "Wall", "Ground")
                    let type_name = display_name_for_tile_type(registry, &p.tile_type).unwrap_or_else(|| "Platform".to_string());
                    let object_id = get_meta_text(&p.metadata, "object_id").unwrap_or_default();
                    game_objects.push(json!({
                        "type": type_name,
                        "position": { "x": p.min_x, "y": p.min_y },
                        "size": { "x": p.max_x - p.min_x + 1, "y": p.max_y - p.min_y + 1 },
                        "enabled": true,
                        "mutable": get_meta_bool(&p.metadata, "mutable", false),
                        "object_id": object_id,
                    }));
                }
            }

            // Stairs fully contained within module
            for s in &self.stairs {
                if s.min_x >= start_x && s.max_x < end_x {
                    let size = (s.max_x - s.min_x + 1).max(s.max_y - s.min_y + 1);
                    let object_id = get_meta_text(&s.metadata, "object_id").unwrap_or_default();
                    game_objects.push(json!({
                        "type": "stairs",
                        "position": { "x": s.min_x, "y": s.min_y },
                        "size": size,
                        "enabled": true,
                        "mutable": get_meta_bool(&s.metadata, "mutable", false),
                        "object_id": object_id,
                    }));
                }
            }

            // Individual tiles not in any structure, within module
            for y in 0..self.height {
                for x in start_x..end_x.min(self.width) {
                    if self.platform_at(x, y).is_none() && self.stairs_at(x, y).is_none() {
                        let t = &self.tiles[y][x];
                        if let TileType::Custom(k) = &t.tile_type {
                            let kind = display_name_for_tile_type(registry, &t.tile_type).unwrap_or_else(|| k.clone());
                            let object_id = get_meta_text(&t.metadata, "object_id");
                            let powerup = get_meta_text(&t.metadata, "powerup");
                            let speed = get_meta_number(&t.metadata, "speed");
                            game_objects.push(json!({
                                "type": kind,
                                "position": { "x": x, "y": y },
                                "enabled": get_meta_bool(&t.metadata, "enabled", true),
                                "mutable": get_meta_bool(&t.metadata, "mutable", false),
                                "object_id": object_id.unwrap_or_default(),
                                "powerup": powerup,
                                "speed": speed,
                            }));
                        }
                    }
                }
            }

            modules.push(ExportModule { module_id: i, x_span: span, game_objects });
            start_x = end_x;
        }

        let export = ExportLevel { name, modules };
        serde_json::to_string_pretty(&export)
    }
}

fn get_meta_text(fields: &[MetaField], key: &str) -> Option<String> {
    for f in fields {
        if let MetaField::Text { key: k, value, .. } = f { if *k == key { return Some(value.clone()); } }
    }
    None
}

fn get_meta_bool(fields: &[MetaField], key: &str, default_value: bool) -> bool {
    for f in fields {
        if let MetaField::Bool { key: k, value, .. } = f { if *k == key { return *value; } }
    }
    default_value
}

fn get_meta_number(fields: &[MetaField], key: &str) -> Option<f32> {
    for f in fields {
        if let MetaField::Number { key: k, value, .. } = f { if *k == key { return Some(*value); } }
    }
    None
}

fn display_name_for_tile_type(registry: &TileRegistry, t: &TileType) -> Option<String> {
    match t {
        TileType::Air => Some("Air".to_string()),
        TileType::Custom(k) => registry.get(k).map(|tk| tk.display_name.clone()),
    }
}
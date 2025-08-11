pub mod history;
use history::*;
use crate::tile::*;

use egui_macroquad::macroquad::prelude::*;
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

} 
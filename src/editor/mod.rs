mod brushes;

pub use brushes::BrushType;
pub use brushes::structure::StructureMode;

use crate::camera::Camera;
use crate::level::Level;
use crate::tile::{TileType, Tile, TileRegistry};
use brushes::BrushManager;
use egui_macroquad::macroquad::prelude::*;

// Constants
const ZOOM_FACTOR: f32 = 1.1;
const ZOOM_MIN: f32 = 0.05;
const ZOOM_MAX: f32 = 10.0;
const PAN_SENSITIVITY: f32 = 0.001;
const PAN_MARGIN: f32 = 5.0; // Extra margin around the level for panning
const CAMERA_VIEWPORT_SIZE: f32 = 2.0; // Camera viewport size in world units

pub struct LevelEditor {
    level: Level,
    camera: Camera,
    brush_manager: BrushManager,
    show_tile_selector: bool,
    registry: TileRegistry,
    show_modules: bool,
}

impl LevelEditor {
    pub async fn new(level_width: usize, level_height: usize) -> Self {
        let mut level = Level::new(level_width, level_height);
        // Initialize with 2 modules of size 15 (width will become 30)
        if level.modules().is_empty() {
            level.modules_mut().clear();
            level.modules_mut().push(15);
            level.modules_mut().push(15);
            level.apply_modules_as_width();
        }

        let camera = Camera::new(level.width() as f32, level.height() as f32);
        let registry = TileRegistry::load_from_dir("assets/textures").await;

        Self {
            level,
            camera,
            brush_manager: BrushManager::new(),
            show_tile_selector: true,
            registry,
            show_modules: false,
        }
    }

    pub fn handle_input(&mut self) {
        let current_mouse_pos = mouse_position();
        let current_mouse_vec = vec2(current_mouse_pos.0, current_mouse_pos.1);
        
        // Handle keyboard shortcuts
        if is_key_down(KeyCode::LeftControl) {
            if is_key_pressed(KeyCode::Z) {
                self.undo();
            }
            
            if is_key_pressed(KeyCode::Y) {
                self.redo();
            }
        }
        
        
        // If UI is capturing the pointer, skip all world mouse interactions this frame


        // Handle brush inputs which should work anywhere on screen
        if is_mouse_button_pressed(MouseButton::Right) {
            self.brush_manager.handle_mouse_cancel(&mut self.level);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.brush_manager.handle_mouse_release(&mut self.level);
        }
        
        // Handle mouse input for brush operations
        let world_pos = self.camera.screen_to_world(current_mouse_vec);
        let tile_x = world_pos.x.floor() as i32;
        let tile_y = world_pos.y.floor() as i32;
        
        // Check if coordinates are within bounds
        if tile_x >= 0 && tile_x < self.level.width() as i32 && 
           tile_y >= 0 && tile_y < self.level.height() as i32 {
            let x = tile_x as usize;
            let y = tile_y as usize;
            
            // Handle brush input while inside level
            if is_mouse_button_pressed(MouseButton::Left) {
                self.brush_manager.handle_mouse_press(&mut self.level, x, y);
            } else if is_mouse_button_down(MouseButton::Left) {
                self.brush_manager.handle_mouse_drag(&mut self.level, x, y);
            }
        }

        // Update highlights for current brush
        let mouse_tile_x = if tile_x >= 0 && tile_x < self.level.width() as i32 { Some(tile_x as usize) } else { None };
        let mouse_tile_y = if tile_y >= 0 && tile_y < self.level.height() as i32 { Some(tile_y as usize) } else { None };
        self.brush_manager.update_highlights(&mut self.level, mouse_tile_x, mouse_tile_y);

        // Handle camera zoom with mouse wheel
        self.handle_zoom();

        // Handle camera pan with right mouse button
        self.handle_pan(current_mouse_vec);

        
        self.camera.set_last_mouse_pos(current_mouse_vec);
    }

    fn handle_zoom(&mut self) {
        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            let zoom_factor = if wheel.1 > 0.0 { ZOOM_FACTOR } else { 1.0 / ZOOM_FACTOR };
            // Apply uniform zoom to maintain square tiles
            let new_zoom = self.camera.zoom().x * zoom_factor;
            let clamped_zoom = new_zoom.clamp(ZOOM_MIN, ZOOM_MAX);
            self.camera.set_zoom(vec2(clamped_zoom, clamped_zoom));
        }
    }

    fn handle_pan(&mut self, current_mouse_vec: Vec2) {
        if is_mouse_button_down(MouseButton::Right) {
            let mouse_delta = current_mouse_vec - self.camera.last_mouse_pos();
            let new_target = self.camera.target() - mouse_delta * PAN_SENSITIVITY / self.camera.zoom().x;
            
            let clamped_target = self.calculate_pan_bounds(new_target);
            self.camera.set_target(clamped_target);
        }
    }

    fn calculate_pan_bounds(&self, new_target: Vec2) -> Vec2 {
        // Calculate bounds to keep level visible
        let level_width_world = self.level.width() as f32;
        let level_height_world = self.level.height() as f32;
        
        // Calculate how much of the level is visible at current zoom (uniform zoom)
        let visible_size = CAMERA_VIEWPORT_SIZE / self.camera.zoom().x; // Same for both X and Y
        
        // Calculate bounds - camera target should be at least half the visible area from the edges
        let min_x = visible_size / 2.0 - PAN_MARGIN;
        let max_x = level_width_world - visible_size / 2.0 + PAN_MARGIN;
        let min_y = visible_size / 2.0 - PAN_MARGIN;
        let max_y = level_height_world - visible_size / 2.0 + PAN_MARGIN;
        
        // Ensure bounds are valid (min should be less than max)
        let clamped_x = if min_x < max_x {
            new_target.x.clamp(min_x, max_x)
        } else {
            level_width_world / 2.0 // Center if bounds are invalid
        };
        
        let clamped_y = if min_y < max_y {
            new_target.y.clamp(min_y, max_y)
        } else {
            level_height_world / 2.0 // Center if bounds are invalid
        };
        
        vec2(clamped_x, clamped_y)
    }

    pub fn setup_camera(&mut self) {
        self.camera.setup_camera();
    }

    pub fn draw_level(&self) {
        self.level.draw(&self.registry);
        // Draw selection indicator if a tile is selected
        if self.brush_manager.brush_type == BrushType::Selector {
            self.level.draw_selection_indicator(self.get_selected_tile_coords());
        }
    }

    pub fn clear_highlights(&mut self) {
        self.level.clear_highlights();
    }

    // Brush management methods
    pub fn set_brush_type(&mut self, brush_type: BrushType) {
        self.brush_manager.set_brush_type(brush_type);
    }

    pub fn brush_type(&self) -> BrushType {
        self.brush_manager.brush_type
    }

    pub fn selected_tile(&self) -> TileType {
        self.brush_manager.selected_tile.clone()
    }

    pub fn set_selected_tile(&mut self, tile: TileType) {
        let is_selector = matches!(self.brush_manager.brush_type, BrushType::Selector);
        self.brush_manager.set_selected_tile(tile);
        if is_selector {
            self.brush_manager.switch_to_last_drawing_brush();
        }
    }

    pub fn show_tile_selector(&self) -> bool {
        self.show_tile_selector
    }

    pub fn set_show_tile_selector(&mut self, show: bool) {
        self.show_tile_selector = show;
    }

    // Add undo/redo methods
    pub fn undo(&mut self) {
        self.level.undo();
    }

    pub fn redo(&mut self) {
        self.level.redo();
    }

    pub fn can_undo(&self) -> bool {
        self.level.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.level.can_redo()
    }

    pub fn get_selected_tile_coords(&self) -> Option<(usize, usize)> {
        self.brush_manager.get_selected_tile_coords()
    }

    pub fn get_selected_tile(&self) -> Option<&Tile> {
        if let Some((x, y)) = self.get_selected_tile_coords() {
            self.level.get_tile(x, y)
        } else {
            None
        }
    }

    pub fn level_mut(&mut self) -> &mut Level { &mut self.level }
    pub fn registry(&self) -> &TileRegistry { &self.registry }
    pub fn set_structure_mode(&mut self, mode: StructureMode) { self.brush_manager.set_structure_mode(mode); }
    pub fn structure_mode(&self) -> StructureMode { self.brush_manager.structure_mode() }

    pub fn get_selected_platform_info(&self) -> Option<(TileType, usize, usize, usize, usize)> {
        if let Some((x, y)) = self.get_selected_tile_coords() {
            if let Some(platform) = self.level.platform_at(x, y) {
                return Some((platform.tile_type.clone(), platform.min_x, platform.min_y, platform.max_x, platform.max_y));
            }
        }
        None
    }

    pub fn update_selected_tile(&mut self, tile_type: TileType) {
        if let Some((x, y)) = self.get_selected_tile_coords() {
            if let Some(tile) = self.level.get_tile_mut(x, y) {
                tile.set_tile_type(tile_type);
            }
        }
    }

    pub fn update_selected_tile_name(&mut self, name: String) {
        if let Some((x, y)) = self.get_selected_tile_coords() {
            if let Some(tile) = self.level.get_tile_mut(x, y) {
                tile.set_name(name);
            }
        }
    }

    // Modules helpers for UI
    pub fn toggle_modules_view(&mut self) { self.show_modules = !self.show_modules; }
    pub fn show_modules_view(&self) -> bool { self.show_modules }
    pub fn modules(&self) -> &Vec<usize> { self.level.modules() }
    pub fn set_module_span(&mut self, idx: usize, span: usize) { if let Some(s) = self.level.modules_mut().get_mut(idx) { *s = span; } self.level.apply_modules_as_width(); }
    pub fn add_module(&mut self, span: usize) { self.level.modules_mut().push(span.max(1)); self.level.apply_modules_as_width(); }
    pub fn remove_module(&mut self, idx: usize) { if idx < self.level.modules_mut().len() { self.level.modules_mut().remove(idx); self.level.apply_modules_as_width(); } }
    pub fn level_width(&self) -> usize { self.level.width() }

    // Export
    pub fn level_export_json(&self, name: String) -> serde_json::Result<String> { self.level.export_to_json() }
} 
use crate::tile::TileType;
use crate::level::Level;
use super::modes::{Mode, ModeTrait, DrawingMode, SelectorMode, DrawingBrushType};

// Mode manager that handles the current mode and selected tile
pub struct ModeManager {
    pub current_mode: Mode,
    pub selected_tile: TileType,
    drawing_mode: DrawingMode,
    selector_mode: SelectorMode,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            current_mode: Mode::Drawing,
            selected_tile: TileType::Air,
            drawing_mode: DrawingMode::new(),
            selector_mode: SelectorMode::new(),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.current_mode = mode;
        // Cancel any active operations when switching modes
        self.drawing_mode.on_mouse_cancel(&mut Level::new(0, 0)); // Dummy level
        self.selector_mode.clear_selection();
    }

    pub fn set_selected_tile(&mut self, tile: TileType) {
        // Auto-detect brush type when selecting a tile in drawing mode
        if self.current_mode == Mode::Drawing {
            self.drawing_mode.auto_detect_brush_type(&tile);
        }
        self.selected_tile = tile;
    }

    pub fn get_current_mode(&mut self) -> &mut dyn ModeTrait {
        match self.current_mode {
            Mode::Drawing => &mut self.drawing_mode,
            Mode::Selector => &mut self.selector_mode,
        }
    }

    pub fn handle_mouse_press(&mut self, level: &mut Level, x: usize, y: usize) {
        let selected_tile = self.selected_tile.clone();
        let mode = self.get_current_mode();
        mode.on_mouse_press(level, x, y, selected_tile);
    }

    pub fn handle_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize) {
        let selected_tile = self.selected_tile.clone();
        let mode = self.get_current_mode();
        mode.on_mouse_drag(level, x, y, selected_tile);
    }

    pub fn handle_mouse_release(&mut self, level: &mut Level) {
        let selected_tile = self.selected_tile.clone();
        let mode = self.get_current_mode();
        mode.on_mouse_release(level, selected_tile);
    }

    pub fn handle_mouse_cancel(&mut self, level: &mut Level) {
        let mode = self.get_current_mode();
        mode.on_mouse_cancel(level);
    }

    pub fn handle_right_click(&mut self, level: &mut Level, x: usize, y: usize) {
        let mode = self.get_current_mode();
        mode.on_right_click(level, x, y);
    }

    pub fn is_mode_active(&self) -> bool {
        match self.current_mode {
            Mode::Drawing => self.drawing_mode.is_active(),
            Mode::Selector => self.selector_mode.is_active(),
        }
    }

    pub fn get_selected_tile_coords(&self) -> Option<(usize, usize)> {
        if self.current_mode == Mode::Selector {
            self.selector_mode.get_selected_tile()
        } else {
            None
        }
    }

    pub fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>) {
        let mode = self.get_current_mode();
        mode.update_highlights(level, mouse_x, mouse_y);
    }

    // Drawing mode specific methods
    pub fn set_drawing_brush_type(&mut self, brush_type: DrawingBrushType) {
        if self.current_mode == Mode::Drawing {
            self.drawing_mode.set_brush_type(brush_type);
        }
    }
    
    pub fn drawing_brush_type(&self) -> DrawingBrushType {
        self.drawing_mode.brush_type()
    }

    pub fn mode(&self) -> Mode {
        self.current_mode
    }

    pub fn selected_tile(&self) -> &TileType {
        &self.selected_tile
    }
}

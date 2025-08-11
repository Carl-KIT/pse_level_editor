use super::Brush;
use crate::*;

#[derive(Clone, Debug)]
pub struct FillState {
    pub start_pos: (usize, usize),
    pub current_pos: (usize, usize),
    pub fill_tile: TileType,
}

impl FillState {
    pub fn new(start_x: usize, start_y: usize, fill_tile: TileType) -> Self {
        Self {
            start_pos: (start_x, start_y),
            current_pos: (start_x, start_y),
            fill_tile,
        }
    }

    pub fn update_position(&mut self, x: usize, y: usize) {
        self.current_pos = (x, y);
    }

    pub fn get_bounds(&self) -> (usize, usize, usize, usize) {
        let (start_x, start_y) = self.start_pos;
        let (current_x, current_y) = self.current_pos;
        
        let min_x = start_x.min(current_x);
        let max_x = start_x.max(current_x);
        let min_y = start_y.min(current_y);
        let max_y = start_y.max(current_y);
        
        (min_x, min_y, max_x, max_y)
    }
}

// Fill brush implementation
pub struct FillBrush {
    fill_state: Option<FillState>,
}

impl FillBrush {
    pub fn new() -> Self {
        Self { fill_state: None }
    }
}

impl Brush for FillBrush {
    fn name(&self) -> &'static str {
        "Fill"
    }

    fn on_mouse_press(&mut self, _level: &mut Level, x: usize, y: usize, tile: TileType) -> bool {
        self.fill_state = Some(FillState::new(x, y, tile));
        true
    }

    fn on_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize, _tile: TileType) -> bool {
        if let Some(ref mut fill_state) = self.fill_state {
            fill_state.update_position(x, y);
            let (min_x, min_y, max_x, max_y) = fill_state.get_bounds();
            
            // Create highlighted tiles for the fill area
            let mut highlighted_tiles = Vec::new();
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if x < level.width() && y < level.height() {
                        highlighted_tiles.push((x, y));
                    }
                }
            }
            level.set_highlighted_tiles(highlighted_tiles);
        }
        true
    }

    fn on_mouse_release(&mut self, level: &mut Level) -> bool {
        if let Some(fill_state) = self.fill_state.take() {
            let (min_x, min_y, max_x, max_y) = fill_state.get_bounds();
            level.fill_rectangle(min_x, min_y, max_x, max_y, fill_state.fill_tile);
            level.finish_operation();
            level.clear_highlights();
        }
        true
    }

    fn on_mouse_cancel(&mut self, level: &mut Level) -> bool {
        self.fill_state = None;
        level.clear_highlights();
        true
    }

    fn draw_preview(&self, _level: &Level) {
        // Preview is handled by the level's draw method
    }

    fn is_active(&self) -> bool {
        self.fill_state.is_some()
    }

    fn update_highlights(&mut self, _level: &mut Level, _mouse_x: Option<usize>, _mouse_y: Option<usize>) {
        // Fill brush doesn't update highlights
    }
}
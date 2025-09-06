use super::Brush;
use crate::*;

pub struct SelectorBrush {
    selected_tile: Option<(usize, usize)>, // (x, y) coordinates of selected tile
}

impl SelectorBrush {
    pub fn new() -> Self {
        Self { selected_tile: None }
    }

    pub fn get_selected_tile(&self) -> Option<(usize, usize)> {
        self.selected_tile
    }

    pub fn clear_selection(&mut self) {
        self.selected_tile = None;
    }
}

impl Brush for SelectorBrush {
    fn name(&self) -> &'static str {
        "Selector"
    }

    fn on_mouse_press(&mut self, _level: &mut Level, x: usize, y: usize, _tile: TileType) -> bool {
        self.selected_tile = Some((x, y));
        true
    }

    fn on_mouse_drag(&mut self, _level: &mut Level, _x: usize, _y: usize, _tile: TileType) -> bool {
        // Selector doesn't do anything on drag
        true
    }

    fn on_mouse_release(&mut self, _level: &mut Level, _tile: TileType) -> bool {
        // Selector doesn't do anything on release
        true
    }

    fn on_mouse_cancel(&mut self, _level: &mut Level) -> bool {
        self.clear_selection();
        true
    }

    fn draw_preview(&self, _level: &Level) {
        // Selector doesn't need preview
    }

    fn is_active(&self) -> bool {
        self.selected_tile.is_some()
    }

    fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>) {
        // Selector brush highlights selected tile if one is selected, otherwise highlights mouse position
        if let Some((selected_x, selected_y)) = self.selected_tile {
            if selected_x < level.width() && selected_y < level.height() {
                level.set_highlighted_tiles(vec![(selected_x, selected_y)]);
            } else {
                level.clear_highlights();
            }
        } else if let (Some(x), Some(y)) = (mouse_x, mouse_y) {
            if x < level.width() && y < level.height() {
                level.set_highlighted_tiles(vec![(x, y)]);
            } else {
                level.clear_highlights();
            }
        } else {
            level.clear_highlights();
        }
    }
}
use super::Brush;
use crate::*;

pub struct SingleBrush {
    active: bool,
}

impl SingleBrush {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Brush for SingleBrush {
    fn name(&self) -> &'static str {
        "Single"
    }

    fn on_mouse_press(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool {
        self.active = true;
        level.set_tile(x, y, tile);
        true
    }

    fn on_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool {
        if self.active {
            level.set_tile(x, y, tile);
        }
        true
    }

    fn on_mouse_release(&mut self, level: &mut Level) -> bool {
        if self.active {
            level.finish_operation();
            self.active = false;
        }
        true
    }

    fn on_mouse_cancel(&mut self, _level: &mut Level) -> bool {
        self.active = false;
        true
    }

    fn draw_preview(&self, _level: &Level) {
        // Single brush doesn't need preview
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>) {
        // Single brush always highlights the tile at mouse position
        if let (Some(x), Some(y)) = (mouse_x, mouse_y) {
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
use super::Brush;
use crate::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DrawingMode {
    Single,      // Single tile placement
    PlatformRect, // Rectangle platform
    Stairs,      // Stairs structure
}

impl DrawingMode {
    fn shape(&self, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        match self {
            Self::Single => {
                vec![(start.0, start.1)]
            }
            Self::PlatformRect => {
                // determine top left corner
                let start_x = start.0.min(end.0);
                let start_y = start.1.min(end.1);

                // determine bottom right corner
                let end_x = start.0.max(end.0);
                let end_y = start.1.max(end.1);

                let mut res = vec![];

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        res.push((x, y));
                    }
                }

                res
            }
            Self::Stairs => {
                let mut res = Vec::new();

                let (sx, sy) = start;
                let (ex, ey) = end;

                let dx = sx.abs_diff(ex);
                let dy = sy.abs_diff(ey);

                // Force stairs to be square by using the minimum dimension
                let steps = dx.min(dy);

                // Calculate the actual end position to ensure square shape
                let (actual_ex, actual_ey) = if dx < dy {
                    // Width is smaller, extend height to match
                    if sy < ey {
                        (ex, sy + steps)
                    } else {
                        (ex, sy - steps)
                    }
                } else {
                    // Height is smaller, extend width to match
                    if sx < ex {
                        (sx + steps, ey)
                    } else {
                        (sx - steps, ey)
                    }
                };

                // Generate stairs pattern
                for i in 0..=steps {
                    let (start_pos, end_y) = if sy < actual_ey {
                        if sx < actual_ex {
                            (
                                (sx + i, sy + i),
                                actual_ey
                            )
                        } else {
                            (
                                (sx - i, sy + i),
                                actual_ey
                            )
                        }
                    } else {
                        if sx < actual_ex {
                            (
                                (sx + i, sy - i),
                                actual_ey
                            )
                        } else {
                            (
                                (sx - i, sy - i),
                                actual_ey
                            )
                        }
                    };

                    let (x, y_start) = start_pos;
                    for y in y_start..=end_y {
                        res.push((x, y));
                    }
                }

                res
            }
        }
    }
}

pub struct DrawingBrush {
    active: bool,
    start_pos: Option<(usize, usize)>,
    end_pos: Option<(usize, usize)>,
    mode: DrawingMode,
}

impl DrawingBrush {
    pub fn new() -> Self {
        Self {
            active: false,
            start_pos: None,
            end_pos: None,
            mode: DrawingMode::Single,
        }
    }

    pub fn set_mode(&mut self, mode: DrawingMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> DrawingMode {
        self.mode
    }

    pub fn get_selected_tile(&self) -> Option<TileType> {
        // This will be set by the brush manager when a tile is selected
        None // For now, we'll pass it through the method parameters
    }

    // Determine the appropriate mode based on the selected tile
    pub fn auto_detect_mode(&mut self, tile: &TileType) {
        match tile {
            TileType::Custom(name) => {
                match name.as_str() {
                    "stairs" => self.mode = DrawingMode::Stairs,
                    "platform" => self.mode = DrawingMode::PlatformRect,
                    _ => self.mode = DrawingMode::Single,
                }
            }
            TileType::Air => self.mode = DrawingMode::Single,
        }
    }
}

impl Brush for DrawingBrush {
    fn name(&self) -> &'static str {
        "Drawing"
    }

    fn on_mouse_press(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool {
        // Auto-detect mode based on tile type
        self.auto_detect_mode(&tile);
        
        self.active = true;
        self.start_pos = Some((x, y));
        
        match self.mode {
            DrawingMode::Single => {
                level.set_tile(x, y, tile);
            }
            DrawingMode::PlatformRect | DrawingMode::Stairs => {
                // For structures, we'll place on mouse release
                // Just highlight the start position for now
                level.set_highlighted_tiles(vec![(x, y)]);
            }
        }
        true
    }

    fn on_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool {
        if !self.active {
            return false;
        }

        match self.mode {
            DrawingMode::Single => {
                level.set_tile(x, y, tile);
            }
            DrawingMode::PlatformRect | DrawingMode::Stairs => {
                self.end_pos = Some((x, y));
                if let Some(start) = self.start_pos {
                    let shape = self.mode.shape(start, (x, y));
                    level.set_highlighted_tiles(shape);
                }
            }
        }
        true
    }

    fn on_mouse_release(&mut self, level: &mut Level, tile: TileType) -> bool {
        if !self.active {
            return false;
        }

        match self.mode {
            DrawingMode::Single => {
                level.finish_operation();
            }
            DrawingMode::PlatformRect | DrawingMode::Stairs => {
                if let (Some(start), Some(end)) = (self.start_pos, self.end_pos) {
                    let shape = self.mode.shape(start, end);
                    
                    // Check if crosses module border
                    if crosses_module_border(level, &shape) {
                        self.active = false;
                        self.start_pos = None;
                        self.end_pos = None;
                        level.clear_highlights();
                        return true;
                    }
                    
                    // Use the selected tile type
                    let tile_type = tile;
                    
                    // Place tiles first
                    for (x, y) in &shape {
                        level.set_tile(*x, *y, tile_type.clone());
                    }
                    
                    // Create the appropriate structure
                    match self.mode {
                        DrawingMode::PlatformRect => {
                            // Platforms are automatically detected by rebuild_platforms
                            // No additional action needed
                        }
                        DrawingMode::Stairs => {
                            // Calculate orientation for stairs
                            let orientation = stairs_orientation(start, end);
                            level.assign_stairs_with_cells(tile_type, &shape, orientation);
                        }
                        _ => {}
                    }
                    
                    level.finish_operation();
                }
            }
        }

        self.active = false;
        self.start_pos = None;
        level.clear_highlights();
        true
    }

    fn on_mouse_cancel(&mut self, level: &mut Level) -> bool {
        self.active = false;
        self.start_pos = None;
        self.end_pos = None;
        level.clear_highlights();
        true
    }

    fn draw_preview(&self, _level: &Level) {
        // Preview is handled by highlights
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>) {
        if !self.active {
            // Update highlights based on current mode and mouse position
            if let (Some(x), Some(y)) = (mouse_x, mouse_y) {
                if x < level.width() && y < level.height() {
                    match self.mode {
                        DrawingMode::Single => {
                            level.set_highlighted_tiles(vec![(x, y)]);
                        }
                        DrawingMode::PlatformRect | DrawingMode::Stairs => {
                            // For structures, show preview of what would be created
                            if let Some(start) = self.start_pos {
                                let shape = self.mode.shape(start, (x, y));
                                level.set_highlighted_tiles(shape);
                            } else {
                                level.set_highlighted_tiles(vec![(x, y)]);
                            }
                        }
                    }
                } else {
                    level.clear_highlights();
                }
            } else {
                level.clear_highlights();
            }
        }
    }
}

fn crosses_module_border(level: &Level, cells: &[(usize, usize)]) -> bool {
    if cells.is_empty() { return false; }
    let mut module: Option<usize> = None;
    for &(x, _y) in cells {
        if let Some(mi) = level.module_index_for_x(x) {
            if let Some(m0) = module { if m0 != mi { return true; } } else { module = Some(mi); }
        }
    }
    false
}

fn stairs_orientation(start: (usize, usize), end: (usize, usize)) -> i32 {
    if start.0 < end.0 && start.1 < end.1 {
        return 1;
    }
    else if start.0 > end.0 && start.1 > end.1 {
        return 1;
    }

    return -1;
}

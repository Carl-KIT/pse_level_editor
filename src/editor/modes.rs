use crate::tile::TileType;
use crate::level::Level;
use egui_macroquad::macroquad::prelude::*;

// High-level modes that the user can select
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode {
    Drawing,  // Drawing mode - can place tiles, platforms, stairs
    Selector, // Selection mode - can select and inspect tiles
}

impl Mode {
    pub fn name(&self) -> &'static str {
        match self {
            Mode::Drawing => "Drawing",
            Mode::Selector => "Selector",
        }
    }
}

// Internal brush types used within drawing mode
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DrawingBrushType {
    Single,      // Single tile placement
    Platform,    // Rectangle platform
    Stairs,      // Stairs structure
}

impl DrawingBrushType {
    pub fn name(&self) -> &'static str {
        match self {
            DrawingBrushType::Single => "Single",
            DrawingBrushType::Platform => "Platform", 
            DrawingBrushType::Stairs => "Stairs",
        }
    }
}

// Trait for modes that can act on a level
pub trait ModeTrait {
    fn name(&self) -> &'static str;
    fn on_mouse_press(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool;
    fn on_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool;
    fn on_mouse_release(&mut self, level: &mut Level, tile: TileType) -> bool;
    fn on_mouse_cancel(&mut self, level: &mut Level) -> bool;
    fn on_right_click(&mut self, level: &mut Level, x: usize, y: usize) -> bool;
    fn draw_preview(&self, level: &Level);
    fn is_active(&self) -> bool;
    fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>);
}

// Drawing mode implementation
pub struct DrawingMode {
    active: bool,
    start_pos: Option<(usize, usize)>,
    end_pos: Option<(usize, usize)>,
    brush_type: DrawingBrushType,
}

impl DrawingMode {
    pub fn new() -> Self {
        Self {
            active: false,
            start_pos: None,
            end_pos: None,
            brush_type: DrawingBrushType::Single,
        }
    }

    pub fn set_brush_type(&mut self, brush_type: DrawingBrushType) {
        self.brush_type = brush_type;
    }

    pub fn brush_type(&self) -> DrawingBrushType {
        self.brush_type
    }

    // Auto-detect brush type based on selected tile
    pub fn auto_detect_brush_type(&mut self, tile: &TileType) {
        match tile {
            TileType::Custom(name) => {
                match name.as_str() {
                    "stairs" => self.brush_type = DrawingBrushType::Stairs,
                    "platform" => self.brush_type = DrawingBrushType::Platform,
                    _ => self.brush_type = DrawingBrushType::Single,
                }
            }
            TileType::Air => self.brush_type = DrawingBrushType::Single,
        }
    }

    fn shape(&self, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        match self.brush_type {
            DrawingBrushType::Single => {
                vec![(start.0, start.1)]
            }
            DrawingBrushType::Platform => {
                // Rectangle platform
                let start_x = start.0.min(end.0);
                let start_y = start.1.min(end.1);
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
            DrawingBrushType::Stairs => {
                // Stairs - create proper stairs pattern
                let mut res = Vec::new();
                let (sx, sy) = start;
                let (ex, ey) = end;
                let dx = sx.abs_diff(ex);
                let dy = sy.abs_diff(ey);
                let steps = dx.min(dy); // Force stairs to be square
                
                for i in 0..=steps {
                    let (start_pos, end_y) = if sy < ey {
                        if sx < ex {
                            ((sx + i, sy + i), ey)
                        } else {
                            ((sx - i, sy + i), ey)
                        }
                    } else {
                        if sx < ex {
                            ((sx + i, sy - i), sy)
                        } else {
                            ((sx - i, sy - i), sy)
                        }
                    };
                    
                    for y in start_pos.1..=end_y {
                        res.push((start_pos.0, y));
                    }
                }
                res
            }
        }
    }
}

impl ModeTrait for DrawingMode {
    fn name(&self) -> &'static str {
        "Drawing"
    }

    fn on_mouse_press(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool {
        // Auto-detect brush type based on tile
        self.auto_detect_brush_type(&tile);
        
        self.active = true;
        self.start_pos = Some((x, y));
        
        match self.brush_type {
            DrawingBrushType::Single => {
                level.set_tile(x, y, tile);
            }
            DrawingBrushType::Platform | DrawingBrushType::Stairs => {
                // For structures, we'll place on mouse release
                level.set_highlighted_tiles(vec![(x, y)]);
            }
        }
        true
    }

    fn on_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool {
        if !self.active {
            return false;
        }

        match self.brush_type {
            DrawingBrushType::Single => {
                level.set_tile(x, y, tile);
            }
            DrawingBrushType::Platform | DrawingBrushType::Stairs => {
                self.end_pos = Some((x, y));
                if let Some(start) = self.start_pos {
                    let shape = self.shape(start, (x, y));
                    level.set_highlighted_tiles(shape);
                }
            }
        }
        true
    }

    fn on_mouse_release(&mut self, level: &mut Level, _tile: TileType) -> bool {
        if !self.active {
            return false;
        }

        match self.brush_type {
            DrawingBrushType::Single => {
                level.finish_operation();
            }
            DrawingBrushType::Platform | DrawingBrushType::Stairs => {
                if let (Some(start), Some(end)) = (self.start_pos, self.end_pos) {
                    let shape = self.shape(start, end);
                    
                    // Check if crosses module border
                    if self.crosses_module_border(level, &shape) {
                        self.active = false;
                        self.start_pos = None;
                        self.end_pos = None;
                        level.clear_highlights();
                        return true;
                    }
                    
                    // For structures, use wall tile for the actual tiles
                    let wall_tile = TileType::Custom("wall".to_string());
                    for (x, y) in &shape {
                        level.set_tile(*x, *y, wall_tile.clone());
                    }
                    
                    // Create the appropriate structure
                    match self.brush_type {
                        DrawingBrushType::Platform => {
                            // Platforms are automatically detected by rebuild_platforms
                            // No additional action needed
                        }
                        DrawingBrushType::Stairs => {
                            // Calculate orientation for stairs
                            let orientation = self.stairs_orientation(start, end);
                            level.assign_stairs_with_cells(wall_tile, &shape, orientation);
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

    fn on_right_click(&mut self, level: &mut Level, x: usize, y: usize) -> bool {
        // Only allow removal if not currently dragging
        if self.active {
            return false;
        }

        // Remove tile at the clicked position
        level.set_tile(x, y, TileType::Air);
        level.finish_operation();
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
            if let (Some(x), Some(y)) = (mouse_x, mouse_y) {
                if x < level.width() && y < level.height() {
                    match self.brush_type {
                        DrawingBrushType::Single => {
                            level.set_highlighted_tiles(vec![(x, y)]);
                        }
                        DrawingBrushType::Platform | DrawingBrushType::Stairs => {
                            if let Some(start) = self.start_pos {
                                let shape = self.shape(start, (x, y));
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

impl DrawingMode {
    fn crosses_module_border(&self, level: &Level, cells: &[(usize, usize)]) -> bool {
        if cells.is_empty() { return false; }
        let mut module: Option<usize> = None;
        for &(x, _y) in cells {
            if let Some(mi) = level.module_index_for_x(x) {
                if let Some(m0) = module { if m0 != mi { return true; } } else { module = Some(mi); }
            }
        }
        false
    }

    fn stairs_orientation(&self, start: (usize, usize), end: (usize, usize)) -> i32 {
        if start.0 < end.0 && start.1 < end.1 {
            return -1; // Inverted: was 1
        }
        else if start.0 > end.0 && start.1 > end.1 {
            return -1; // Inverted: was 1
        }
        return 1; // Inverted: was -1
    }
}

// Selector mode implementation
pub struct SelectorMode {
    selected_tile: Option<(usize, usize)>,
}

impl SelectorMode {
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

impl ModeTrait for SelectorMode {
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

    fn on_right_click(&mut self, level: &mut Level, x: usize, y: usize) -> bool {
        // Remove tile at the clicked position
        level.set_tile(x, y, TileType::Air);
        level.finish_operation();
        true
    }

    fn draw_preview(&self, _level: &Level) {
        // Selector doesn't need preview
    }

    fn is_active(&self) -> bool {
        self.selected_tile.is_some()
    }

    fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>) {
        // Selector highlights selected tile if one is selected, otherwise highlights mouse position
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

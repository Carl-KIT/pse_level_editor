use crate::tile::TileType;
use crate::level::Level;
use egui_macroquad::macroquad::prelude::*;

// Different brush types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BrushType {
    Single,  // Single tile placement
    Fill,    // Rectangle fill
    Selector, // Tile selection and inspection
}

impl BrushType {
    pub fn name(&self) -> &'static str {
        match self {
            BrushType::Single => "Single",
            BrushType::Fill => "Fill",
            BrushType::Selector => "Selector",
        }
    }
}

// Trait for brushes that can act on a level
pub trait Brush {
    fn name(&self) -> &'static str;
    fn on_mouse_press(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool;
    fn on_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize, tile: TileType) -> bool;
    fn on_mouse_release(&mut self, level: &mut Level) -> bool;
    fn on_mouse_cancel(&mut self, level: &mut Level) -> bool;
    fn draw_preview(&self, level: &Level);
    fn is_active(&self) -> bool;
    fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>);
}

// Single tile brush implementation
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

// Selector brush implementation
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

    fn on_mouse_release(&mut self, _level: &mut Level) -> bool {
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

// Fill brush state
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

// Brush manager that holds the current brush
pub struct BrushManager {
    pub brush_type: BrushType,
    pub selected_tile: TileType,
    single_brush: SingleBrush,
    fill_brush: FillBrush,
    selector_brush: SelectorBrush,
}

impl BrushManager {
    pub fn new() -> Self {
        Self {
            brush_type: BrushType::Single,
            selected_tile: TileType::Grass,
            single_brush: SingleBrush::new(),
            fill_brush: FillBrush::new(),
            selector_brush: SelectorBrush::new(),
        }
    }

    pub fn set_brush_type(&mut self, brush_type: BrushType) {
        self.brush_type = brush_type;
        // Cancel any active operations when switching brushes
        self.single_brush.on_mouse_cancel(&mut Level::new(0, 0)); // Dummy level
        self.fill_brush.on_mouse_cancel(&mut Level::new(0, 0)); // Dummy level
        self.selector_brush.clear_selection(); // Clear selector selection
    }

    pub fn set_selected_tile(&mut self, tile: TileType) {
        self.selected_tile = tile;
    }

    pub fn get_current_brush(&mut self) -> &mut dyn Brush {
        match self.brush_type {
            BrushType::Single => &mut self.single_brush,
            BrushType::Fill => &mut self.fill_brush,
            BrushType::Selector => &mut self.selector_brush,
        }
    }

    pub fn handle_mouse_press(&mut self, level: &mut Level, x: usize, y: usize) {
        let selected_tile = self.selected_tile;
        let brush = self.get_current_brush();
        brush.on_mouse_press(level, x, y, selected_tile);
    }

    pub fn handle_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize) {
        let selected_tile = self.selected_tile;
        let brush = self.get_current_brush();
        brush.on_mouse_drag(level, x, y, selected_tile);
    }

    pub fn handle_mouse_release(&mut self, level: &mut Level) {
        let brush = self.get_current_brush();
        brush.on_mouse_release(level);
    }

    pub fn handle_mouse_cancel(&mut self, level: &mut Level) {
        let brush = self.get_current_brush();
        brush.on_mouse_cancel(level);
    }

    pub fn is_brush_active(&self) -> bool {
        match self.brush_type {
            BrushType::Single => self.single_brush.is_active(),
            BrushType::Fill => self.fill_brush.is_active(),
            BrushType::Selector => self.selector_brush.is_active(),
        }
    }

    pub fn get_selected_tile_coords(&self) -> Option<(usize, usize)> {
        if self.brush_type == BrushType::Selector {
            self.selector_brush.get_selected_tile()
        } else {
            None
        }
    }

    pub fn update_highlights(&mut self, level: &mut Level, mouse_x: Option<usize>, mouse_y: Option<usize>) {
        let brush = self.get_current_brush();
        brush.update_highlights(level, mouse_x, mouse_y);
    }
} 
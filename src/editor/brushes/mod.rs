use crate::tile::TileType;
use crate::level::Level;
use egui_macroquad::macroquad::prelude::*;

mod single;
mod selector;
mod fill;
pub mod structure;

use single::SingleBrush;
use selector::SelectorBrush;
use fill::FillBrush;
use structure::{StructureBrush, StructureMode};

// Different brush types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BrushType {
    Single,  // Single tile placement
    Fill,    // Rectangle fill
    Selector, // Tile selection and inspection
    Structure, // Manual multi-tile structures
}

impl BrushType {
    pub fn name(&self) -> &'static str {
        match self {
            BrushType::Single => "Single",
            BrushType::Fill => "Fill",
            BrushType::Selector => "Selector",
            BrushType::Structure => "Structure",
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


// Brush manager that holds the current brush
pub struct BrushManager {
    pub brush_type: BrushType,
    pub selected_tile: TileType,
    single_brush: SingleBrush,
    fill_brush: FillBrush,
    selector_brush: SelectorBrush,
    last_drawing_brush: BrushType,
    structure_brush: StructureBrush,
}

impl BrushManager {
    pub fn new() -> Self {
        Self {
            brush_type: BrushType::Single,
            selected_tile: TileType::Air,
            single_brush: SingleBrush::new(),
            fill_brush: FillBrush::new(),
            selector_brush: SelectorBrush::new(),
            last_drawing_brush: BrushType::Single,
            structure_brush: StructureBrush::new(),
        }
    }

    pub fn set_brush_type(&mut self, brush_type: BrushType) {
        // Track last drawing brush (Single/Fill)
        if matches!(brush_type, BrushType::Single | BrushType::Fill) {
            self.last_drawing_brush = brush_type;
        }
        self.brush_type = brush_type;
        if matches!(brush_type, BrushType::Structure) {
            // Ensure structures place Wall tiles by default
            self.selected_tile = TileType::Custom("wall".to_string());
        }
        // Cancel any active operations when switching brushes
        self.single_brush.on_mouse_cancel(&mut Level::new(0, 0)); // Dummy level
        self.fill_brush.on_mouse_cancel(&mut Level::new(0, 0)); // Dummy level
        self.selector_brush.clear_selection(); // Clear selector selection
        self.structure_brush.on_mouse_cancel(&mut Level::new(0, 0));
    }

    pub fn set_selected_tile(&mut self, tile: TileType) {
        self.selected_tile = tile;
    }

    pub fn get_current_brush(&mut self) -> &mut dyn Brush {
        match self.brush_type {
            BrushType::Single => &mut self.single_brush,
            BrushType::Fill => &mut self.fill_brush,
            BrushType::Selector => &mut self.selector_brush,
            BrushType::Structure => &mut self.structure_brush,
        }
    }

    pub fn handle_mouse_press(&mut self, level: &mut Level, x: usize, y: usize) {
        let selected_tile = self.selected_tile.clone();
        let brush = self.get_current_brush();
        brush.on_mouse_press(level, x, y, selected_tile);
    }

    pub fn handle_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize) {
        let selected_tile = self.selected_tile.clone();
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
            BrushType::Structure => self.structure_brush.is_active(),
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

    pub fn switch_to_last_drawing_brush(&mut self) {
        let target = self.last_drawing_brush;
        self.set_brush_type(target);
    }

    pub fn set_structure_mode(&mut self, mode: StructureMode) { self.structure_brush.set_mode(mode); }
    pub fn structure_mode(&self) -> StructureMode { self.structure_brush.mode() }
} 
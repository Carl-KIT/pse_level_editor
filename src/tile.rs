use egui_macroquad::macroquad::prelude::*;

// Tile types with their colors
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Air,
    Grass,
    Ground,
}

impl TileType {
    pub fn color(&self) -> Color {
        match self {
            TileType::Air => WHITE,
            TileType::Grass => GREEN,
            TileType::Ground => BROWN,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            TileType::Air => "Air",
            TileType::Grass => "Grass",
            TileType::Ground => "Ground",
        }
    }
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Air
    }
}

// Complete tile with editable attributes
#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub name: String,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            name: tile_type.name().to_string(),
        }
    }

    pub fn color(&self) -> Color {
        self.tile_type.color()
    }

    pub fn set_tile_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
        // Update name to match tile type if it was the default
        if self.name == self.tile_type.name() {
            self.name = tile_type.name().to_string();
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new(TileType::Air)
    }
} 
# New Tile Type System

This document explains the new expandable tile type system that replaces the hardcoded approach used previously.

## Overview

The new system is based on a trait-based architecture that allows for easy addition of new tile types with custom metadata and behavior. The system is organized into categories and provides a unified interface for all tile types.

## Key Components

### 1. TileType Trait

All tile types must implement the `TileType` trait, which defines the common interface:

```rust
pub trait TileType: Clone + std::fmt::Debug + Send + Sync {
    fn id(&self) -> &str;
    fn display_name(&self) -> &str;
    fn category(&self) -> TileCategory;
    fn texture(&self) -> Option<&Texture2D>;
    fn color(&self) -> Color;
    fn default_metadata(&self) -> Vec<MetaField>;
    fn position(&self) -> Option<(u32, u32)>;
    fn set_position(&mut self, pos: (u32, u32)>;
    fn size(&self) -> Option<(u32, u32)>;
    fn can_use_brush(&self, brush_type: BrushType) -> bool;
    fn editable_metadata(&self) -> Vec<MetaField>;
    fn render_metadata_ui(&mut self, ui: &mut egui::Ui);
}
```

### 2. Tile Categories

Tiles are organized into categories:

- **Tiles**: Basic terrain tiles (ground, grass, wall, etc.)
- **Structures**: Multi-tile structures (platforms, stairs)
- **Enemies**: Hostile entities (birds, pigs, snails, etc.)
- **Collectables**: Items that can be collected (powerups, flags, etc.)

### 3. Metadata System

The metadata system uses a flexible `MetaField` enum that supports:

- **Text**: String values with optional editing
- **Number**: Numeric values with min/max constraints
- **Bool**: Boolean values with optional editing
- **Label**: Read-only display values
- **Choice**: Dropdown selections

### 4. Registry System

The `TileTypeRegistry` manages all tile types and provides:

- Registration of new tile types
- Lookup by ID or category
- Filtering by brush compatibility

## Adding New Tile Types

### Example 1: Simple Enemy

```rust
#[derive(Clone, Debug)]
pub struct GoblinTile {
    id: String,
    display_name: String,
    category: TileCategory,
    texture: Option<Texture2D>,
    color: Color,
    metadata: Vec<MetaField>,
    position: Option<(u32, u32)>,
    attack_power: f32,
}

impl GoblinTile {
    pub fn new(id: String, display_name: String, texture: Option<Texture2D>, color: Color) -> Self {
        let mut metadata = create_common_metadata();
        
        // Set the type field
        if let Some(MetaField::Label { value, .. }) = metadata.iter_mut().find(|f| f.key() == "type") {
            *value = "Goblin".to_string();
        }
        
        // Add goblin-specific metadata
        metadata.push(MetaField::Number {
            key: "attackPower".to_string(),
            label: "Attack Power".to_string(),
            value: 25.0,
            min: 1.0,
            max: 100.0,
            editable: true,
        });
        
        Self {
            id,
            display_name,
            category: TileCategory::Enemies,
            texture,
            color,
            metadata,
            position: None,
            attack_power: 25.0,
        }
    }
}

impl TileType for GoblinTile {
    fn id(&self) -> &str { &self.id }
    fn display_name(&self) -> &str { &self.display_name }
    fn category(&self) -> TileCategory { self.category }
    fn texture(&self) -> Option<&Texture2D> { self.texture.as_ref() }
    fn color(&self) -> Color { self.color }
    fn default_metadata(&self) -> Vec<MetaField> { self.metadata.clone() }
    fn position(&self) -> Option<(u32, u32)> { self.position }
    fn set_position(&mut self, pos: (u32, u32)) { self.position = Some(pos); }
    fn size(&self) -> Option<(u32, u32)> { None } // Single tile
    fn can_use_brush(&self, brush_type: BrushType) -> bool {
        matches!(brush_type, BrushType::Single | BrushType::Selector)
    }
    fn editable_metadata(&self) -> Vec<MetaField> {
        self.metadata.iter().filter(|f| f.is_editable()).cloned().collect()
    }
    fn render_metadata_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Goblin Metadata:");
        ui.label(format!("Attack Power: {:.1}", self.attack_power));
        
        for field in &mut self.metadata {
            field.ui(ui);
        }
    }
}
```

### Example 2: Complex Structure

```rust
#[derive(Clone, Debug)]
pub struct CastleTile {
    id: String,
    display_name: String,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
    metadata: Vec<MetaField>,
    texture: Option<Texture2D>,
    color: Color,
    tower_count: u32,
    wall_height: f32,
}

impl CastleTile {
    pub fn new(id: String, display_name: String, min_x: usize, min_y: usize, max_x: usize, max_y: usize, texture: Option<Texture2D>, color: Color) -> Self {
        let mut metadata = create_common_metadata();
        
        // Set the type field
        if let Some(MetaField::Label { value, .. }) = metadata.iter_mut().find(|f| f.key() == "type") {
            *value = "Castle".to_string();
        }
        
        // Add castle-specific metadata
        metadata.push(MetaField::Number {
            key: "towerCount".to_string(),
            label: "Tower Count".to_string(),
            value: 4.0,
            min: 0.0,
            max: 20.0,
            editable: true,
        });
        
        metadata.push(MetaField::Number {
            key: "wallHeight".to_string(),
            label: "Wall Height".to_string(),
            value: 10.0,
            min: 1.0,
            max: 50.0,
            editable: true,
        });
        
        metadata.push(MetaField::Choice {
            key: "castleType".to_string(),
            label: "Castle Type".to_string(),
            options: vec!["Fortress".to_string(), "Palace".to_string(), "Keep".to_string()],
            selected: 0,
            editable: true,
        });
        
        Self {
            id,
            display_name,
            min_x,
            min_y,
            max_x,
            max_y,
            metadata,
            texture,
            color,
            tower_count: 4,
            wall_height: 10.0,
        }
    }
}

impl TileType for CastleTile {
    fn id(&self) -> &str { &self.id }
    fn display_name(&self) -> &str { &self.display_name }
    fn category(&self) -> TileCategory { TileCategory::Structures }
    fn texture(&self) -> Option<&Texture2D> { self.texture.as_ref() }
    fn color(&self) -> Color { self.color }
    fn default_metadata(&self) -> Vec<MetaField> { self.metadata.clone() }
    fn position(&self) -> Option<(u32, u32)> { Some((self.min_x as u32, self.min_y as u32)) }
    fn set_position(&mut self, pos: (u32, u32)) { 
        let width = self.max_x - self.min_x + 1;
        let height = self.max_y - self.min_y + 1;
        self.min_x = pos.0 as usize;
        self.min_y = pos.1 as usize;
        self.max_x = self.min_x + width - 1;
        self.max_y = self.min_y + height - 1;
    }
    fn size(&self) -> Option<(u32, u32)> { 
        Some(((self.max_x - self.min_x + 1) as u32, (self.max_y - self.min_y + 1) as u32))
    }
    fn can_use_brush(&self, brush_type: BrushType) -> bool {
        matches!(brush_type, BrushType::Structure | BrushType::Selector)
    }
    fn editable_metadata(&self) -> Vec<MetaField> {
        self.metadata.iter().filter(|f| f.is_editable()).cloned().collect()
    }
    fn render_metadata_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Castle Metadata:");
        ui.label(format!("Position: ({}, {})", self.min_x, self.min_y));
        ui.label(format!("Size: {} x {}", self.max_x - self.min_x + 1, self.max_y - self.min_y + 1));
        ui.label(format!("Tower Count: {}", self.tower_count));
        ui.label(format!("Wall Height: {:.1}", self.wall_height));
        
        for field in &mut self.metadata {
            field.ui(ui);
        }
    }
}
```

## Registering New Tile Types

To register new tile types, add them to the registry:

```rust
pub fn add_custom_tile_types(registry: &mut TileTypeRegistry) {
    registry.register(GoblinTile::new(
        "goblin".to_string(),
        "Goblin".to_string(),
        None, // No texture for now
        GREEN,
    ));
    
    registry.register(CastleTile::new(
        "castle".to_string(),
        "Castle".to_string(),
        0, 0, 5, 5, // 5x5 castle
        None,
        GRAY,
    ));
}
```

## Benefits of the New System

1. **Extensibility**: Easy to add new tile types without modifying core code
2. **Type Safety**: Compile-time checking of tile type implementations
3. **Consistency**: All tile types follow the same interface
4. **Flexibility**: Custom metadata and behavior per tile type
5. **Maintainability**: Clear separation of concerns
6. **UI Integration**: Automatic UI generation based on metadata

## Migration from Old System

The old system is still available for backward compatibility. To migrate:

1. Create new tile type implementations using the `TileType` trait
2. Register them in the `TileTypeRegistry`
3. Update UI components to use the new registry
4. Gradually replace old tile types with new ones

## Common Metadata Fields

All tile types should include these common fields:

- `objectID`: Unique identifier for the object
- `type`: Display name of the tile type
- `enabled`: Whether the tile is active
- `mutable`: Whether the tile can be modified

Additional fields can be added as needed for specific tile types.

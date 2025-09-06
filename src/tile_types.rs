use crate::tile_type_system::*;
use egui_macroquad::macroquad::prelude::*;

// Factory functions for creating specific tile types
pub async fn create_tile_types() -> TileTypeRegistry {
    let mut registry = TileTypeRegistry::new();
    
    // Load textures
    let air_texture = load_texture("assets/textures/ground.png").await.unwrap_or_else(|_| {
        // Fallback: create a white texture if loading fails
        let image = Image::gen_image_color(32, 32, WHITE);
        Texture2D::from_image(&image)
    });
    
    let ground_texture = load_texture("assets/textures/ground.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, BROWN);
        Texture2D::from_image(&image)
    });
    
    let grass_texture = load_texture("assets/textures/grass.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, GREEN);
        Texture2D::from_image(&image)
    });
    
    let wall_texture = load_texture("assets/textures/wall.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, GRAY);
        Texture2D::from_image(&image)
    });
    
    let ice_texture = load_texture("assets/textures/ice.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, Color::new(0.5, 0.8, 1.0, 1.0));
        Texture2D::from_image(&image)
    });
    
    let mud_texture = load_texture("assets/textures/mud.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, DARKBROWN);
        Texture2D::from_image(&image)
    });

    // Basic tiles
    registry.register(TileType::BasicTile {
        id: "air".to_string(),
        display_name: "Air".to_string(),
        category: TileCategory::Tiles,
        texture: air_texture,
        metadata: create_common_metadata_with_type("Air".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "ground".to_string(),
        display_name: "Ground".to_string(),
        category: TileCategory::Tiles,
        texture: ground_texture,
        metadata: create_common_metadata_with_type("Ground".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "grass".to_string(),
        display_name: "Grass".to_string(),
        category: TileCategory::Tiles,
        texture: grass_texture,
        metadata: create_common_metadata_with_type("Grass".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "wall".to_string(),
        display_name: "Wall".to_string(),
        category: TileCategory::Tiles,
        texture: wall_texture.clone(),
        metadata: create_common_metadata_with_type("Wall".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "ice".to_string(),
        display_name: "Ice".to_string(),
        category: TileCategory::Tiles,
        texture: ice_texture,
        metadata: create_common_metadata_with_type("Ice".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "mud".to_string(),
        display_name: "Mud".to_string(),
        category: TileCategory::Tiles,
        texture: mud_texture,
        metadata: create_common_metadata_with_type("Mud".to_string()),
        position: None,
    });
    
    // Load more textures
    let powerup_texture = load_texture("assets/textures/powerup_tile.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, YELLOW);
        Texture2D::from_image(&image)
    });
    
    let bird_texture = load_texture("assets/textures/bird.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, BLUE);
        Texture2D::from_image(&image)
    });
    
    let pig_texture = load_texture("assets/textures/pig.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, PINK);
        Texture2D::from_image(&image)
    });
    
    let beartrap_texture = load_texture("assets/textures/beartrap.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, BROWN);
        Texture2D::from_image(&image)
    });
    
    let flagpole_texture = load_texture("assets/textures/flagpole.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, RED);
        Texture2D::from_image(&image)
    });
    
    let grain_texture = load_texture("assets/textures/grain.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, YELLOW);
        Texture2D::from_image(&image)
    });
    
    let grow_powerup_texture = load_texture("assets/textures/grow_powerup.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, GREEN);
        Texture2D::from_image(&image)
    });
    
    let oneup_texture = load_texture("assets/textures/oneup.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, GREEN);
        Texture2D::from_image(&image)
    });
    
    let redbull_texture = load_texture("assets/textures/redbull.png").await.unwrap_or_else(|_| {
        let image = Image::gen_image_color(32, 32, RED);
        Texture2D::from_image(&image)
    });

    // Powerup tile with special metadata
    let mut powerup_metadata = create_common_metadata_with_type("Powerup Tile".to_string());
    powerup_metadata.push(MetaField::Text {
        key: "collectableClass".to_string(),
        label: "Collectable Class".to_string(),
        value: String::new(),
        editable: true,
    });
    powerup_metadata.push(MetaField::Text {
        key: "item".to_string(),
        label: "Item".to_string(),
        value: String::new(),
        editable: true,
    });
    
    registry.register(TileType::BasicTile {
        id: "powerup_tile".to_string(),
        display_name: "Powerup Tile".to_string(),
        category: TileCategory::Tiles,
        texture: powerup_texture,
        metadata: powerup_metadata,
        position: None,
    });
    
    // Enemies
    let bird_metadata = create_common_metadata_with_type("Bird".to_string());

    registry.register(TileType::BasicTile {
        id: "bird".to_string(),
        display_name: "Bird".to_string(),
        category: TileCategory::Enemies,
        texture: bird_texture,
        metadata: bird_metadata,
        position: None,
    });
    
    let pig_metadata = create_common_metadata_with_type("Pig".to_string());

    
    registry.register(TileType::BasicTile {
        id: "pig".to_string(),
        display_name: "Pig".to_string(),
        category: TileCategory::Enemies,
        texture: pig_texture,
        metadata: pig_metadata,
        position: None,
    });
    
    // Snail doesn't have a specific texture, use a fallback
    let snail_texture = {
        let image = Image::gen_image_color(32, 32, GREEN);
        Texture2D::from_image(&image)
    };
    
    registry.register(TileType::BasicTile {
        id: "snail".to_string(),
        display_name: "Snail".to_string(),
        category: TileCategory::Enemies,
        texture: snail_texture,
        metadata: create_common_metadata_with_type("Snail".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "beartrap".to_string(),
        display_name: "Bear Trap".to_string(),
        category: TileCategory::Enemies,
        texture: beartrap_texture,
        metadata: create_common_metadata_with_type("Bear Trap".to_string()),
        position: None,
    });
    
    // Collectables
    registry.register(TileType::BasicTile {
        id: "flagpole".to_string(),
        display_name: "Flag Pole".to_string(),
        category: TileCategory::Collectables,
        texture: flagpole_texture,
        metadata: create_common_metadata_with_type("Flag Pole".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "grain".to_string(),
        display_name: "Grain".to_string(),
        category: TileCategory::Collectables,
        texture: grain_texture,
        metadata: create_common_metadata_with_type("Grain".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "grow_powerup".to_string(),
        display_name: "Grow Powerup".to_string(),
        category: TileCategory::Collectables,
        texture: grow_powerup_texture,
        metadata: create_common_metadata_with_type("Grow Powerup".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "oneup".to_string(),
        display_name: "One Up".to_string(),
        category: TileCategory::Collectables,
        texture: oneup_texture,
        metadata: create_common_metadata_with_type("One Up".to_string()),
        position: None,
    });
    
    registry.register(TileType::BasicTile {
        id: "redbull".to_string(),
        display_name: "Red Bull".to_string(),
        category: TileCategory::Collectables,
        texture: redbull_texture,
        metadata: create_common_metadata_with_type("Red Bull".to_string()),
        position: None,
    });
    
    // Structures - use wall texture for both platform and stairs
    let mut platform_metadata = create_common_metadata_with_type("Platform".to_string());
    platform_metadata.push(MetaField::Label {
        label: "Size".to_string(),
        value: "0 x 0".to_string(),
    });
    
    registry.register(TileType::PlatformTile {
        id: "platform".to_string(),
        display_name: "Platform".to_string(),
        min_x: 0,
        min_y: 0,
        max_x: 0,
        max_y: 0,
        metadata: platform_metadata,
        texture: wall_texture.clone(),
    });
    
    let mut stairs_metadata = create_common_metadata_with_type("Stairs".to_string());
    stairs_metadata.push(MetaField::Label {
        label: "Size".to_string(),
        value: "0".to_string(),
    });
    stairs_metadata.push(MetaField::Choice {
        key: "orientation".to_string(),
        label: "Orientation".to_string(),
        options: vec!["-1".to_string(), "1".to_string()],
        selected: 0,
        editable: true,
    });
    
    registry.register(TileType::StairsTile {
        id: "stairs".to_string(),
        display_name: "Stairs".to_string(),
        min_x: 0,
        min_y: 0,
        max_x: 0,
        max_y: 0,
        orientation: 1,
        metadata: stairs_metadata,
        texture: wall_texture.clone(),
    });
    
    registry
}

fn create_common_metadata_with_type(type_name: String) -> Vec<MetaField> {
    let mut metadata = create_common_metadata();
    // Set the type field
    if let Some(MetaField::Label { value, .. }) = metadata.iter_mut().find(|f| f.key() == "type") {
        *value = type_name;
    }
    metadata
}
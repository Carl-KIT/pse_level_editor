use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui::{self, Color32};

// Theme colors - designed to work well together for both egui and macroquad
pub struct Theme {
    // Background colors
    pub background: Color32,
    pub panel_background: Color32,
    pub tile_background: Color32,
    
    // Text colors
    pub text_primary: Color32,
    pub text_secondary: Color32,
    
    // UI element colors
    pub button_background: Color32,
    pub button_hover: Color32,
    pub button_active: Color32,
    pub separator: Color32,
    
    // Tile colors
    pub tile_air: Color32,
    pub tile_grass: Color32,
    pub tile_ground: Color32,
    
    // Selection and highlight colors
    pub selection: Color32,
    pub highlight: Color32,
    pub grid_line: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Background colors - using a soft, neutral palette
            background: egui::Color32::from_rgb(242, 242, 242), // Very light gray
            panel_background: egui::Color32::from_rgb(250, 250, 250), // Almost white
            tile_background: egui::Color32::from_rgb(235, 235, 235), // Light gray
            
            // Text colors - dark but not pure black
            text_primary: egui::Color32::from_rgb(51, 51, 51), // Dark gray
            text_secondary: egui::Color32::from_rgb(102, 102, 102), // Medium gray
            
            // UI element colors
            button_background: egui::Color32::from_rgb(217, 217, 217), // Light gray
            button_hover: egui::Color32::from_rgb(191, 191, 191), // Medium gray
            button_active: egui::Color32::from_rgb(166, 166, 166), // Darker gray
            separator: egui::Color32::from_rgb(204, 204, 204), // Light gray
            
            // Tile colors - more muted and professional
            tile_air: egui::Color32::from_rgb(250, 250, 250), // Almost white
            tile_grass: egui::Color32::from_rgb(153, 204, 153), // Muted green
            tile_ground: egui::Color32::from_rgb(178, 153, 127), // Muted brown
            
            // Selection and highlight colors
            selection: egui::Color32::from_rgb(51, 153, 229), // Semi-transparent blue
            highlight: egui::Color32::from_rgb(255, 229, 102), // Semi-transparent yellow
            grid_line: egui::Color32::from_rgb(204, 204, 204), // Light gray
        }
    }
}

impl Theme {
    pub fn apply_egui_theme(&self, ctx: &egui::Context) {
        let mut style = egui::Style::default();

        // Configure the visual style with a light theme approach
        style.visuals.dark_mode = false;
        
        // Override specific colors to match our theme
        style.visuals.widgets.noninteractive.bg_fill = self.panel_background;
        style.visuals.widgets.inactive.bg_fill = egui::Color32::RED;
        style.visuals.widgets.hovered.bg_fill = self.button_hover;
        style.visuals.widgets.active.bg_fill = self.button_active;
        
        // Panel and window colors
        style.visuals.panel_fill = self.panel_background;
        style.visuals.window_fill = self.panel_background;
        
        // Separator color
        style.visuals.widgets.noninteractive.bg_stroke.color = self.separator;
        
        // Text colors - this should apply to all text elements including buttons
        style.visuals.override_text_color = Some(self.text_primary);
        
        // Text input field colors - ensure proper contrast
        style.visuals.widgets.inactive.bg_stroke.color = self.separator;
        style.visuals.widgets.hovered.bg_stroke.color = self.separator;
        style.visuals.widgets.active.bg_stroke.color = self.separator;

        style.visuals.widgets.inactive.bg_stroke.width = 1.0;
        style.visuals.widgets.hovered.bg_stroke.width = 1.0;
        style.visuals.widgets.active.bg_stroke.width = 1.0; 
        
        ctx.set_style(style);
    }
    
    pub fn tile_color(&self, tile_type: crate::tile::TileType) -> Color32 {
        match tile_type {
            crate::tile::TileType::Air => self.tile_air,
            crate::tile::TileType::Grass => self.tile_grass,
            crate::tile::TileType::Ground => self.tile_ground,
        }
    }

    pub fn to_macroquad_color(&self, color: Color32) -> Color {
        Color::new(
            color.r() as f32 / 255.0,
            color.g() as f32 / 255.0,
            color.b() as f32 / 255.0,
            color.a() as f32 / 255.0,
        )
    }
}

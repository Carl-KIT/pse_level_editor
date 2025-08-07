use egui_macroquad::macroquad::prelude::*;

// Constants
const CAMERA_VIEWPORT_SIZE: f32 = 2.0; // Camera viewport size in world units

// Simple window resize handler
pub struct WindowResizeHandler {
    last_width: f32,
    last_height: f32,
}

impl WindowResizeHandler {
    pub fn new() -> Self {
        Self {
            last_width: screen_width(),
            last_height: screen_height(),
        }
    }

    pub fn check_for_resize(&mut self) -> Option<(f32, f32)> {
        let current_width = screen_width();
        let current_height = screen_height();

        if current_width != self.last_width || current_height != self.last_height {
            self.last_width = current_width;
            self.last_height = current_height;
            Some((current_width, current_height))
        } else {
            None
        }
    }
}

pub struct Camera {
    camera: Camera2D,
    base_zoom: f32, // Track the base zoom separately from corrected zoom
    last_mouse_pos: Vec2,
    resize_handler: WindowResizeHandler,
}

impl Camera {
    pub fn new(level_width: f32, level_height: f32) -> Self {
        let max_dimension = level_width.max(level_height);
        let zoom = CAMERA_VIEWPORT_SIZE / max_dimension;
        
        let camera = Camera2D {
            target: vec2(level_width / 2.0, level_height / 2.0),
            zoom: vec2(zoom, zoom), // Uniform zoom for square tiles
            ..Default::default()
        };
        
        Self {
            camera,
            base_zoom: zoom,
            last_mouse_pos: vec2(0.0, 0.0),
            resize_handler: WindowResizeHandler::new(),
        }
    }

    fn apply_aspect_ratio_correction(&mut self) {
        // Calculate aspect ratio correction to maintain square pixels
        let screen_width = screen_width();
        let screen_height = screen_height();
        let aspect_ratio = screen_width / screen_height;
        
        // Apply aspect ratio correction to maintain square pixels
        let corrected_zoom = if aspect_ratio > 1.0 {
            // Screen is wider - adjust X zoom
            vec2(self.base_zoom / aspect_ratio, self.base_zoom)
        } else {
            // Screen is taller - adjust Y zoom
            vec2(self.base_zoom, self.base_zoom * aspect_ratio)
        };
        
        // Update the camera's zoom directly
        self.camera.zoom = corrected_zoom;
    }

    pub fn setup_camera(&mut self) {
        // Check for window resize and apply correction if needed
        if self.resize_handler.check_for_resize().is_some() {
            self.apply_aspect_ratio_correction();
        }
        
        // Set the camera for rendering
        set_camera(&self.camera);
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        // The camera is already corrected, so we can use it directly
        self.camera.screen_to_world(screen_pos)
    }

    pub fn zoom(&self) -> Vec2 {
        // Return the base zoom (uniform) for external use
        vec2(self.base_zoom, self.base_zoom)
    }

    pub fn set_zoom(&mut self, zoom: Vec2) {
        // Store the base zoom and apply correction immediately
        self.base_zoom = zoom.x; // Both X and Y should be the same
        self.apply_aspect_ratio_correction();
    }

    pub fn target(&self) -> Vec2 {
        self.camera.target
    }

    pub fn set_target(&mut self, target: Vec2) {
        self.camera.target = target;
    }

    pub fn last_mouse_pos(&self) -> Vec2 {
        self.last_mouse_pos
    }

    pub fn set_last_mouse_pos(&mut self, pos: Vec2) {
        self.last_mouse_pos = pos;
    }
} 
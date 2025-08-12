use crate::editor::BrushType;
use crate::editor::LevelEditor;
use crate::tile::{SelectableMeta, TileType};
use crate::ui::module_view::show_module_view;
use egui_macroquad::egui;

mod inspector;
mod menu_bar;
mod module_view;
mod tile_selector;

pub struct UI;

impl UI {
    pub fn draw_all(editor: &mut LevelEditor, egui_ctx: &egui::Context) {
        // Draw menu bar
        menu_bar::show_menu_bar(egui_ctx, editor);

        if !editor.show_modules_view()
            && editor.show_tile_selector()
            && !matches!(editor.brush_type(), BrushType::Structure)
        {
            // Draw left panel: either tile selector or modules view (mutually exclusive)
            tile_selector::show_tiles(egui_ctx, editor);
        } else if matches!(editor.brush_type(), BrushType::Structure) {
            // Structure panel when Structure brush active
            tile_selector::show_structures(egui_ctx, editor);
        } 
        
        if editor.show_modules_view() {
            // Draw modules panel on left when enabled
            show_module_view(egui_ctx, editor);
        } else if let Some(tile) = editor.get_selected_tile() {
            inspector::show_inspector(egui_ctx, editor);
        }
    }
}

use egui_macroquad::egui::{self, Context};
use crate::{editor::{BrushType, LevelEditor}, tile::TileType};

pub fn show_structures(egui_ctx: &Context, editor: &mut LevelEditor) {
    egui::SidePanel::left("structure_panel")
    .resizable(true)
    .default_width(200.0)
    .show(egui_ctx, |ui| {
        ui.heading("Structures");
        ui.separator();
        let modes = [
            ("Platform", crate::editor::StructureMode::PlatformRect),
            ("Stairs", crate::editor::StructureMode::Stairs),
        ];
        for (label, mode) in modes {
            let is_selected = editor.structure_mode() == mode;
            if ui.selectable_label(is_selected, label).clicked() {
                editor.set_structure_mode(mode);
            }
        }

        ui.separator();
        ui.heading("Brush Type");
        ui.separator();
        for brush_type in [BrushType::Single, BrushType::Fill, BrushType::Selector, BrushType::Structure] {
            let is_selected = editor.brush_type() == brush_type;
            if ui.selectable_label(is_selected, brush_type.name()).clicked() {
                editor.set_brush_type(brush_type);
            }
        }
    });
}
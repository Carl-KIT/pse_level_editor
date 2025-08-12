use egui_macroquad::egui::{self, Context};
use crate::{editor::{BrushType, LevelEditor}, tile::{Platform, SelectableMeta, Tile, TileType}};


pub fn show_module_view(egui_ctx: &Context, editor: &mut LevelEditor) {
    egui::SidePanel::left("modules_panel")
                .resizable(true)
                .default_width(220.0)
                .show(egui_ctx, |ui| {
                    ui.heading("Modules");
                    ui.separator();
                    ui.label(format!("Level width: {}", editor.level_width()));
                    ui.separator();
                    // Modules spans list
                    let mut to_remove: Option<usize> = None;
                    let modules_snapshot = editor.modules().clone();
                    for (i, span) in modules_snapshot.iter().copied().enumerate() {
                        let mut span_mut = span as i32;
                        ui.horizontal(|ui| {
                            ui.label(format!("Module {} span:", i));
                            ui.add(egui::DragValue::new(&mut span_mut).range(1..=100000));
                            if ui.button("Remove").clicked() {
                                to_remove = Some(i);
                            }
                        });
                        if span_mut as usize != span {
                            editor.set_module_span(i, span_mut.max(1) as usize);
                        }
                    }
                    if let Some(idx) = to_remove {
                        editor.remove_module(idx);
                    }
                    ui.separator();
                    if ui.button("Add Module").clicked() {
                        editor.add_module(10);
                    }
                });
}
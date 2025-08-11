use core::num;

use super::Brush;
use crate::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StructureMode {
    PlatformRect,
    Stairs,
}

impl StructureMode {
    fn shape(&self, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
        match self {
            Self::PlatformRect => {
                // determine top left corner
                let start_x = start.0.min(end.0);
                let start_y = start.1.min(end.1);

                // determine bottom right corner
                let end_x = start.0.max(end.0);
                let end_y = start.1.max(end.1);

                let mut res = vec![];

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        res.push((x, y));
                    }
                }

                res
            }
            Self::Stairs => {
                let mut res = Vec::new();

                let (sx, sy) = start;
                let (ex, ey) = end;

                let dx = sx.abs_diff(ex);
                let dy = sy.abs_diff(ey);

                let steps = dx.min(dy);

                for i in 0..=steps {
                    let (start_pos, end_y) = if sy < ey {
                        if sx < ex {
                            (
                                (sx + i, sy + i),
                                ey
                            )
                        } else {
                            (
                                (sx - i, sy + i),
                                ey
                            )
                        }
                    } else {
                        if sx < ex {
                            (
                                (sx + i, sy - i),
                                sy
                            )
                        } else {
                            (
                                (sx - i, sy - i),
                                sy
                            )
                        }
                    };

                    for y in start_pos.1..end_y {
                        res.push((start_pos.0, y));
                    }

                    // res.push(pos);
                }
                

                res
            }
        }
    }
}

pub struct StructureBrush {
    mode: StructureMode,
    active: bool,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
}

impl StructureBrush {
    pub fn new() -> Self {
        Self {
            mode: StructureMode::PlatformRect,
            active: false,
            start: None,
            end: None,
        }
    }
    pub fn set_mode(&mut self, mode: StructureMode) {
        self.mode = mode;
    }
    pub fn mode(&self) -> StructureMode {
        self.mode
    }
}

impl Brush for StructureBrush {
    fn name(&self) -> &'static str {
        "Structure"
    }

    fn on_mouse_press(&mut self, _level: &mut Level, x: usize, y: usize, _tile: TileType) -> bool {
        self.active = true;
        self.start = Some((x, y));
        true
    }

    fn on_mouse_drag(&mut self, level: &mut Level, x: usize, y: usize, _tile: TileType) -> bool {
        if !self.active {
            return true;
        }

        self.end = Some((x, y));

        level.clear_highlights();
        
        if let Some((sx, sy)) = self.start {
            let shape = self.mode.shape((sx, sy), (x, y));
            level.set_highlighted_tiles(shape);
        }

        true
    }

    fn on_mouse_release(&mut self, level: &mut Level) -> bool {
        if !self.active {
            return true;
        }
        if let (Some((sx, sy)), Some((ex, ey))) = (self.start.take(), self.end.take()) {
            let wall = TileType::Custom("wall".to_string());

            let shape = self.mode.shape((sx, sy), (ex, ey));

            for (x, y) in shape {
                level.set_tile(x, y, wall.clone());
            }

            level.finish_operation();
            level.clear_highlights();
        }
        self.active = false;
        true
    }

    fn on_mouse_cancel(&mut self, level: &mut Level) -> bool {
        self.active = false;
        self.start = None;
        level.clear_highlights();
        true
    }
    fn draw_preview(&self, _level: &Level) {}
    fn is_active(&self) -> bool {
        self.active
    }
    fn update_highlights(&mut self, _level: &mut Level, _mx: Option<usize>, _my: Option<usize>) {}
}

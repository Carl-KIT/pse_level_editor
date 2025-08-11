use crate::tile::TileType;
use crate::level::{Level, Platform};

impl Level {
    pub(crate) fn rebuild_platforms(&mut self) {
        self.platforms.clear();
        for row in &mut self.platform_map { for cell in row.iter_mut() { *cell = None; } }

        let mut visited = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                if visited[y][x] { continue; }
                let tile_type = self.tiles[y][x].tile_type;
                if tile_type != TileType::Grass && tile_type != TileType::Ground { continue; }

                // First row width from (x,y)
                let mut max_width = 0usize;
                let mut xi = x;
                while xi < self.width {
                    if !visited[y][xi] && self.tiles[y][xi].tile_type == tile_type {
                        max_width += 1;
                        xi += 1;
                    } else { break; }
                }
                if max_width == 0 { continue; }

                let mut rect_width = max_width;
                let mut rect_height = 1usize;
                let mut yy = y + 1;
                while yy < self.height {
                    let mut run = 0usize;
                    let mut xx = x;
                    while xx < x + rect_width && xx < self.width {
                        if !visited[yy][xx] && self.tiles[yy][xx].tile_type == tile_type { run += 1; xx += 1; } else { break; }
                    }
                    if run == 0 { break; }
                    rect_width = rect_width.min(run);
                    rect_height += 1;
                    yy += 1;
                }

                let min_x = x;
                let min_y = y;
                let max_x = x + rect_width - 1;
                let max_y = y + rect_height - 1;

                let platform_index = self.platforms.len();
                for ty in min_y..=max_y { for tx in min_x..=max_x { visited[ty][tx] = true; self.platform_map[ty][tx] = Some(platform_index); } }
                self.platforms.push(Platform { tile_type, min_x, min_y, max_x, max_y });
            }
        }
    }

    pub(crate) fn try_update_platforms_locally(&mut self, x: usize, y: usize) {
        let t = self.tiles[y][x].tile_type;
        if !Self::is_platform_type(t) { return; }

        // Adjacent platform indices of same type
        let mut neighbor_indices = std::collections::BTreeSet::<usize>::new();
        let mut add_idx = |ox: isize, oy: isize| {
            let nx = x as isize + ox; let ny = y as isize + oy;
            if nx >= 0 && ny >= 0 {
                let nxu = nx as usize; let nyu = ny as usize;
                if nxu < self.width && nyu < self.height {
                    if let Some(idx) = self.platform_map[nyu][nxu] { if self.platforms[idx].tile_type == t { neighbor_indices.insert(idx); } }
                }
            }
        };
        add_idx(-1, 0); add_idx(1, 0); add_idx(0, -1); add_idx(0, 1);
        if let Some(idx) = self.platform_map[y][x] { if self.platforms[idx].tile_type == t { neighbor_indices.insert(idx); } }

        let mut merged_rect: Option<(usize, usize, usize, usize)> = None;
        if !neighbor_indices.is_empty() {
            let mut min_x = x; let mut min_y = y; let mut max_x = x; let mut max_y = y;
            for &idx in &neighbor_indices { let p = &self.platforms[idx]; min_x = min_x.min(p.min_x); min_y = min_y.min(p.min_y); max_x = max_x.max(p.max_x); max_y = max_y.max(p.max_y); }
            if self.rect_is_uniform_type(min_x, min_y, max_x, max_y, t) { merged_rect = Some((min_x, min_y, max_x, max_y)); }
        }

        let target_rect = merged_rect.unwrap_or_else(|| self.maximal_rect_including(x, y, t));
        self.remove_platforms_overlapping_rect_of_type(target_rect, t);
        self.assign_platform_rect(t, target_rect);
    }

    pub(crate) fn is_platform_type(t: TileType) -> bool { matches!(t, TileType::Grass | TileType::Ground) }

    pub(crate) fn rect_is_uniform_type(&self, min_x: usize, min_y: usize, max_x: usize, max_y: usize, t: TileType) -> bool {
        for yy in min_y..=max_y { for xx in min_x..=max_x { if self.tiles[yy][xx].tile_type != t { return false; } } }
        true
    }

    pub(crate) fn maximal_rect_including(&self, x: usize, y: usize, t: TileType) -> (usize, usize, usize, usize) {
        let mut left = x; while left > 0 && self.tiles[y][left - 1].tile_type == t { left -= 1; }
        let mut right = x; while right + 1 < self.width && self.tiles[y][right + 1].tile_type == t { right += 1; }
        let mut top = y; while top > 0 {
            let nt = top - 1; let mut ok = true; for xx in left..=right { if self.tiles[nt][xx].tile_type != t { ok = false; break; } }
            if ok { top = nt; } else { break; }
        }
        let mut bottom = y; while bottom + 1 < self.height {
            let nb = bottom + 1; let mut ok = true; for xx in left..=right { if self.tiles[nb][xx].tile_type != t { ok = false; break; } }
            if ok { bottom = nb; } else { break; }
        }
        (left, top, right, bottom)
    }

    pub(crate) fn remove_platforms_overlapping_rect_of_type(&mut self, rect: (usize, usize, usize, usize), t: TileType) {
        use std::collections::BTreeSet;
        let (min_x, min_y, max_x, max_y) = rect;
        let mut to_remove: BTreeSet<usize> = BTreeSet::new();
        for yy in min_y..=max_y { for xx in min_x..=max_x { if let Some(idx) = self.platform_map[yy][xx] { if self.platforms[idx].tile_type == t { to_remove.insert(idx); } } } }
        if to_remove.is_empty() { return; }

        let mut new_platforms: Vec<Platform> = Vec::with_capacity(self.platforms.len() - to_remove.len());
        let mut old_to_new: Vec<Option<usize>> = vec![None; self.platforms.len()];
        for (old_idx, plat) in self.platforms.iter().enumerate() {
            if to_remove.contains(&old_idx) { continue; }
            let new_idx = new_platforms.len(); new_platforms.push(plat.clone()); old_to_new[old_idx] = Some(new_idx);
        }

        for yy in 0..self.height { for xx in 0..self.width { if let Some(old_idx) = self.platform_map[yy][xx] {
            if to_remove.contains(&old_idx) { self.platform_map[yy][xx] = None; } else if let Some(new_idx) = old_to_new[old_idx] { self.platform_map[yy][xx] = Some(new_idx); }
        } } }
        self.platforms = new_platforms;
    }

    pub(crate) fn assign_platform_rect(&mut self, t: TileType, rect: (usize, usize, usize, usize)) {
        let (min_x, min_y, max_x, max_y) = rect; let new_index = self.platforms.len();
        self.platforms.push(Platform { tile_type: t, min_x, min_y, max_x, max_y });
        for yy in min_y..=max_y { for xx in min_x..=max_x { self.platform_map[yy][xx] = Some(new_index); } }
    }
}


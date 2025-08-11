use crate::tile::TileType;

// Represents a single tile change operation
#[derive(Clone, Debug)]
pub struct TileChange {
    pub x: usize,
    pub y: usize,
    pub old_tile: TileType,
    pub new_tile: TileType,
}

// Represents a group of tile changes that should be undone/redone together
#[derive(Clone, Debug)]
pub struct TileOperation {
    pub changes: Vec<TileChange>,
    pub description: String,
}

impl TileOperation {
    pub fn new(description: String) -> Self {
        Self {
            changes: Vec::new(),
            description,
        }
    }

    pub fn add_change(&mut self, x: usize, y: usize, old_tile: TileType, new_tile: TileType) {
        self.changes.push(TileChange {
            x,
            y,
            old_tile,
            new_tile,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

// Main history manager for undo/redo functionality
pub struct HistoryManager {
    undo_stack: Vec<TileOperation>,
    redo_stack: Vec<TileOperation>,
    max_history_size: usize,
}

impl HistoryManager {
    pub fn new(max_history_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history_size,
        }
    }

    pub fn add_operation(&mut self, operation: TileOperation) {
        // Clear redo stack when new operation is added
        self.redo_stack.clear();
        
        // Only add non-empty operations
        if !operation.is_empty() {
            self.undo_stack.push(operation);
            
            // Limit history size
            if self.undo_stack.len() > self.max_history_size {
                self.undo_stack.remove(0);
            }
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn undo(&mut self) -> Option<TileOperation> {
        self.undo_stack.pop().map(|operation| {
            // Move to redo stack
            self.redo_stack.push(operation.clone());
            operation
        })
    }

    pub fn redo(&mut self) -> Option<TileOperation> {
        self.redo_stack.pop().map(|operation| {
            // Move back to undo stack
            self.undo_stack.push(operation.clone());
            operation
        })
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
} 
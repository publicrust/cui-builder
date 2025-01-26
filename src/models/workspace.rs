#[derive(Clone, PartialEq)]
pub struct WorkspaceState {
    pub offset_x: f64,
    pub offset_y: f64,
    pub scale: f64,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
        }
    }
} 
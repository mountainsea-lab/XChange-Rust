use crate::dto::account::open_position::OpenPosition;

#[derive(Debug, Clone)]
pub struct OpenPositions {
    pub open_positions: Vec<OpenPosition>,
}

impl OpenPositions {
    pub fn new() -> Self {
        Self {
            open_positions: Vec::new(),
        }
    }

    pub fn with_positions(mut self, positions: Vec<OpenPosition>) -> Self {
        self.open_positions = positions;
        self
    }

    pub fn add_position(mut self, position: OpenPosition) -> Self {
        self.open_positions.push(position);
        self
    }
}

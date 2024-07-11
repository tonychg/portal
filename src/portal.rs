pub use crate::Position;

pub struct Portal {
    pub position: Position,
}

impl Portal {
    pub fn new(position: Position) -> Portal {
        Portal { position }
    }
}

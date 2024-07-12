mod map;
mod position;
mod redirection;

pub use crate::map::{Cell, Map};
pub use crate::position::Position;
pub use crate::redirection::Redirection;

pub struct Portal {
    pub position: Position,
}

impl Portal {
    pub fn new(position: Position) -> Portal {
        Portal { position }
    }
}

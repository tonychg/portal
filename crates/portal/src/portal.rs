mod map;
mod position;
mod redirection;

pub use crate::map::{Cell, Map};
pub use crate::position::Position;
pub use crate::redirection::Redirection;

#[derive(Debug, Clone)]
pub struct Portal {
    pub position: Position,
}

impl Portal {
    pub fn new(position: Position) -> Portal {
        Portal { position }
    }

    // Debug function to print portals in a grid with their index.
    pub fn debug(width: i16, height: i16, portals: &[Portal]) {
        for y in 0..=height {
            for x in 0..=width {
                match portals
                    .iter()
                    .position(|r| r.position == Position::new(x, y))
                {
                    Some(index) => print!("{:2} ", index),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }
}

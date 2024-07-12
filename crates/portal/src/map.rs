use crate::{Portal, Position};
use std::{collections::HashMap, fmt::Display};

const MAX_PORTAL: usize = 4;

#[derive(Debug, thiserror::Error)]
pub enum MapError {
    #[error("Portal cannot be place on {0}")]
    InvalidCell(String),
    #[error("Cell is out of bound")]
    OutOfBound,
}

pub struct Map {
    portals: Vec<Portal>,
    cells: HashMap<Position, Cell>,
}

impl Map {
    pub fn place_portal(&mut self, position: Position) -> Result<(), MapError> {
        match self.cells.get(&position) {
            Some(cell) => match cell {
                Cell::Empty => {
                    if self.portals.len() == MAX_PORTAL {
                        self.portals.pop();
                    }
                    self.portals.insert(0, Portal::new(position));
                    Ok(())
                }
                Cell::Hole | Cell::Block => Err(MapError::InvalidCell(cell.to_string())),
            },
            None => Err(MapError::OutOfBound),
        }
    }
}

pub enum Cell {
    Empty,
    Block,
    Hole,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "Empty"),
            Cell::Block => write!(f, "Block"),
            Cell::Hole => write!(f, "Hole"),
        }
    }
}
